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

use core::borrow::Borrow;
use core::borrow::BorrowMut;
use std::time::{Duration, Instant, SystemTime};

use chrono::{DateTime, Local};
use futures::StreamExt;
use rand::Rng;

use crate::{allocation_map, RunMode, ticks_consumer, migrators_consumer, constants, commute_consumer};
use crate::allocation_map::AgentLocationMap;
use crate::config::{Config, Population, StartingInfections};
use crate::disease::Disease;
use crate::geography;
use crate::geography::{Grid, Point};
use crate::interventions::hospital::BuildNewHospital;
use crate::interventions::lockdown::LockdownIntervention;
use crate::interventions::vaccination::VaccinateIntervention;
use crate::kafka_producer::{KafkaProducer, TickAck};
use crate::listeners::csv_service::CsvListener;
use crate::listeners::disease_tracker::Hotspot;
use crate::listeners::events::counts::Counts;
use crate::listeners::events_kafka_producer::EventsKafkaProducer;
use crate::listeners::listener::{Listeners, Listener};
use crate::random_wrapper::RandomWrapper;
use rdkafka::consumer::{MessageStream, DefaultConsumerContext};
use crate::ticks_consumer::Tick;
use crate::travel_plan::{EngineMigrationPlan, MigratorsByRegion, Migrator, MigrationPlan};
use futures::join;
use crate::listeners::travel_counter::TravelCounter;
use crate::listeners::intervention_reporter::InterventionReporter;
use crate::interventions::Interventions;
use crate::constants::HOSPITAL_STAFF_PERCENTAGE;
use crate::agent::Citizen;
use crate::commute::{CommutePlan, Commuter, CommutersByRegion};
use crate::custom_types::{Count, Hour};
use crate::disease_state_machine::State;
use crate::kafka_consumer::TravelPlanConfig;

pub struct Epidemiology {
    pub agent_location_map: allocation_map::AgentLocationMap,
    pub write_agent_location_map: allocation_map::AgentLocationMap,
    pub grid: Grid,
    pub disease: Disease,
    pub sim_id: String,
}

impl Epidemiology {
    pub fn new(config: &Config, travel_plan_config: Option<TravelPlanConfig>, sim_id: String) -> Epidemiology {
        let start = Instant::now();
        let disease = config.get_disease();
        let start_infections = config.get_starting_infections();
        let mut grid = geography::define_geography(config.get_grid_size(), sim_id.clone());
        let mut rng = RandomWrapper::new();
        let (start_locations, agent_list) = match config.get_population() {
            Population::Csv(csv_pop) => grid.read_population(&csv_pop, &start_infections, &mut rng, &sim_id),
            Population::Auto(auto_pop) => grid.generate_population(&auto_pop, &start_infections, &mut rng, travel_plan_config, sim_id.clone()),
        };
        grid.resize_hospital(agent_list.len() as i32, HOSPITAL_STAFF_PERCENTAGE, config.get_geography_parameters().hospital_beds_percentage, sim_id.clone());

        let agent_location_map = allocation_map::AgentLocationMap::new(config.get_grid_size(), &agent_list, &start_locations);
        let write_agent_location_map = agent_location_map.clone();

        info!("Initialization completed in {} seconds", start.elapsed().as_secs_f32());
        Epidemiology { agent_location_map, write_agent_location_map, grid, disease, sim_id }
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
            _ => zero_active_cases
        }
    }

    fn output_file_format(config: &Config, run_mode: &RunMode) -> String {
        let now: DateTime<Local> = SystemTime::now().into();
        let mut output_file_prefix = config.get_output_file().unwrap_or("simulation".to_string());
        if let RunMode::MultiEngine { engine_id } = run_mode {
            output_file_prefix = format!("{}_{}", output_file_prefix, engine_id);
        }
        format!("output/{}_{}", output_file_prefix, now.format("%Y-%m-%dT%H:%M:%S"))
    }

    fn create_listeners(&self, config: &Config, run_mode: &RunMode) -> Listeners {
        let output_file_format = Epidemiology::output_file_format(config, run_mode);
        let counts_file_name = format!("{}.csv", output_file_format);

        let csv_listener = CsvListener::new(counts_file_name);
        let population = self.agent_location_map.current_population();

        let hotspot_tracker = Hotspot::new();
        let intervention_reporter = InterventionReporter::new(format!("{}_interventions.json", output_file_format));
        let mut listeners_vec: Vec<Box<dyn Listener>> = vec![Box::new(csv_listener),
                                                             Box::new(hotspot_tracker),
                                                             Box::new(intervention_reporter)];

        match run_mode {
            RunMode::Standalone => {}
            RunMode::SingleDaemon => {
                let kafka_listener = EventsKafkaProducer::new(self.sim_id.clone(), population as usize,
                                                              config.enable_citizen_state_messages());
                listeners_vec.push(Box::new(kafka_listener));
            }
            RunMode::MultiEngine { .. } => {
                let travels_file_name = format!("{}_outgoing_travels.csv", output_file_format);
                let travel_counter = TravelCounter::new(travels_file_name);
                listeners_vec.push(Box::new(travel_counter));

                let kafka_listener = EventsKafkaProducer::new(self.sim_id.clone(), population as usize,
                                                              config.enable_citizen_state_messages());
                listeners_vec.push(Box::new(kafka_listener));
            }
        }

        Listeners::from(listeners_vec)
    }

    fn counts_at_start(population: Count, start_infections: &StartingInfections) -> Counts {
        let s = population - start_infections.total();
        let e = start_infections.get_exposed();
        let i = start_infections.total_infected();
        assert_eq!(s + e + i, population);
        Counts::new(s, e, i)
    }

    fn init_interventions(&mut self, config: &Config, rng: &mut RandomWrapper) -> Interventions {
        let vaccinations = VaccinateIntervention::init(config);
        let lock_down_details = LockdownIntervention::init(config);
        let hospital_intervention = BuildNewHospital::init(config);
        let essential_workers_population = lock_down_details.get_essential_workers_percentage();

        for (_, agent) in self.agent_location_map.iter_mut() {
            agent.assign_essential_worker(essential_workers_population, rng);
        }
        Interventions {
            vaccinate: vaccinations,
            lockdown: lock_down_details,
            build_new_hospital: hospital_intervention,
        }
    }

    fn process_interventions(interventions: &mut Interventions, counts_at_hr: &Counts,
                             listeners: &mut Listeners, rng: &mut RandomWrapper, write_buffer: &mut AgentLocationMap,
                             config: &Config, grid: &mut Grid, sim_id: String) {
        Epidemiology::apply_vaccination_intervention(
            &interventions.vaccinate,
            counts_at_hr,
            write_buffer,
            rng,
            listeners,
        );

        if interventions.lockdown.should_apply(counts_at_hr) {
            interventions.lockdown.apply();
            Epidemiology::lock_city(counts_at_hr.get_hour(), write_buffer);
            listeners.intervention_applied(counts_at_hr.get_hour(), &interventions.lockdown)
        }
        if interventions.lockdown.should_unlock(counts_at_hr) {
            Epidemiology::unlock_city(counts_at_hr.get_hour(), write_buffer);
            interventions.lockdown.unapply();
            listeners.intervention_applied(counts_at_hr.get_hour(), &interventions.lockdown)
        }

        interventions.build_new_hospital.counts_updated(counts_at_hr);
        if interventions.build_new_hospital.should_apply(counts_at_hr) {
            info!("Increasing the hospital size");
            grid.increase_hospital_size(config.get_grid_size(), sim_id);
            interventions.build_new_hospital.apply();

            listeners.grid_updated(grid);
            listeners.intervention_applied(counts_at_hr.get_hour(), &interventions.build_new_hospital);
        }
    }

    pub async fn run(&mut self, config: &Config, travel_plan_config: Option<TravelPlanConfig>, run_mode: &RunMode) {
        let mut listeners = self.create_listeners(config, run_mode);
        let population = self.agent_location_map.current_population();
        let mut counts_at_hr = Epidemiology::counts_at_start(population, &config.get_starting_infections());
        let mut rng = RandomWrapper::new();

        self.write_agent_location_map.init_with_capacity(population as usize);

        let mut interventions = self.init_interventions(config, &mut rng);

        listeners.grid_updated(&self.grid);
        match run_mode {
            RunMode::MultiEngine { .. } => {
                self.run_multi_engine(config, travel_plan_config.unwrap(), run_mode,  &mut listeners, &mut counts_at_hr,
                                      &mut interventions, &mut rng).await
            }
            _ => {
                self.run_single_engine(config, run_mode, &mut listeners, &mut counts_at_hr,
                                       &mut interventions, &mut rng, self.sim_id.to_string()).await
            }
        }
    }

    pub async fn run_single_engine(&mut self, config: &Config, run_mode: &RunMode, listeners: &mut Listeners,
                                   counts_at_hr: &mut Counts, interventions: &mut Interventions, rng: &mut RandomWrapper, sim_id: String) {
        let start_time = Instant::now();
        let mut outgoing = Vec::new();
        let mut outgoing_commuters = Vec::new();
        let percent_outgoing = 0.0;

        counts_at_hr.log();
        for simulation_hour in 1..config.get_hours() {
            counts_at_hr.increment_hour();

            let mut read_buffer_reference = self.agent_location_map.borrow();
            let mut write_buffer_reference = self.write_agent_location_map.borrow_mut();

            if simulation_hour % 2 == 0 {
                read_buffer_reference = self.write_agent_location_map.borrow();
                write_buffer_reference = self.agent_location_map.borrow_mut();
            }

            let population_before_travel = read_buffer_reference.current_population();

            if population_before_travel == 0 {
                panic!("No citizens!");
            }

            Epidemiology::simulate(counts_at_hr, simulation_hour, read_buffer_reference, write_buffer_reference,
                                   &self.grid, listeners, rng, &self.disease, percent_outgoing,
                                   &mut outgoing, &mut outgoing_commuters, config.enable_citizen_state_messages(), None, &sim_id);

            listeners.counts_updated(*counts_at_hr);
            Epidemiology::process_interventions(interventions, counts_at_hr, listeners,
                                                rng, write_buffer_reference, config, &mut self.grid, sim_id.to_string());

            if Epidemiology::stop_simulation(&mut interventions.lockdown, run_mode, *counts_at_hr) {
                break;
            }

            if simulation_hour % 100 == 0 {
                info!("Throughput: {} iterations/sec; simulation hour {} of {}",
                      simulation_hour as f32 / start_time.elapsed().as_secs_f32(),
                      simulation_hour, config.get_hours());
                counts_at_hr.log();
            }
        }
        let elapsed_time = start_time.elapsed().as_secs_f32();
        info!("Number of iterations: {}, Total Time taken {} seconds", counts_at_hr.get_hour(), elapsed_time);
        info!("Iterations/sec: {}", counts_at_hr.get_hour() as f32 / elapsed_time);
        listeners.simulation_ended();
    }

    pub async fn run_multi_engine(&mut self, config: &Config, travel_plan_config: TravelPlanConfig, run_mode: &RunMode, listeners: &mut Listeners,
                                  counts_at_hr: &mut Counts, interventions: &mut Interventions, rng: &mut RandomWrapper) {
        let start_time = Instant::now();
        let mut producer = KafkaProducer::new();
        let migration_plan = MigrationPlan::new(travel_plan_config.get_regions(), travel_plan_config.get_migration_matrix());

        //todo stream should be started only in case of multi-sim mode
        let standalone_engine_id = "standalone".to_string();
        let engine_id = if let RunMode::MultiEngine { engine_id } = run_mode {
            engine_id
        } else {
            &standalone_engine_id
        };

        let mut engine_migration_plan = EngineMigrationPlan::new(engine_id, Some(migration_plan),self.agent_location_map.current_population());
        let commute_plan = travel_plan_config.commute_plan();
        let ticks_consumer = ticks_consumer::start(engine_id);
        let mut ticks_stream = ticks_consumer.start_with(Duration::from_millis(1), false);
        let migrators_consumer = migrators_consumer::start(engine_id);
        let mut migration_stream = migrators_consumer.start_with(Duration::from_millis(1), false);
        let commute_consumer = commute_consumer::start(engine_id);
        let mut commute_stream = commute_consumer.start_with(Duration::from_millis(1), false);
        let mut outgoing: Vec<(Point, Migrator)> = Vec::new();
        let mut n_incoming = 0;
        let mut n_outgoing = 0;

        counts_at_hr.log();
        for simulation_hour in 1..config.get_hours() {
            let tick = Epidemiology::receive_tick(run_mode, &mut ticks_stream, simulation_hour).await;
            match &tick {
                None => {}
                Some(t) => {
                    if t.terminate() {
                        info!("received tick {:?}", t);
                        break;
                    }
                }
            }
            outgoing.clear();

            counts_at_hr.increment_hour();

            let mut read_buffer_reference = self.agent_location_map.borrow();
            let mut write_buffer_reference = self.write_agent_location_map.borrow_mut();

            if simulation_hour % 2 == 0 {
                read_buffer_reference = self.write_agent_location_map.borrow();
                write_buffer_reference = self.agent_location_map.borrow_mut();
            }

            let population_before_travel = read_buffer_reference.current_population();

            if population_before_travel == 0 {
                panic!("No citizens!");
            }
            engine_migration_plan.set_current_population(read_buffer_reference.current_population());

            let grid = &self.grid;
            let disease = &self.disease;
            let mut percent_outgoing = 0.0;

            if simulation_hour % 24 == 0 {
                percent_outgoing = engine_migration_plan.percent_outgoing();
            }
            let mut actual_outgoing: Vec<(Point, Migrator)> = Vec::new();
            let received_migrators = Epidemiology::receive_migrators(tick.clone(), &mut migration_stream, &engine_migration_plan);
            let mut outgoing_commuters : Vec<Commuter> = Vec::new();
            let sim = async {
                Epidemiology::simulate(counts_at_hr, simulation_hour, read_buffer_reference, write_buffer_reference,
                                       grid, listeners, rng, disease, percent_outgoing,
                                       &mut outgoing, &mut outgoing_commuters, config.enable_citizen_state_messages(), Some(&travel_plan_config), engine_id);
                let (outgoing_migrators_by_region, actual_total_outgoing) = engine_migration_plan.alloc_outgoing_to_regions(&outgoing);
                actual_outgoing = actual_total_outgoing;
                if simulation_hour % 24 == 0 {
                    listeners.outgoing_migrators_added(simulation_hour, &outgoing_migrators_by_region);
                }

                let outgoing_commuters_by_region = commute_plan.get_commuters_by_region(outgoing_commuters);

                Epidemiology::send_migrators(tick.clone(), &mut producer, outgoing_migrators_by_region);
                Epidemiology::send_commuters(tick.clone(), &mut producer, outgoing_commuters_by_region);
            };

            let received_commuters = Epidemiology::receive_commuters(tick.clone(), &mut commute_stream, &commute_plan, engine_id);

            let (mut incoming, mut incoming_commuters, ()) = join!(received_migrators, received_commuters, sim);
            n_incoming += incoming.len();
            n_outgoing += outgoing.len();
            write_buffer_reference.remove_migrators(&actual_outgoing, counts_at_hr, &mut self.grid);
            write_buffer_reference.assimilate_migrators(&mut incoming, &mut self.grid, counts_at_hr, rng);

            write_buffer_reference.assimilate_commuters( &mut incoming_commuters, &mut self.grid, counts_at_hr, rng);

            listeners.counts_updated(*counts_at_hr);
            Epidemiology::process_interventions(interventions, counts_at_hr, listeners,
                                                rng, write_buffer_reference, config, &mut self.grid, engine_id.to_string());

            if Epidemiology::stop_simulation(&mut interventions.lockdown, run_mode, *counts_at_hr) {
                break;
            }

            Epidemiology::send_ack(run_mode, &mut producer, *counts_at_hr, simulation_hour, &interventions.lockdown).await;

            if simulation_hour % 100 == 0 {
                info!("Throughput: {} iterations/sec; simulation hour {} of {}",
                      simulation_hour as f32 / start_time.elapsed().as_secs_f32(),
                      simulation_hour, config.get_hours());
                counts_at_hr.log();
                info!("Incoming: {}, Outgoing: {}, Current Population: {}", n_incoming, n_outgoing,
                      write_buffer_reference.current_population());
                n_incoming = 0;
                n_outgoing = 0;
            }
        }
        let elapsed_time = start_time.elapsed().as_secs_f32();
        info!("Number of iterations: {}, Total Time taken {} seconds", counts_at_hr.get_hour(), elapsed_time);
        info!("Iterations/sec: {}", counts_at_hr.get_hour() as f32 / elapsed_time);
        listeners.simulation_ended();
    }

    async fn extract_tick(message_stream: &mut MessageStream<'_, DefaultConsumerContext>) -> Tick {
        let msg = message_stream.next().await;
        let mut tick = ticks_consumer::read(msg);
        while  tick == None  {
            let next_msg = message_stream.next().await;
            tick = ticks_consumer::read(next_msg);
        }
        tick.unwrap()
    }

    async fn receive_tick(run_mode: &RunMode, message_stream: &mut MessageStream<'_, DefaultConsumerContext>,
                          simulation_hour: Hour) -> Option<Tick> {
        if simulation_hour > 1 && simulation_hour % 24 != 0 && simulation_hour % 24 != constants::ROUTINE_TRAVEL_END_TIME && simulation_hour % 24 != constants::ROUTINE_TRAVEL_START_TIME {
            return None;
        }
        if let RunMode::MultiEngine { engine_id: _e } = run_mode {
            let t = Epidemiology::extract_tick(message_stream).await;
            if t.hour() != simulation_hour {
                panic!("Local hour is {}, but received tick for {}", simulation_hour, t.hour());
            }
            Some(t)
        } else {
            None
        }
    }

    async fn send_ack(run_mode: &RunMode, producer: &mut KafkaProducer, counts: Counts, simulation_hour: Hour,
                      lockdown: &LockdownIntervention) {
        if simulation_hour > 1 && simulation_hour % 24 != 0 && simulation_hour % 24!= constants::ROUTINE_TRAVEL_START_TIME && simulation_hour % 24 != constants::ROUTINE_TRAVEL_END_TIME {
            return;
        }
        if let RunMode::MultiEngine { engine_id } = run_mode {
            let ack = TickAck {
                engine_id: engine_id.to_string(),
                hour: simulation_hour,
                counts,
                locked_down: lockdown.is_locked_down(),
            };
            match producer.send_ack(&ack).await.unwrap() {
                Ok(_) => {}
                Err(e) => panic!("Failed while sending acknowledgement: {:?}", e.0)
            }
        }
    }

    fn send_migrators(tick: Option<Tick>, producer: &mut KafkaProducer, outgoing: Vec<MigratorsByRegion>) {
        if tick.is_some() && tick.unwrap().hour() % 24 == 0 {
            producer.send_migrators(outgoing);
        }
    }

    fn send_commuters(tick: Option<Tick>, producer: &mut KafkaProducer, outgoing: Vec<CommutersByRegion>) {
        if tick.is_some(){
            let hour = tick.unwrap().hour() % 24;
            if hour == constants::ROUTINE_TRAVEL_START_TIME || hour == constants::ROUTINE_TRAVEL_END_TIME {
                producer.send_commuters(outgoing);
            }
        }
    }

    async fn receive_migrators(tick: Option<Tick>, message_stream: &mut MessageStream<'_, DefaultConsumerContext>,
                               engine_migration_plan: &EngineMigrationPlan) -> Vec<Migrator> {
        if tick.is_some() && tick.unwrap().hour() % 24 == 0 {
            let expected_incoming_regions = engine_migration_plan.incoming_regions_count();
            let mut received_incoming_regions = 0;
            debug!("Receiving migrators from {} regions", expected_incoming_regions);
            let mut incoming: Vec<Migrator> = Vec::new();
            while expected_incoming_regions != received_incoming_regions {
                let maybe_msg = Epidemiology::receive_migrators_from_region(message_stream, engine_migration_plan).await;
                match maybe_msg {
                    None => {}
                    Some(region_incoming) => {
                        incoming.extend(region_incoming.get_migrators());
                        received_incoming_regions += 1;
                    }
                }
            }
            incoming
        } else {
            Vec::new()
        }
    }


    async fn receive_commuters(tick: Option<Tick>, message_stream: &mut MessageStream<'_, DefaultConsumerContext>,
                               commute_plan: &CommutePlan, engine_id: &String) -> Vec<Commuter> {
        if tick.is_some() && tick.unwrap().hour() % 24 == constants::ROUTINE_TRAVEL_START_TIME | constants::ROUTINE_TRAVEL_END_TIME {
            let expected_incoming_regions = commute_plan.incoming_regions_count(engine_id);
            let mut received_incoming_regions = 0;
            debug!("Receiving migrators from {} regions", expected_incoming_regions);
            let mut incoming: Vec<Commuter> = Vec::new();
            while expected_incoming_regions != received_incoming_regions {
                let maybe_msg = Epidemiology::receive_commuters_from_region(message_stream, engine_id).await;
                match maybe_msg {
                    None => {}
                    Some(region_incoming) => {
                        incoming.extend( region_incoming.get_commuters());
                        received_incoming_regions += 1;
                    }
                }
            }
            incoming
        } else {
            Vec::new()
        }
    }

    async fn receive_commuters_from_region(message_stream: &mut MessageStream<'_, DefaultConsumerContext>,
                                           engine_id: &String) -> Option<CommutersByRegion> {
        let msg = message_stream.next().await;
        commute_consumer::read(msg).filter(|incoming| {
            incoming.to_engine_id() == engine_id
        })
    }

    async fn receive_migrators_from_region(message_stream: &mut MessageStream<'_, DefaultConsumerContext>,
                                           engine_migration_plan: &EngineMigrationPlan) -> Option<MigratorsByRegion> {
        let msg = message_stream.next().await;
        migrators_consumer::read(msg).filter(|incoming| {
            incoming.to_engine_id() == engine_migration_plan.engine_id()
        })
    }

    fn apply_vaccination_intervention(vaccinations: &VaccinateIntervention, counts: &Counts,
                                      write_buffer_reference: &mut AgentLocationMap, rng: &mut RandomWrapper,
                                      listeners: &mut Listeners) {
        match vaccinations.get_vaccination_percentage(counts) {
            Some(vac_percent) => {
                info!("Vaccination");
                Epidemiology::vaccinate(*vac_percent, write_buffer_reference, rng);
                listeners.intervention_applied(counts.get_hour(), vaccinations)
            }
            _ => {}
        };
    }

    fn vaccinate(vaccination_percentage: f64, write_buffer_reference: &mut AgentLocationMap, rng: &mut RandomWrapper) {
        for (_v, agent) in write_buffer_reference.iter_mut() {
            if agent.state_machine.is_susceptible() && rng.get().gen_bool(vaccination_percentage) {
                agent.set_vaccination(true);
            }
        }
    }

    fn simulate(csv_record: &mut Counts, simulation_hour: Hour, read_buffer: &AgentLocationMap,
                write_buffer: &mut AgentLocationMap, grid: &Grid, listeners: &mut Listeners,
                rng: &mut RandomWrapper, disease: &Disease, percent_outgoing: f64,
                outgoing: &mut Vec<(Point, Migrator)>, outgoing_commuters: &mut Vec<Commuter>,
                publish_citizen_state: bool, travel_plan_config: Option<&TravelPlanConfig>,
                region_name: &String) {
        write_buffer.clear();
        csv_record.clear();
        for (cell, agent) in read_buffer.iter() {
            let mut current_agent = agent.clone();
            let infection_status = current_agent.state_machine.is_infected();
            let point = current_agent.perform_operation(*cell, simulation_hour, grid, read_buffer, rng, disease);
            Epidemiology::update_counts(csv_record, &current_agent);

            if !infection_status && current_agent.state_machine.is_infected() {
                listeners.citizen_got_infected(cell);
            }

            let agent_option = write_buffer.get(&point);
            let new_location = match agent_option {
                Some(mut _agent) => cell, //occupied
                _ => &point
            };

            let start_migration_hour= if travel_plan_config.is_some() {travel_plan_config.unwrap().get_start_migration_hour()} else {0};
            let end_migration_hour = if travel_plan_config.is_some() {travel_plan_config.unwrap().get_end_migration_hour()} else {0};

            // this code get executed only in multi-engine simulations mode
            if simulation_hour % 24 == 0 && current_agent.can_move() && current_agent.work_location.location_id == *region_name &&
                simulation_hour > start_migration_hour && simulation_hour < end_migration_hour &&
                rng.get().gen_bool(percent_outgoing) {
                    let migrator = Migrator::from(&current_agent);
                    outgoing.push((*new_location, migrator));

            }

            if (simulation_hour % 24 == constants::ROUTINE_TRAVEL_START_TIME || simulation_hour % 24 == constants::ROUTINE_TRAVEL_END_TIME) && current_agent.can_move() && current_agent.work_location.location_id != *region_name {
                let commuter = Commuter::from(&current_agent);
                outgoing_commuters.push(commuter);
            }

            write_buffer.insert(*new_location, current_agent.clone());
            if publish_citizen_state {
                listeners.citizen_state_updated(simulation_hour, &current_agent, new_location);
            }
        }
        assert_eq!(csv_record.total(), write_buffer.current_population());
    }

    fn update_counts(counts_at_hr: &mut Counts, citizen: &Citizen) {
        match citizen.state_machine.state {
            State::Susceptible { .. } => { counts_at_hr.update_susceptible(1) },
            State::Exposed { .. } => { counts_at_hr.update_exposed(1) },
            State::Infected { .. } => {
                if citizen.is_hospitalized() {
                    counts_at_hr.update_hospitalized(1);
                } else {
                    counts_at_hr.update_infected(1)
                }
            },
            State::Recovered { .. } => { counts_at_hr.update_recovered(1) },
            State::Deceased { .. } => { counts_at_hr.update_deceased(1) } ,
        }
    }

    fn lock_city(hr: Hour, write_buffer_reference: &mut AgentLocationMap) {
        info!("Locking the city. Hour: {}", hr);
        for (_v, agent) in write_buffer_reference.iter_mut() {
            if !agent.is_essential_worker() {
                agent.set_isolation(true);
            }
        }
    }

    fn unlock_city(hr: Hour, write_buffer_reference: &mut AgentLocationMap) {
        info!("Unlocking city. Hour: {}", hr);
        for (_v, agent) in write_buffer_reference.iter_mut() {
            if agent.is_isolated() {
                agent.set_isolation(false);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::config::{AutoPopulation, GeographyParameters};
    use crate::geography::Area;
    use crate::geography::Point;
    use crate::interventions::InterventionConfig;
    use crate::interventions::vaccination::VaccinateConfig;
    use crate::STANDALONE_SIM_ID;

    use super::*;

    #[test]
    fn should_init() {
        let pop = AutoPopulation {
            number_of_agents: 10,
            public_transport_percentage: 1.0,
            working_percentage: 1.0,
        };
        let disease = Disease::new(0, 0, 0, 0, 0, 0.0, 0.0, 0.0, 0.0, 0.0, 0, 0);
        let vac = VaccinateConfig {
            at_hour: 5000,
            percent: 0.2,
        };
        let geography_parameters = GeographyParameters::new(100, 0.003);
        let config = Config::new(Population::Auto(pop), disease, geography_parameters, vec![], 100, vec![InterventionConfig::Vaccinate(vac)], None);
        let epidemiology: Epidemiology = Epidemiology::new(&config, None,"id".to_string());
        let expected_housing_area = Area::new(STANDALONE_SIM_ID.to_string(), Point::new(0, 0), Point::new(39, 100));
        assert_eq!(epidemiology.grid.housing_area, expected_housing_area);

        let expected_transport_area = Area::new(STANDALONE_SIM_ID.to_string(), Point::new(40, 0), Point::new(49, 100));
        assert_eq!(epidemiology.grid.transport_area, expected_transport_area);

        let expected_work_area = Area::new(STANDALONE_SIM_ID.to_string(), Point::new(50, 0), Point::new(69, 100));
        assert_eq!(epidemiology.grid.work_area, expected_work_area);

        let expected_hospital_area = Area::new(STANDALONE_SIM_ID.to_string(), Point::new(70, 0), Point::new(79, 0));
        assert_eq!(epidemiology.grid.hospital_area, expected_hospital_area);

        assert_eq!(epidemiology.agent_location_map.current_population(), 10);
    }
}
