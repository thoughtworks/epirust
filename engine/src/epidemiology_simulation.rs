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
use std::borrow::Borrow;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use futures::join;
use opentelemetry::trace::{FutureExt, Span, TraceContextExt, Tracer};
use opentelemetry::{global, Context, KeyValue};

use common::config::{Config, Population, TravelPlanConfig};
use common::models::CommutePlan;
use common::utils::RandomWrapper;

use crate::allocation_map::CitizenLocationMap;
use crate::geography;
use crate::geography::Point;
use crate::interventions::hospital::BuildNewHospital;
use crate::interventions::lockdown::LockdownIntervention;
use crate::interventions::vaccination::VaccinateIntervention;
use crate::interventions::Interventions;
use crate::listeners::csv_service::CsvListener;
use crate::listeners::disease_tracker::Hotspot;
use crate::listeners::events_kafka_producer::EventsKafkaProducer;
use crate::listeners::intervention_reporter::InterventionReporter;
use crate::listeners::listener::{Listener, Listeners};
use crate::listeners::travel_counter::TravelCounter;
use crate::models::constants;
use crate::models::events::Counts;
use crate::run_mode::{MultiEngineMode, RunMode};
use crate::state_machine::DiseaseHandler;
use crate::transport::engine_handlers::EngineHandlers;
use crate::transport::Transport;
use crate::travel::commute::Commuter;
use crate::travel::commute::CommutersByRegion;
use crate::travel::migration::{EngineMigrationPlan, Migrator};
use crate::utils::create_out_dir_if_not_present;
use crate::utils::util::{counts_at_start, output_file_format};

pub struct Epidemiology<D: DiseaseHandler + Sync, T: Transport, EH: EngineHandlers> {
    engine_id: String,
    pub citizen_location_map: CitizenLocationMap,
    pub travel_plan_config: Option<TravelPlanConfig>,
    pub config: Config,
    counts_at_hr: Counts,
    listeners: Listeners,
    interventions: Interventions,
    rng: RandomWrapper,
    disease_handler: D,
    transport: Option<Arc<Mutex<T>>>,
    engine_handlers: EH,
    run_mode: RunMode,
}

impl<D: DiseaseHandler + Sync, T: Transport, EH: EngineHandlers> Epidemiology<D, T, EH> {
    pub fn new(
        engine_id: String,
        config: Config,
        travel_plan_config: Option<TravelPlanConfig>,
        run_mode: &RunMode,
        disease_handler: D,
        transport: Option<T>,
        engine_handlers: EH,
        output_dir_path: &Path,
    ) -> Self {
        let start = Instant::now();
        let start_infections = config.get_starting_infections();
        let output_path = create_out_dir_if_not_present(output_dir_path);
        let mut grid = geography::define_geography(config.get_grid_size(), engine_id.clone());
        let mut rng = RandomWrapper::new();
        let (start_locations, agent_list) = match config.get_population() {
            Population::Csv(csv_pop) => grid.read_population(csv_pop, start_infections, &mut rng, &engine_id, &output_path),
            Population::Auto(auto_pop) => grid.generate_population(
                auto_pop,
                start_infections,
                &mut rng,
                &travel_plan_config,
                engine_id.clone(),
                &output_path,
            ),
        };
        grid.resize_hospital(
            agent_list.len() as i32,
            constants::HOSPITAL_STAFF_PERCENTAGE,
            config.get_geography_parameters().hospital_beds_percentage,
            engine_id.clone(),
        );

        let mut citizen_location_map = CitizenLocationMap::new(grid, &agent_list, &start_locations);

        info!("Initialization completed in {} seconds", start.elapsed().as_secs_f32());
        let current_population = citizen_location_map.current_population();

        //creating a path {output_dir_path}/output if it doesn't exist already
        let listeners = Self::create_listeners(&engine_id, current_population as usize, run_mode, &config, &output_path);
        let counts_at_hr = counts_at_start(current_population, config.get_starting_infections());

        let interventions = Self::init_interventions(&config, &mut citizen_location_map, &mut rng);
        let transport = transport.map(|x| Arc::new(Mutex::new(x)));
        let run_mode = run_mode.clone();

        Epidemiology {
            engine_id,
            interventions,
            counts_at_hr,
            listeners,
            config,
            travel_plan_config,
            citizen_location_map,
            rng,
            disease_handler,
            transport,
            engine_handlers,
            run_mode,
        }
    }

    fn create_listeners(
        engine_id: &str,
        current_pop: usize,
        run_mode: &RunMode,
        config: &Config,
        output_dir_path: &PathBuf,
    ) -> Listeners {
        let output_file_format = output_file_format(output_dir_path, engine_id.to_string());
        let counts_file_name = format!("{output_file_format}.csv");

        let csv_listener = CsvListener::new(counts_file_name);

        let hotspot_tracker = Hotspot::new();
        let intervention_reporter = InterventionReporter::new(format!("{output_file_format}_interventions.json"));
        let mut listeners_vec: Vec<Box<dyn Listener>> =
            vec![Box::new(csv_listener), Box::new(hotspot_tracker), Box::new(intervention_reporter)];

        match run_mode {
            RunMode::Standalone => {}

            RunMode::MultiEngine { mode } => match mode {
                MultiEngineMode::Kafka => {
                    let travels_file_name = format!("{output_file_format}_outgoing_travels.csv");
                    let travel_counter = TravelCounter::new(travels_file_name);
                    listeners_vec.push(Box::new(travel_counter));

                    let kafka_listener =
                        EventsKafkaProducer::new(engine_id.to_string(), current_pop, config.enable_citizen_state_messages());
                    listeners_vec.push(Box::new(kafka_listener));
                }
                MultiEngineMode::MPI => {
                    let travels_file_name = format!("{output_file_format}_outgoing_travels.csv");
                    let travel_counter = TravelCounter::new(travels_file_name);
                    listeners_vec.push(Box::new(travel_counter));
                }
            },
        }

        Listeners::from(listeners_vec)
    }

    fn init_interventions(
        config: &Config,
        citizen_location_map: &mut CitizenLocationMap,
        rng: &mut RandomWrapper,
    ) -> Interventions {
        let vaccinations = VaccinateIntervention::init(config);
        let lock_down_details = LockdownIntervention::init(config);
        let hospital_intervention = BuildNewHospital::init(config);
        let essential_workers_population = lock_down_details.get_essential_workers_percentage();

        citizen_location_map.iter_mut().for_each(|r| {
            (*r.1).assign_essential_worker(essential_workers_population, rng);
        });
        Interventions { vaccinate: vaccinations, lockdown: lock_down_details, build_new_hospital: hospital_intervention }
    }

    pub async fn run(&mut self, threads: u32) {
        rayon::ThreadPoolBuilder::new().num_threads(threads as usize).build_global().unwrap();

        self.listeners.grid_updated(&self.citizen_location_map.grid);
        match self.run_mode {
            RunMode::MultiEngine { .. } => {
                let tracer = global::tracer("epirust-trace");
                let mut span = tracer.start(format!("multi-engine - {}", self.engine_id));
                span.set_attribute(KeyValue::new("mode", "multi-engine"));
                span.set_attribute(KeyValue::new("engine_id", self.engine_id.to_string()));
                let cx = Context::current_with_span(span);
                self.run_multi_engine().with_context(cx).await
            }
            _ => self.run_single_engine().await,
        }
    }

    pub async fn run_single_engine(&mut self) {
        let start_time = Instant::now();
        let mut outgoing_migrators = Vec::new();
        let mut outgoing_commuters = Vec::new();
        let percent_outgoing = 0.0;

        let counts_at_hr = self.counts_at_hr.borrow_mut();
        let interventions = self.interventions.borrow_mut();
        let rng = self.rng.borrow_mut();

        counts_at_hr.log();
        let listeners = self.listeners.borrow_mut();
        for simulation_hour in 1..self.config.get_hours() {
            counts_at_hr.increment_hour();

            let population_before_travel = self.citizen_location_map.current_population();

            if population_before_travel == 0 {
                panic!("No citizens!");
            }

            self.citizen_location_map.simulate(
                counts_at_hr,
                simulation_hour,
                listeners,
                rng,
                percent_outgoing,
                &mut outgoing_migrators,
                &mut outgoing_commuters,
                self.config.enable_citizen_state_messages(),
                None,
                &self.engine_id,
                &self.disease_handler,
            );

            listeners.counts_updated(*counts_at_hr);
            self.citizen_location_map.process_interventions(
                interventions,
                counts_at_hr,
                listeners,
                rng,
                &self.config,
                &self.engine_id,
            );

            if Self::stop_simulation(&mut interventions.lockdown, &self.run_mode, *counts_at_hr) {
                break;
            }

            if simulation_hour % 100 == 0 {
                info!(
                    "Throughput: {} iterations/sec; simulation hour {} of {}",
                    simulation_hour as f32 / start_time.elapsed().as_secs_f32(),
                    simulation_hour,
                    self.config.get_hours()
                );
                counts_at_hr.log();
            }
        }
        let elapsed_time = start_time.elapsed().as_secs_f32();
        info!("Number of iterations: {}, Total Time taken {} seconds", counts_at_hr.get_hour(), elapsed_time);
        info!("Iterations/sec: {}", counts_at_hr.get_hour() as f32 / elapsed_time);
        listeners.simulation_ended();
    }

    pub async fn run_multi_engine(&mut self) {
        let start_time = Instant::now();
        // let mut producer = KafkaProducer::new();
        let engine_id = self.engine_id.to_string();

        let travel_plan_config = self.travel_plan_config.as_ref().unwrap();

        debug!("{}: Start Multi Engine Simulation", engine_id);
        let is_commute_enabled = travel_plan_config.commute.enabled;
        let is_migration_enabled = travel_plan_config.migration.enabled;

        let migration_plan = if is_migration_enabled { Some(travel_plan_config.migration_plan()) } else { None };

        let mut engine_migration_plan =
            EngineMigrationPlan::new(engine_id.clone(), migration_plan, self.citizen_location_map.current_population());

        debug!("{}: Start Migrator Consumer", engine_id);
        // let migrators_consumer = travel_consumer::start(engine_id, &[&*format!("{MIGRATION_TOPIC}{engine_id}")], "migrate");
        // let mut migration_stream = migrators_consumer.stream();

        let commute_plan = if is_commute_enabled {
            travel_plan_config.commute_plan()
        } else {
            CommutePlan { regions: Vec::new(), matrix: Vec::new() }
        };

        debug!("{}: Start Commuter Consumer", engine_id);
        // let commute_consumer = travel_consumer::start(engine_id, &[&*format!("{COMMUTE_TOPIC}{engine_id}")], "commute");
        // let mut commute_stream = commute_consumer.stream();

        // let ticks_consumer = ticks_consumer::start(engine_id);
        // let mut ticks_stream = ticks_consumer.stream();

        let mut n_incoming = 0;
        let mut n_outgoing = 0;

        let counts_at_hr = self.counts_at_hr.borrow_mut();
        let interventions = self.interventions.borrow_mut();
        let rng = self.rng.borrow_mut();
        let disease_handler = self.disease_handler.borrow();
        let engine_handlers = self.engine_handlers.borrow_mut();

        let transport = self.transport.borrow();

        counts_at_hr.log();

        let mut total_tick_sync_time = 0;
        let mut total_receive_commute_sync_time = 0;
        let mut total_receive_migration_sync_time = 0;
        let mut total_send_commuters_time = 0;
        let mut total_send_migrator_time = 0;
        // let run_mode = RunMode::MultiEngine { mode: };

        let hours = self.config.get_hours();
        let config = &self.config;
        for simulation_hour in 1..hours {
            // let transport = transport.as_ref();
            let start_time = Instant::now();
            let tracer = global::tracer("epirust-trace");

            let tick = transport
                .clone()
                .unwrap()
                .try_lock()
                .unwrap()
                .receive_tick(simulation_hour, is_commute_enabled, is_migration_enabled)
                .await;
            if let Some(t) = tick {
                total_tick_sync_time += start_time.elapsed().as_millis();
                info!("total tick sync time as hour {} - is {}", simulation_hour, total_tick_sync_time);
                if t.terminate() {
                    info!("received tick {:?}", t);
                    break;
                }
            }

            counts_at_hr.increment_hour();

            let population_before_travel = self.citizen_location_map.current_population();

            if population_before_travel == 0 {
                panic!("No citizens!");
            }
            if is_migration_enabled {
                engine_migration_plan.set_current_population(population_before_travel);
            }

            let mut percent_outgoing = 0.0;
            let mut outgoing: Vec<(Point, Migrator)> = Vec::new();

            if simulation_hour % 24 == 0 && is_migration_enabled {
                percent_outgoing = engine_migration_plan.percent_outgoing();
            }
            let mut actual_outgoing: Vec<(Point, Migrator)> = Vec::new();

            let received_migrators = async {
                let migrators = if is_migration_enabled {
                    let arc = transport.clone().unwrap();
                    let mut guard = arc.try_lock().unwrap();
                    debug!("{}: Received Migrators | Simulation hour: {}", engine_id, simulation_hour);
                    // let (incoming, ) = join!(result.as_mut().unwrap().receive_migrators(tick.unwrap().hour(), &engine_migration_plan));
                    let vec = guard.receive_migrators(simulation_hour, &engine_migration_plan).await;
                    Some(vec)
                    // Some(engine_migration_plan.receive_migrators(tick, &mut migration_stream))
                } else {
                    None
                };
                migrators
            };

            let mut outgoing_commuters: Vec<(Point, Commuter)> = Vec::new();
            let location_map = self.citizen_location_map.borrow_mut();
            let listeners = self.listeners.borrow_mut();
            let sim = async {
                debug!("{}: Start simulation for hour: {}", engine_id, simulation_hour);
                location_map.simulate(
                    counts_at_hr,
                    simulation_hour,
                    listeners,
                    rng,
                    percent_outgoing,
                    &mut outgoing,
                    &mut outgoing_commuters,
                    config.enable_citizen_state_messages(),
                    Some(travel_plan_config),
                    &engine_id,
                    disease_handler,
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

                if is_migration_enabled && tick.is_some() {
                    debug!("{}: Send Migrators", engine_id);
                    let send_migrator_start_time = Instant::now();
                    transport
                        .clone()
                        .unwrap()
                        .try_lock()
                        .unwrap()
                        .send_migrators(tick.unwrap().hour(), outgoing_migrators_by_region)
                        .await;
                    // Self::send_migrators(, &mut producer, outgoing_migrators_by_region);
                    debug!("{}: Send Migrators Successful", engine_id);
                    total_send_migrator_time += send_migrator_start_time.elapsed().as_millis();
                }
                if is_commute_enabled && tick.is_some() {
                    debug!("{}: Send Commuters", engine_id);
                    let send_commuter_start_time = Instant::now();
                    transport
                        .clone()
                        .unwrap()
                        .try_lock()
                        .unwrap()
                        .send_commuters(tick.unwrap().hour(), outgoing_commuters_by_region)
                        .await;
                    debug!("{}: Send Commuters Successful", engine_id);
                    // Self::send_commuters(tick, &mut producer, outgoing_commuters_by_region);
                    total_send_commuters_time += send_commuter_start_time.elapsed().as_millis();
                }
            };

            let mut span1 = tracer.start("simulation");
            span1.set_attribute(KeyValue::new("hour", simulation_hour.to_string()));
            let cx1 = Context::current_with_span(span1);
            let _ = join!(sim).with_context(cx1);

            if is_migration_enabled {
                let migration_start_time = Instant::now();
                let (incoming,) = join!(received_migrators);
                total_receive_migration_sync_time += migration_start_time.elapsed().as_millis();
                // let mut incoming = received_migrators.unwrap();
                let mut incoming = incoming.unwrap();
                n_incoming += incoming.len();
                n_outgoing += outgoing.len();
                self.citizen_location_map.remove_migrators(&actual_outgoing, counts_at_hr);
                self.citizen_location_map.assimilate_migrators(&mut incoming, counts_at_hr, rng);
                debug!("{}: assimilated the migrators", engine_id);
            }

            let option = transport.clone().unwrap();

            if is_commute_enabled && tick.is_some() {
                let commute_start_time = Instant::now();
                let mut span2 = tracer.start("receive_commuters");
                span2.set_attribute(KeyValue::new("hour", simulation_hour.to_string()));
                let cx2 = Context::current_with_span(span2);
                // let received_commuters = commute::receive_commuters(&commute_plan, tick, &mut commute_stream, engine_id);
                let mut guard1 = option.try_lock().unwrap();
                let received_commuters = guard1.receive_commuters(tick.unwrap().hour(), &commute_plan);
                let mut incoming_commuters = received_commuters.with_context(cx2).await;
                total_receive_commute_sync_time += commute_start_time.elapsed().as_millis();
                info!("total commute sync time as hour {} - is {}", simulation_hour, total_receive_commute_sync_time);
                n_incoming += incoming_commuters.len();
                n_outgoing += outgoing_commuters.len();
                self.citizen_location_map.remove_commuters(&outgoing_commuters, counts_at_hr);
                self.citizen_location_map.assimilate_commuters(&mut incoming_commuters, counts_at_hr, rng, simulation_hour);
                debug!("{}: assimilated the commuters", engine_id);
            }

            self.listeners.counts_updated(*counts_at_hr);
            self.citizen_location_map.process_interventions(
                interventions,
                counts_at_hr,
                &mut self.listeners,
                rng,
                &self.config,
                &self.engine_id,
            );

            if Self::stop_simulation(&mut interventions.lockdown, &self.run_mode, *counts_at_hr) {
                break;
            }

            engine_handlers.on_tick_end(
                &self.engine_id,
                *counts_at_hr,
                simulation_hour,
                &interventions.lockdown,
                is_commute_enabled,
                is_migration_enabled,
            );
            // send_ack(
            //
            //     &mut producer,
            //     ation_enabled,
            // );

            if simulation_hour % 100 == 0 {
                info!(
                    "Throughput: {} iterations/sec; simulation hour {} of {}",
                    simulation_hour as f32 / start_time.elapsed().as_secs_f32(),
                    simulation_hour,
                    self.config.get_hours()
                );
                counts_at_hr.log();
                info!(
                    "Incoming: {}, Outgoing: {}, Current Population: {}",
                    n_incoming,
                    n_outgoing,
                    self.citizen_location_map.current_population()
                );
                n_incoming = 0;
                n_outgoing = 0;
            }
        }
        let elapsed_time = start_time.elapsed().as_secs_f32();
        info!("Number of iterations: {}, Total Time taken {} seconds", counts_at_hr.get_hour(), elapsed_time);
        info!("Iterations/sec: {}", counts_at_hr.get_hour() as f32 / elapsed_time);
        info!("total tick sync time: {}", total_tick_sync_time);
        info!("total receive commute sync time: {}", total_receive_commute_sync_time);
        info!("total receive migration sync time: {}", total_receive_migration_sync_time);
        info!("total send commuters sync time: {}", total_send_commuters_time);
        info!("total send migrators sync time: {}", total_send_migrator_time);
        self.listeners.simulation_ended();
    }

    // fn send_migrators(tick: Option<Tick>, producer: &mut KafkaProducer, outgoing: Vec<MigratorsByRegion>) {
    //     if tick.is_some() && tick.unwrap().hour() % 24 == 0 {
    //         producer.send_migrators(outgoing);
    //     }
    // }
    //
    // fn send_commuters(tick_op: Option<Tick>, producer: &mut KafkaProducer, outgoing: Vec<CommutersByRegion>) {
    //     if let Some(tick) = tick_op {
    //         let hour = tick.hour() % 24;
    //         if hour == constants::ROUTINE_TRAVEL_START_TIME || hour == constants::ROUTINE_TRAVEL_END_TIME {
    //             producer.send_commuters(outgoing);
    //         }
    //     }
    // }

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
    // #[test]
    // fn should_init() {
    //     let pop = AutoPopulation { number_of_agents: 10, public_transport_percentage: 1.0, working_percentage: 1.0 };
    //     let disease = Disease::new(0, 0, 0, 0, 0, 0.0, 0.0, 0.0, 0.0, 0.0, 0, 0);
    //     let vac = VaccinateConfig { at_hour: 5000, percent: 0.2 };
    //     let geography_parameters = GeographyParameters::new(100, 0.003);
    //     let config = Config::new(
    //         Population::Auto(pop),
    //         Some(disease),
    //         geography_parameters,
    //         vec![],
    //         100,
    //         vec![InterventionConfig::Vaccinate(vac)],
    //         None,
    //     );
    //     let epidemiology: Epidemiology<_> =
    //         Epidemiology::new(config, None, STANDALONE_SIM_ID.to_string(), &RunMode::Standalone, disease, KafkaTransport::);
    //     let expected_housing_area = Area::new(&STANDALONE_SIM_ID.to_string(), Point::new(0, 0), Point::new(39, 100));
    //     assert_eq!(epidemiology.citizen_location_map.grid.housing_area, expected_housing_area);
    //
    //     let expected_transport_area = Area::new(&STANDALONE_SIM_ID.to_string(), Point::new(40, 0), Point::new(59, 100));
    //     assert_eq!(epidemiology.citizen_location_map.grid.transport_area, expected_transport_area);
    //
    //     let expected_work_area = Area::new(&STANDALONE_SIM_ID.to_string(), Point::new(60, 0), Point::new(79, 100));
    //     assert_eq!(epidemiology.citizen_location_map.grid.work_area, expected_work_area);
    //
    //     let expected_hospital_area = Area::new(&STANDALONE_SIM_ID.to_string(), Point::new(80, 0), Point::new(89, 0));
    //     assert_eq!(epidemiology.citizen_location_map.grid.hospital_area, expected_hospital_area);
    //
    //     assert_eq!(epidemiology.citizen_location_map.current_population(), 10);
    // }
}
