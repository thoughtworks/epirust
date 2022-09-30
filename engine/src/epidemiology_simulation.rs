/*
 * EpiRust
 * Copyright (c) 2020  ThoughtWorks, Inc.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

use core::borrow::BorrowMut;
use std::time::Instant;

use common::config::{Config, Population, TravelPlanConfig};
use common::disease::Disease;
use common::models::CommutePlan;
use common::utils::RandomWrapper;
use futures::join;

use crate::allocation_map::CitizenLocationMap;
use crate::geography;
use crate::geography::Point;
use crate::interventions::hospital::BuildNewHospital;
use crate::interventions::lockdown::LockdownIntervention;
use crate::interventions::vaccination::VaccinateIntervention;
use crate::interventions::Interventions;
use crate::kafka::kafka_producer::{KafkaProducer, COMMUTE_TOPIC, MIGRATION_TOPIC};
use crate::kafka::{ticks_consumer, travel_consumer};
use crate::listeners::csv_service::CsvListener;
use crate::listeners::disease_tracker::Hotspot;
use crate::listeners::events_kafka_producer::EventsKafkaProducer;
use crate::listeners::intervention_reporter::InterventionReporter;
use crate::listeners::listener::{Listener, Listeners};
use crate::listeners::travel_counter::TravelCounter;
use crate::models::constants;
use crate::models::events::Counts;
use crate::models::events::Tick;
use crate::tick::{receive_tick, send_ack};
use crate::travel::commute;
use crate::travel::commute::Commuter;
use crate::travel::commute::CommutersByRegion;
use crate::travel::migration::{EngineMigrationPlan, Migrator, MigratorsByRegion};
use crate::utils::util::{counts_at_start, output_file_format};
use crate::RunMode;

pub struct Epidemiology {
    pub agent_location_map: CitizenLocationMap,
    pub disease: Disease,
    pub sim_id: String,
    pub travel_plan_config: Option<TravelPlanConfig>,
}

impl Epidemiology {
    pub fn new(config: &Config, travel_plan_config: Option<TravelPlanConfig>, sim_id: String) -> Epidemiology {
        let start = Instant::now();
        let disease = config.get_disease();
        let start_infections = config.get_starting_infections();
        let mut grid = geography::define_geography(config.get_grid_size(), sim_id.clone());
        let mut rng = RandomWrapper::new();
        let (start_locations, agent_list) = match config.get_population() {
            Population::Csv(csv_pop) => grid.read_population(csv_pop, start_infections, &mut rng, &sim_id),
            Population::Auto(auto_pop) => {
                grid.generate_population(auto_pop, start_infections, &mut rng, &travel_plan_config, sim_id.clone())
            }
        };
        grid.resize_hospital(
            agent_list.len() as i32,
            constants::HOSPITAL_STAFF_PERCENTAGE,
            config.get_geography_parameters().hospital_beds_percentage,
            sim_id.clone(),
        );

        let agent_location_map = CitizenLocationMap::new(grid, &agent_list, &start_locations);

        info!("Initialization completed in {} seconds", start.elapsed().as_secs_f32());
        Epidemiology { travel_plan_config, agent_location_map, disease, sim_id }
    }

    fn create_listeners(&self, config: &Config, run_mode: &RunMode) -> Listeners {
        let output_file_format = output_file_format(config, run_mode);
        let counts_file_name = format!("{}.csv", output_file_format);

        let csv_listener = CsvListener::new(counts_file_name);
        let population = self.agent_location_map.current_population();

        let hotspot_tracker = Hotspot::new();
        let intervention_reporter = InterventionReporter::new(format!("{}_interventions.json", output_file_format));
        let mut listeners_vec: Vec<Box<dyn Listener>> =
            vec![Box::new(csv_listener), Box::new(hotspot_tracker), Box::new(intervention_reporter)];

        match run_mode {
            RunMode::Standalone => {}
            RunMode::SingleDaemon => {
                let kafka_listener =
                    EventsKafkaProducer::new(self.sim_id.clone(), population as usize, config.enable_citizen_state_messages());
                listeners_vec.push(Box::new(kafka_listener));
            }
            RunMode::MultiEngine { .. } => {
                let travels_file_name = format!("{}_outgoing_travels.csv", output_file_format);
                let travel_counter = TravelCounter::new(travels_file_name);
                listeners_vec.push(Box::new(travel_counter));

                let kafka_listener =
                    EventsKafkaProducer::new(self.sim_id.clone(), population as usize, config.enable_citizen_state_messages());
                listeners_vec.push(Box::new(kafka_listener));
            }
        }

        Listeners::from(listeners_vec)
    }

    fn init_interventions(&mut self, config: &Config, rng: &mut RandomWrapper) -> Interventions {
        let vaccinations = VaccinateIntervention::init(config);
        let lock_down_details = LockdownIntervention::init(config);
        let hospital_intervention = BuildNewHospital::init(config);
        let essential_workers_population = lock_down_details.get_essential_workers_percentage();

        for (_, agent) in self.agent_location_map.iter_mut() {
            agent.assign_essential_worker(essential_workers_population, rng);
        }
        Interventions { vaccinate: vaccinations, lockdown: lock_down_details, build_new_hospital: hospital_intervention }
    }

    pub async fn run(&mut self, config: &Config, run_mode: &RunMode) {
        let mut listeners = self.create_listeners(config, run_mode);
        let population = self.agent_location_map.current_population();
        let mut counts_at_hr = counts_at_start(population, config.get_starting_infections());
        let mut rng = RandomWrapper::new();

        let mut interventions = self.init_interventions(config, &mut rng);

        listeners.grid_updated(&self.agent_location_map.grid);
        match run_mode {
            RunMode::MultiEngine { engine_id } => {
                self.run_multi_engine(config, engine_id, &mut listeners, &mut counts_at_hr, &mut interventions, &mut rng).await
            }
            _ => {
                self.run_single_engine(
                    config,
                    run_mode,
                    &mut listeners,
                    &mut counts_at_hr,
                    &mut interventions,
                    &mut rng,
                    self.sim_id.to_string(),
                )
                .await
            }
        }
    }

    pub async fn run_single_engine(
        &mut self,
        config: &Config,
        run_mode: &RunMode,
        listeners: &mut Listeners,
        counts_at_hr: &mut Counts,
        interventions: &mut Interventions,
        rng: &mut RandomWrapper,
        sim_id: String,
    ) {
        let start_time = Instant::now();
        let mut outgoing_migrators = Vec::new();
        let mut outgoing_commuters = Vec::new();
        let percent_outgoing = 0.0;

        counts_at_hr.log();
        for simulation_hour in 1..config.get_hours() {
            counts_at_hr.increment_hour();

            let population_before_travel = self.agent_location_map.current_population();

            if population_before_travel == 0 {
                panic!("No citizens!");
            }

            self.agent_location_map.simulate(
                counts_at_hr,
                simulation_hour,
                listeners,
                rng,
                &self.disease,
                percent_outgoing,
                &mut outgoing_migrators,
                &mut outgoing_commuters,
                config.enable_citizen_state_messages(),
                None,
                &sim_id,
            );

            listeners.counts_updated(*counts_at_hr);
            self.agent_location_map.process_interventions(interventions, counts_at_hr, listeners, rng, config, &sim_id);

            if Epidemiology::stop_simulation(&mut interventions.lockdown, run_mode, *counts_at_hr) {
                break;
            }

            if simulation_hour % 100 == 0 {
                info!(
                    "Throughput: {} iterations/sec; simulation hour {} of {}",
                    simulation_hour as f32 / start_time.elapsed().as_secs_f32(),
                    simulation_hour,
                    config.get_hours()
                );
                counts_at_hr.log();
            }
        }
        let elapsed_time = start_time.elapsed().as_secs_f32();
        info!("Number of iterations: {}, Total Time taken {} seconds", counts_at_hr.get_hour(), elapsed_time);
        info!("Iterations/sec: {}", counts_at_hr.get_hour() as f32 / elapsed_time);
        listeners.simulation_ended();
    }

    pub async fn run_multi_engine(
        &mut self,
        config: &Config,
        engine_id: &String,
        listeners: &mut Listeners,
        counts_at_hr: &mut Counts,
        interventions: &mut Interventions,
        rng: &mut RandomWrapper,
    ) {
        let start_time = Instant::now();
        let mut producer = KafkaProducer::new();

        let travel_plan_config = self.travel_plan_config.as_ref().unwrap();

        debug!("{}: Start Multi Engine Simulation", engine_id);
        let is_commute_enabled = travel_plan_config.commute.enabled;
        let is_migration_enabled = travel_plan_config.migration.enabled;

        let migration_plan = if is_migration_enabled { Some(travel_plan_config.migration_plan()) } else { None };

        let mut engine_migration_plan =
            EngineMigrationPlan::new(engine_id.clone(), migration_plan, self.agent_location_map.current_population());

        debug!("{}: Start Migrator Consumer", engine_id);
        let migrators_consumer = travel_consumer::start(engine_id, &[&*format!("{}{}", MIGRATION_TOPIC, engine_id)], "migrate");
        let mut migration_stream = migrators_consumer.stream();

        let commute_plan = if is_commute_enabled {
            travel_plan_config.commute_plan()
        } else {
            CommutePlan { regions: Vec::new(), matrix: Vec::new() }
        };

        debug!("{}: Start Commuter Consumer", engine_id);
        let commute_consumer = travel_consumer::start(engine_id, &[&*format!("{}{}", COMMUTE_TOPIC, engine_id)], "commute");
        let mut commute_stream = commute_consumer.stream();

        let ticks_consumer = ticks_consumer::start(engine_id);
        let mut ticks_stream = ticks_consumer.stream();

        let mut n_incoming = 0;
        let mut n_outgoing = 0;

        counts_at_hr.log();

        let mut total_tick_sync_time = 0;
        let mut total_commute_sync_time = 0;
        let run_mode = RunMode::MultiEngine { engine_id: engine_id.to_string() };

        for simulation_hour in 1..config.get_hours() {
            let start_time = Instant::now();
            let tick =
                receive_tick(&run_mode, &mut ticks_stream, simulation_hour, is_commute_enabled, is_migration_enabled).await;
            if let Some(t) = tick {
                total_tick_sync_time += start_time.elapsed().as_millis();
                info!("total tick sync time as hour {} - is {}", simulation_hour, total_tick_sync_time);
                if t.terminate() {
                    info!("received tick {:?}", t);
                    break;
                }
            }

            counts_at_hr.increment_hour();

            let population_before_travel = self.agent_location_map.current_population();

            if population_before_travel == 0 {
                panic!("No citizens!");
            }
            if is_migration_enabled {
                engine_migration_plan.set_current_population(population_before_travel);
            }

            let disease = &self.disease;

            let mut percent_outgoing = 0.0;
            let mut outgoing: Vec<(Point, Migrator)> = Vec::new();

            if simulation_hour % 24 == 0 && is_migration_enabled {
                percent_outgoing = engine_migration_plan.percent_outgoing();
            }
            let mut actual_outgoing: Vec<(Point, Migrator)> = Vec::new();

            let received_migrators = if is_migration_enabled {
                debug!("{}: Received Migrators | Simulation hour: {}", engine_id, simulation_hour);
                Some(engine_migration_plan.receive_migrators(tick, &mut migration_stream))
            } else {
                None
            };

            let mut outgoing_commuters: Vec<(Point, Commuter)> = Vec::new();
            let location_map = self.agent_location_map.borrow_mut();
            let sim = async {
                debug!("{}: Start simulation for hour: {}", engine_id, simulation_hour);
                location_map.simulate(
                    counts_at_hr,
                    simulation_hour,
                    listeners,
                    rng,
                    disease,
                    percent_outgoing,
                    &mut outgoing,
                    &mut outgoing_commuters,
                    config.enable_citizen_state_messages(),
                    Some(travel_plan_config),
                    engine_id,
                );
                debug!("{}: Simulation finished for hour: {}", engine_id, simulation_hour);

                let (outgoing_migrators_by_region, actual_total_outgoing) = if is_migration_enabled {
                    engine_migration_plan.alloc_outgoing_to_regions(&outgoing)
                } else {
                    (Vec::new(), Vec::new())
                };

                actual_outgoing = actual_total_outgoing;

                if simulation_hour % 24 == 0 && is_migration_enabled {
                    listeners.outgoing_migrators_added(simulation_hour, &outgoing_migrators_by_region);
                }

                let outgoing_commuters_by_region = if is_commute_enabled {
                    CommutersByRegion::get_commuters_by_region(&commute_plan.regions, &outgoing_commuters, simulation_hour)
                } else {
                    Vec::new()
                };

                if is_migration_enabled {
                    debug!("{}: Send Migrators", engine_id);
                    Epidemiology::send_migrators(tick, &mut producer, outgoing_migrators_by_region);
                }
                if is_commute_enabled {
                    debug!("{}: Send Commuters", engine_id);
                    Epidemiology::send_commuters(tick, &mut producer, outgoing_commuters_by_region);
                }
            };

            let _ = join!(sim);

            if is_commute_enabled {
                let commute_start_time = Instant::now();
                let received_commuters = commute::receive_commuters(&commute_plan, tick, &mut commute_stream, engine_id);
                let (mut incoming_commuters,) = join!(received_commuters);
                total_commute_sync_time += commute_start_time.elapsed().as_millis();
                info!("total commute sync time as hour {} - is {}", simulation_hour, total_commute_sync_time);
                n_incoming += incoming_commuters.len();
                n_outgoing += outgoing_commuters.len();
                self.agent_location_map.remove_commuters(&outgoing_commuters, counts_at_hr);
                self.agent_location_map.assimilate_commuters(&mut incoming_commuters, counts_at_hr, rng, simulation_hour);
                debug!("{}: assimilated the commuters", engine_id);
            }

            if is_migration_enabled {
                let (mut incoming,) = join!(received_migrators.unwrap());
                n_incoming += incoming.len();
                n_outgoing += outgoing.len();
                self.agent_location_map.remove_migrators(&actual_outgoing, counts_at_hr);
                self.agent_location_map.assimilate_migrators(&mut incoming, counts_at_hr, rng);
                debug!("{}: assimilated the migrators", engine_id);
            }

            listeners.counts_updated(*counts_at_hr);
            self.agent_location_map.process_interventions(interventions, counts_at_hr, listeners, rng, config, engine_id);

            if Epidemiology::stop_simulation(&mut interventions.lockdown, &run_mode, *counts_at_hr) {
                break;
            }

            send_ack(
                &run_mode,
                &mut producer,
                *counts_at_hr,
                simulation_hour,
                &interventions.lockdown,
                is_commute_enabled,
                is_migration_enabled,
            );

            if simulation_hour % 100 == 0 {
                info!(
                    "Throughput: {} iterations/sec; simulation hour {} of {}",
                    simulation_hour as f32 / start_time.elapsed().as_secs_f32(),
                    simulation_hour,
                    config.get_hours()
                );
                counts_at_hr.log();
                info!(
                    "Incoming: {}, Outgoing: {}, Current Population: {}",
                    n_incoming,
                    n_outgoing,
                    self.agent_location_map.current_population()
                );
                n_incoming = 0;
                n_outgoing = 0;
            }
        }
        let elapsed_time = start_time.elapsed().as_secs_f32();
        info!("Number of iterations: {}, Total Time taken {} seconds", counts_at_hr.get_hour(), elapsed_time);
        info!("Iterations/sec: {}", counts_at_hr.get_hour() as f32 / elapsed_time);
        info!("total tick sync time: {}", total_tick_sync_time);
        info!("total commute sync time: {}", total_commute_sync_time);
        listeners.simulation_ended();
    }

    fn send_migrators(tick: Option<Tick>, producer: &mut KafkaProducer, outgoing: Vec<MigratorsByRegion>) {
        if tick.is_some() && tick.unwrap().hour() % 24 == 0 {
            producer.send_migrators(outgoing);
        }
    }

    fn send_commuters(tick_op: Option<Tick>, producer: &mut KafkaProducer, outgoing: Vec<CommutersByRegion>) {
        if let Some(tick) = tick_op {
            let hour = tick.hour() % 24;
            if hour == constants::ROUTINE_TRAVEL_START_TIME || hour == constants::ROUTINE_TRAVEL_END_TIME {
                producer.send_commuters(outgoing);
            }
        }
    }

    fn stop_simulation(lock_down_details: &mut LockdownIntervention, run_mode: &RunMode, row: Counts) -> bool {
        let zero_active_cases = row.get_exposed() == 0 && row.get_infected() == 0 && row.get_hospitalized() == 0;
        match run_mode {
            RunMode::MultiEngine { .. } => {
                if lock_down_details.is_locked_down() && zero_active_cases {
                    lock_down_details.set_zero_infection_hour(row.get_hour());
                }
                false
            }
            _ => zero_active_cases,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::geography::Area;
    use crate::geography::Point;
    use crate::STANDALONE_SIM_ID;
    use common::config::intervention_config::{InterventionConfig, VaccinateConfig};
    use common::config::{AutoPopulation, GeographyParameters};

    use super::*;

    #[test]
    fn should_init() {
        let pop = AutoPopulation { number_of_agents: 10, public_transport_percentage: 1.0, working_percentage: 1.0 };
        let disease = Disease::new(0, 0, 0, 0, 0, 0.0, 0.0, 0.0, 0.0, 0.0, 0, 0);
        let vac = VaccinateConfig { at_hour: 5000, percent: 0.2 };
        let geography_parameters = GeographyParameters::new(100, 0.003);
        let config = Config::new(
            Population::Auto(pop),
            disease,
            geography_parameters,
            vec![],
            100,
            vec![InterventionConfig::Vaccinate(vac)],
            None,
        );
        let epidemiology: Epidemiology = Epidemiology::new(&config, None, STANDALONE_SIM_ID.to_string());
        let expected_housing_area = Area::new(STANDALONE_SIM_ID.to_string(), Point::new(0, 0), Point::new(39, 100));
        assert_eq!(epidemiology.agent_location_map.grid.housing_area, expected_housing_area);

        let expected_transport_area = Area::new(STANDALONE_SIM_ID.to_string(), Point::new(40, 0), Point::new(59, 100));
        assert_eq!(epidemiology.agent_location_map.grid.transport_area, expected_transport_area);

        let expected_work_area = Area::new(STANDALONE_SIM_ID.to_string(), Point::new(60, 0), Point::new(79, 100));
        assert_eq!(epidemiology.agent_location_map.grid.work_area, expected_work_area);

        let expected_hospital_area = Area::new(STANDALONE_SIM_ID.to_string(), Point::new(80, 0), Point::new(89, 0));
        assert_eq!(epidemiology.agent_location_map.grid.hospital_area, expected_hospital_area);

        assert_eq!(epidemiology.agent_location_map.current_population(), 10);
    }
}
