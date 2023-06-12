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

use bincode::{deserialize, serialize};
use core::borrow::BorrowMut;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::io::Read;
use std::time::Instant;

use crate::config::{Config, Population, TravelPlanConfig};
use futures::{join, TryFutureExt};
use mpi::request::{LocalScope, Request, RequestCollection, WaitGuard};
use mpi::topology::SystemCommunicator;
use mpi::traits::{Communicator, Destination, Source};
use mpi::Rank;
use opentelemetry::trace::{FutureExt, Span, TraceContextExt, Tracer};
use opentelemetry::{global, Context, KeyValue};
use snap::raw::{decompress_len, Decoder, Encoder};
use snap::read::{FrameDecoder, FrameEncoder};
use snap::{read, write};

use crate::allocation_map::CitizenLocationMap;
use crate::geography;
use crate::geography::Point;
use crate::interventions::hospital::BuildNewHospital;
use crate::interventions::lockdown::LockdownIntervention;
use crate::interventions::vaccination::VaccinateIntervention;
use crate::interventions::Interventions;
use crate::listeners::csv_service::CsvListener;
use crate::listeners::disease_tracker::Hotspot;
use crate::listeners::intervention_reporter::InterventionReporter;
use crate::listeners::listener::{Listener, Listeners};
use crate::listeners::travel_counter::TravelCounter;
use crate::models::constants;
use crate::models::custom_types::Hour;
use crate::models::events::Counts;
use crate::run_mode::RunMode;
use crate::state_machine::DiseaseHandler;
use crate::travel::commute;
use crate::travel::commute::commute_plan::CommutePlan;
use crate::travel::commute::Commuter;
use crate::travel::commute::CommutersByRegion;
use crate::travel::migration::{EngineMigrationPlan, Migrator, MigratorsByRegion};
use crate::utils::random_wrapper::RandomWrapper;
use crate::utils::util::{counts_at_start, output_file_format};

pub struct Epidemiology<T: DiseaseHandler + Sync> {
    pub citizen_location_map: CitizenLocationMap,
    pub engine_id: String,
    pub travel_plan_config: Option<TravelPlanConfig>,
    pub config: Config,
    counts_at_hr: Counts,
    listeners: Listeners,
    interventions: Interventions,
    rng: RandomWrapper,
    disease_handler: T,
}

impl<T: DiseaseHandler + Sync> Epidemiology<T> {
    pub fn new(
        config: Config,
        travel_plan_config: Option<TravelPlanConfig>,
        engine_id: String,
        run_mode: &RunMode,
        disease_handler: T,
    ) -> Self {
        let start = Instant::now();
        let start_infections = config.get_starting_infections();
        let mut grid = geography::define_geography(config.get_grid_size(), engine_id.clone());
        let mut rng = RandomWrapper::new();
        let (start_locations, agent_list) = match config.get_population() {
            Population::Csv(csv_pop) => grid.read_population(csv_pop, start_infections, &mut rng, &engine_id),
            Population::Auto(auto_pop) => {
                grid.generate_population(auto_pop, start_infections, &mut rng, &travel_plan_config, engine_id.clone())
            }
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
        let listeners = Self::create_listeners(&engine_id, run_mode, &config);
        let counts_at_hr = counts_at_start(current_population, config.get_starting_infections());

        let interventions = Self::init_interventions(&config, &mut citizen_location_map, &mut rng);

        Epidemiology {
            interventions,
            counts_at_hr,
            listeners,
            config,
            travel_plan_config,
            citizen_location_map,
            engine_id,
            rng,
            disease_handler,
        }
    }

    fn create_listeners(engine_id: &str, run_mode: &RunMode, config: &Config) -> Listeners {
        let output_file_format = output_file_format(config, run_mode, engine_id.to_string());
        let counts_file_name = format!("{output_file_format}.csv");

        let csv_listener = CsvListener::new(counts_file_name);

        let hotspot_tracker = Hotspot::new();
        let intervention_reporter = InterventionReporter::new(format!("{output_file_format}_interventions.json"));
        let mut listeners_vec: Vec<Box<dyn Listener>> =
            vec![Box::new(csv_listener), Box::new(hotspot_tracker), Box::new(intervention_reporter)];

        match run_mode {
            RunMode::Standalone => {}
            RunMode::MultiEngine { .. } => {
                let travels_file_name = format!("{output_file_format}_outgoing_travels.csv");
                let travel_counter = TravelCounter::new(travels_file_name);
                listeners_vec.push(Box::new(travel_counter));
            }
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

    pub async fn run(&mut self, run_mode: &RunMode, threads: u32) {
        rayon::ThreadPoolBuilder::new().num_threads(threads as usize).build_global().unwrap();

        self.listeners.grid_updated(&self.citizen_location_map.grid);
        let engine_id = &self.engine_id;
        match run_mode {
            RunMode::MultiEngine => {
                let tracer = global::tracer("epirust-trace");
                let mut span = tracer.start(format!("multi-engine - {engine_id}"));
                span.set_attribute(KeyValue::new("mode", "multi-engine"));
                span.set_attribute(KeyValue::new("engine_id", engine_id.to_string()));
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

            if Self::stop_simulation(&mut interventions.lockdown, &RunMode::Standalone, *counts_at_hr) {
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
        let travel_plan_config = self.travel_plan_config.as_ref().unwrap();
        let engine_id = &self.engine_id;

        let world = SystemCommunicator::world();
        let rank = world.rank();
        let mut engine_ranks: HashMap<String, Rank> = HashMap::new();
        for (i, engine) in travel_plan_config.regions.iter().enumerate() {
            engine_ranks.insert(engine.clone(), Rank::from(i as u8));
        }
        let ranks = engine_ranks.values().cloned().collect::<Vec<Rank>>();
        info!("engine ranks - {:?}", engine_ranks);
        info!("rank in the engine app - {}", rank);

        debug!("{}: Start Multi Engine Simulation", engine_id);
        let is_commute_enabled = travel_plan_config.commute.enabled;
        let is_migration_enabled = travel_plan_config.migration.enabled;

        let migration_plan = if is_migration_enabled { Some(travel_plan_config.migration_plan()) } else { None };

        let mut engine_migration_plan =
            EngineMigrationPlan::new(engine_id.clone(), migration_plan, self.citizen_location_map.current_population());

        let commute_plan = if is_commute_enabled {
            travel_plan_config.commute_plan()
        } else {
            CommutePlan { regions: Vec::new(), matrix: Vec::new() }
        };

        let mut n_incoming = 0;
        let mut n_outgoing = 0;

        let counts_at_hr = self.counts_at_hr.borrow_mut();
        let interventions = self.interventions.borrow_mut();
        let rng = self.rng.borrow_mut();
        let disease_handler = self.disease_handler.borrow();

        counts_at_hr.log();

        let hours = self.config.get_hours();
        let config = &self.config;
        for simulation_hour in 1..hours {
            let start_time = Instant::now();
            let tracer = global::tracer("epirust-trace");

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

            let received_migrators = if is_migration_enabled {
                debug!("{}: Received Migrators | Simulation hour: {}", engine_id, simulation_hour);
                Some(engine_migration_plan.receive_migrators(simulation_hour, world, &ranks))
            } else {
                None
            };

            let mut outgoing_commuters: Vec<(Point, Commuter)> = Vec::new();
            let mut incoming_commuters: Vec<Commuter> = Vec::new();
            let location_map = self.citizen_location_map.borrow_mut();
            let listeners = self.listeners.borrow_mut();
            let engine_id = &engine_id;
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
                    engine_id,
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

                if is_migration_enabled {
                    debug!("{}: Send Migrators", engine_id);
                    Self::send_migrators(simulation_hour, outgoing_migrators_by_region, &engine_ranks, world);
                }
                if is_commute_enabled {
                    debug!("{}: Send Commuters", engine_id);
                    incoming_commuters =
                        Self::send_recv_commuters(simulation_hour, outgoing_commuters_by_region, &engine_ranks, world);
                }
            };

            let mut span1 = tracer.start("simulation");
            span1.set_attribute(KeyValue::new("hour", simulation_hour.to_string()));
            let cx1 = Context::current_with_span(span1);
            let _ = join!(sim).with_context(cx1);

            if is_commute_enabled {
                let mut span2 = tracer.start("receive_commuters");
                span2.set_attribute(KeyValue::new("hour", simulation_hour.to_string()));
                let cx2 = Context::current_with_span(span2);
                // let received_commuters = commute::receive_commuters(&commute_plan, simulation_hour, engine_id, world, &ranks);
                // let mut incoming_commuters = received_commuters.with_context(cx2).await;
                n_incoming += incoming_commuters.len();
                n_outgoing += outgoing_commuters.len();
                self.citizen_location_map.remove_commuters(&outgoing_commuters, counts_at_hr);
                self.citizen_location_map.assimilate_commuters(&mut incoming_commuters, counts_at_hr, rng, simulation_hour);
                debug!("{}: assimilated the commuters", engine_id);
            }

            if is_migration_enabled {
                let (mut incoming,) = join!(received_migrators.unwrap());
                info!("receive migrator call is done for hour - {}", simulation_hour);
                n_incoming += incoming.len();
                n_outgoing += outgoing.len();
                self.citizen_location_map.remove_migrators(&actual_outgoing, counts_at_hr);
                self.citizen_location_map.assimilate_migrators(&mut incoming, counts_at_hr, rng);
                debug!("{}: assimilated the migrators", engine_id);
            }

            self.listeners.counts_updated(*counts_at_hr);
            self.citizen_location_map.process_interventions(
                interventions,
                counts_at_hr,
                &mut self.listeners,
                rng,
                &self.config,
                engine_id,
            );

            if Self::stop_simulation(&mut interventions.lockdown, &RunMode::MultiEngine, *counts_at_hr) {
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
        self.listeners.simulation_ended();
    }

    fn send_migrators(
        hour: Hour,
        outgoing: Vec<MigratorsByRegion>,
        engine_ranks: &HashMap<String, Rank>,
        world: SystemCommunicator,
    ) {
        if hour % 24 == 0 {
            for out_region in outgoing.iter() {
                let rank: &Rank = engine_ranks.iter().find(|(x, _)| *x == out_region.to_engine_id()).unwrap().1;
                let serialized = serialize(&out_region).unwrap();
                world.process_at_rank(*rank).send(&serialized[..]);
                debug!("sent migrators");
            }
        }
    }

    fn send_recv_commuters(
        hour: Hour,
        outgoing: Vec<CommutersByRegion>,
        engine_ranks: &HashMap<String, Rank>,
        world: SystemCommunicator,
    ) -> Vec<Commuter> {
        let h = hour % 24;
        let mut incoming: Vec<Commuter> = Vec::new();

        if h == constants::ROUTINE_TRAVEL_START_TIME || h == constants::ROUTINE_TRAVEL_END_TIME {
            let total_count = engine_ranks.iter().len();
            let self_rank = world.rank();
            assert_eq!(outgoing.len(), total_count);

            let serialized_commuters: Vec<(&Rank, Vec<u8>)> = outgoing
                .iter()
                .map(|s| {
                    let rank: &Rank = engine_ranks.iter().find(|(x, _)| *x == s.to_engine_id()).unwrap().1;
                    let serialized: Vec<u8> = serialize(&s).unwrap();
                    let compressed: Vec<u8> = Encoder::new().compress_vec(&serialized[..]).unwrap();
                    let length_of_buffer = compressed.len();
                    let mut compressed_data_with_length = serialize(&length_of_buffer).unwrap();
                    compressed_data_with_length.extend(compressed);
                    (rank, compressed_data_with_length)
                })
                .collect();
            let buffer = vec![0u8; 60000];
            let mut result = vec![buffer; total_count];

            mpi::request::multiple_scope(total_count, |scope, coll: &mut RequestCollection<[u8]>| {
                for (&rank, data) in serialized_commuters.iter() {
                    let p = world.process_at_rank(rank);
                    let sreq = p.immediate_ready_send_with_tag(scope, &data[..], world.rank());
                    coll.add(sreq);
                }
                let mut send_count = 0;
                while coll.incomplete() > 0 {
                    let (_u, s, r) = coll.wait_any().unwrap();
                    send_count += 1;
                }
                assert_eq!(send_count, total_count);
            });

            mpi::request::multiple_scope(total_count, |scope, coll: &mut RequestCollection<[u8]>| {
                for (index, value) in result.iter_mut().enumerate() {
                    let rank = Rank::from(index as u8);
                    let p = world.process_at_rank(rank);
                    let rreq = p.immediate_receive_into_with_tag(scope, &mut value[..], rank);
                    coll.add(rreq);
                }
                let mut recv_count = 0;
                while coll.incomplete() > 0 {
                    let (_u, s, r) = coll.wait_any().unwrap();
                    let length_of_msg: usize = deserialize::<u32>(&r[0..7]).unwrap() as usize;
                    let decompressed = Decoder::new().decompress_vec(&r[8..length_of_msg + 8]).unwrap();
                    let received: CommutersByRegion = deserialize(&decompressed[..]).unwrap();
                    info!(
                        "engine rank: {}, hour: {}, from_rank: {}, received_commuters - {:?}",
                        self_rank,
                        hour,
                        s.source_rank(),
                        received
                    );
                    incoming.extend(received.get_commuters());
                    recv_count += 1;
                }
                assert_eq!(recv_count, total_count);
            });
        }
        incoming
    }

    fn stop_simulation(lock_down_details: &mut LockdownIntervention, run_mode: &RunMode, row: Counts) -> bool {
        let zero_active_cases = row.get_exposed() == 0 && row.get_infected() == 0 && row.get_hospitalized() == 0;
        match run_mode {
            RunMode::MultiEngine => {
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
    use crate::config::intervention_config::{InterventionConfig, VaccinateConfig};
    use crate::config::{AutoPopulation, GeographyParameters};
    use crate::disease::Disease;
    use crate::geography::Area;
    use crate::geography::Point;

    use super::*;

    #[test]
    fn should_init() {
        let pop = AutoPopulation { number_of_agents: 10, public_transport_percentage: 1.0, working_percentage: 1.0 };
        let disease = Disease::new(0, 0, 0, 0, 0, 0.0, 0.0, 0.0, 0.0, 0.0, 0, 0);
        let vac = VaccinateConfig { at_hour: 5000, percent: 0.2 };
        let geography_parameters = GeographyParameters::new(100, 0.003);
        let config = Config::new(
            Population::Auto(pop),
            Some(disease),
            geography_parameters,
            vec![],
            100,
            vec![InterventionConfig::Vaccinate(vac)],
            None,
        );
        let engine_id = "some_id";
        let epidemiology: Epidemiology<_> = Epidemiology::new(config, None, engine_id.to_string(), &RunMode::Standalone, disease);
        let expected_housing_area = Area::new(&engine_id.to_string(), Point::new(0, 0), Point::new(39, 100));
        assert_eq!(epidemiology.citizen_location_map.grid.housing_area, expected_housing_area);

        let expected_transport_area = Area::new(&engine_id.to_string(), Point::new(40, 0), Point::new(59, 100));
        assert_eq!(epidemiology.citizen_location_map.grid.transport_area, expected_transport_area);

        let expected_work_area = Area::new(&engine_id.to_string(), Point::new(60, 0), Point::new(79, 100));
        assert_eq!(epidemiology.citizen_location_map.grid.work_area, expected_work_area);

        let expected_hospital_area = Area::new(&engine_id.to_string(), Point::new(80, 0), Point::new(89, 0));
        assert_eq!(epidemiology.citizen_location_map.grid.hospital_area, expected_hospital_area);

        assert_eq!(epidemiology.citizen_location_map.current_population(), 10);
    }
}
