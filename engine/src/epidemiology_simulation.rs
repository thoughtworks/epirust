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
use fxhash::{FxBuildHasher, FxHashMap};
use rand::Rng;

use crate::{allocation_map, RunMode, ticks_consumer, travellers_consumer};
use crate::allocation_map::AgentLocationMap;
use crate::config::{Config, Population};
use crate::disease::Disease;
use crate::geography;
use crate::geography::Grid;
use crate::interventions::hospital::BuildNewHospital;
use crate::interventions::lockdown::{LockdownConfig, LockdownIntervention};
use crate::interventions::vaccination::VaccinateIntervention;
use crate::kafka_producer::{KafkaProducer, TickAck};
use crate::listeners::csv_service::CsvListener;
use crate::listeners::disease_tracker::Hotspot;
use crate::listeners::events::counts::Counts;
use crate::listeners::events_kafka_producer::EventsKafkaProducer;
use crate::listeners::listener::Listeners;
use crate::random_wrapper::RandomWrapper;
use rdkafka::consumer::{MessageStream, DefaultConsumerContext};
use crate::ticks_consumer::Tick;
use crate::travel_plan::{EngineTravelPlan, TravellersByRegion};
use crate::agent::Citizen;

pub struct Epidemiology {
    pub agent_location_map: allocation_map::AgentLocationMap,
    pub write_agent_location_map: allocation_map::AgentLocationMap,
    pub grid: Grid,
    pub disease: Disease,
    pub sim_id: String,
}

impl Epidemiology {
    pub fn new(config: &Config, sim_id: String) -> Epidemiology {
        let start = Instant::now();
        let disease = config.get_disease();
        let grid = geography::define_geography(config.get_grid_size());
        let mut rng = RandomWrapper::new();
        let (start_locations, agent_list) = match config.get_population() {
            Population::Csv(csv_pop) => grid.read_population(&csv_pop, &mut rng),
            Population::Auto(auto_pop) => grid.generate_population(&auto_pop, &mut rng),
        };

        let agent_location_map = allocation_map::AgentLocationMap::new(config.get_grid_size(), &agent_list, &start_locations);
        let write_agent_location_map = agent_location_map.clone();

        info!("Initialization completed in {} seconds", start.elapsed().as_secs_f32());
        Epidemiology { agent_location_map, write_agent_location_map, grid, disease, sim_id }
    }

    fn stop_simulation(row: Counts) -> bool {
        row.get_infected() == 0 && row.get_quarantined() == 0
    }

    fn output_file_name(config: &Config, run_mode: &RunMode) -> String {
        let now: DateTime<Local> = SystemTime::now().into();
        let mut output_file_prefix = config.get_output_file().unwrap_or("simulation".to_string());
        if let RunMode::MultiEngine { engine_id } = run_mode {
            output_file_prefix = format!("{}_{}", output_file_prefix, engine_id);
        }
        format!("{}_{}.csv", output_file_prefix, now.format("%Y-%m-%dT%H:%M:%S"))
    }

    fn create_listeners(&self, config: &Config, run_mode: &RunMode) -> Listeners {
        let output_file_name = Epidemiology::output_file_name(config, run_mode);
        let csv_listener = CsvListener::new(output_file_name);
        let population = self.agent_location_map.current_population();
        let kafka_listener = EventsKafkaProducer::new(self.sim_id.clone(), population as usize,
                                                      config.enable_citizen_state_messages());
        let hotspot_tracker = Hotspot::new();
        Listeners::from(vec![Box::new(csv_listener), Box::new(kafka_listener), Box::new(hotspot_tracker)])
    }

    pub async fn run(&mut self, config: &Config, run_mode: &RunMode) {
        let mut listeners = self.create_listeners(config, run_mode);
        let population = self.agent_location_map.current_population();
        let mut counts_at_hr = Counts::new(population - 1, 1);
        let mut rng = RandomWrapper::new();
        let start_time = Instant::now();

        self.write_agent_location_map.agent_cell = FxHashMap::with_capacity_and_hasher(population as usize,
                                                                                       FxBuildHasher::default());

        let vaccinations = VaccinateIntervention::init(config);
        let mut lock_down_details = LockdownIntervention::init(config);
        let mut hospital_intervention = BuildNewHospital::init(config);

        listeners.grid_updated(&self.grid);
        let mut producer = KafkaProducer::new();

        //todo stream should be started only in case of multi-sim mode
        let standalone_engine_id = "standalone".to_string();
        let engine_id = if let RunMode::MultiEngine { engine_id } = run_mode {
            engine_id
        } else {
            &standalone_engine_id
        };
        let mut engine_travel_plan = EngineTravelPlan::new(engine_id, population);
        let ticks_consumer = ticks_consumer::start(engine_id);
        let mut ticks_stream = ticks_consumer.start_with(Duration::from_millis(10), false);
        let travellers_consumer = travellers_consumer::start(engine_id);
        let mut travel_stream = travellers_consumer.start_with(Duration::from_millis(10), false);
        let mut terminate_engine = false;
        let mut n_incoming = 0;
        let mut n_outgoing = 0;

        for simulation_hour in 1..config.get_hours() {
            let tick = Epidemiology::receive_tick(run_mode, &mut ticks_stream, simulation_hour).await;
            engine_travel_plan.receive_tick(tick.clone());

            counts_at_hr.increment_hour();

            let mut read_buffer_reference = self.agent_location_map.borrow();
            let mut write_buffer_reference = self.write_agent_location_map.borrow_mut();

            if simulation_hour % 2 == 0 {
                read_buffer_reference = self.write_agent_location_map.borrow();
                write_buffer_reference = self.agent_location_map.borrow_mut();
            }

            if read_buffer_reference.current_population() == 0 {
                panic!("No citizens!");
            }
            engine_travel_plan.set_current_population(read_buffer_reference.current_population());

            if hospital_intervention.should_apply(&counts_at_hr) {
                info!("Increasing the hospital size");
                self.grid.increase_hospital_size(config.get_grid_size());
                listeners.grid_updated(&self.grid);
            }

            Epidemiology::simulate(&mut counts_at_hr, simulation_hour, read_buffer_reference, write_buffer_reference,
                                   &self.grid, &mut listeners, &mut rng, &self.disease, &mut engine_travel_plan);
            Epidemiology::send_travellers(tick.clone(), &mut producer, &mut engine_travel_plan).await;
            let mut incoming = Epidemiology::receive_travellers(tick.clone(), &mut travel_stream, &engine_travel_plan).await;
            n_incoming += incoming.len();
            n_outgoing += engine_travel_plan.get_outgoing().len();
            write_buffer_reference.remove_citizens(engine_travel_plan.get_outgoing(), &mut counts_at_hr);
            write_buffer_reference.assimilate_citizens(&mut incoming, &mut self.grid, &mut counts_at_hr, &mut rng);

            listeners.counts_updated(counts_at_hr);
            hospital_intervention.counts_updated(&counts_at_hr);

            if lock_down_details.should_apply(&counts_at_hr) {
                lock_down_details.apply(&counts_at_hr);
                Epidemiology::lock_city(&mut write_buffer_reference, &mut rng, &lock_down_details.get_config().unwrap());
            }

            if lock_down_details.should_unlock(&counts_at_hr) {
                Epidemiology::unlock_city(&mut write_buffer_reference);
            }

            Epidemiology::apply_vaccination_intervention(&vaccinations, &counts_at_hr, &mut write_buffer_reference, &mut rng);

            if Epidemiology::stop_simulation(counts_at_hr) {
                terminate_engine = true;
            }

            Epidemiology::send_ack(run_mode, &mut producer, terminate_engine, simulation_hour).await;

            if simulation_hour % 100 == 0 {
                info!("Throughput: {} iterations/sec; simulation hour {} of {}",
                      simulation_hour as f32 / start_time.elapsed().as_secs_f32(),
                      simulation_hour, config.get_hours());
                info!("S: {}, I: {}, Q: {}, R: {}, D: {}", counts_at_hr.get_susceptible(), counts_at_hr.get_infected(),
                      counts_at_hr.get_quarantined(), counts_at_hr.get_recovered(), counts_at_hr.get_deceased());
                info!("Incoming: {}, Outgoing: {}, Current Population: {}", n_incoming, n_outgoing,
                      write_buffer_reference.current_population());
                n_incoming = 0;
                n_outgoing = 0;
            }

            if terminate_engine {
                break;
            }
        }
        let elapsed_time = start_time.elapsed().as_secs_f32();
        info!("Number of iterations: {}, Total Time taken {} seconds", counts_at_hr.get_hour(), elapsed_time);
        info!("Iterations/sec: {}", counts_at_hr.get_hour() as f32 / elapsed_time);
        listeners.simulation_ended();
    }

    async fn receive_tick(run_mode: &RunMode, message_stream: &mut MessageStream<'_, DefaultConsumerContext>,
                          simulation_hour: i32) -> Option<Tick> {
        if let RunMode::MultiEngine { engine_id: _e } = run_mode {
            let msg = message_stream.next().await;
            let clock_tick = ticks_consumer::read(msg);
            match clock_tick {
                None => { None }
                Some(t) => {
                    debug!("tick {}", t.hour());
                    if t.hour() != simulation_hour {
                        panic!("Local hour is {}, but received tick for {}", simulation_hour, t.hour());
                    }
                    Some(t)
                }
            }
        } else {
            None
        }
    }

    async fn send_ack(run_mode: &RunMode, producer: &mut KafkaProducer, terminate_engine: bool, simulation_hour: i32) {
        if let RunMode::MultiEngine { engine_id } = run_mode {
            let ack = TickAck { engine_id: engine_id.to_string(), hour: simulation_hour, terminate: terminate_engine };
            match producer.send_ack(&ack).await.unwrap() {
                Ok(_) => {}
                Err(e) => panic!("Failed while sending acknowledgement: {:?}", e.0)
            }
        }
    }

    async fn send_travellers(tick: Option<Tick>, producer: &mut KafkaProducer, engine_travel_plan: &mut EngineTravelPlan) {
        if tick.is_some() && tick.unwrap().hour() % 24 == 0 {
            let outgoing = engine_travel_plan.alloc_outgoing_to_regions();
            match producer.send_travellers(outgoing).await.unwrap() {
                Ok(_) => {}
                Err(e) => { panic!("Failed to send travellers: {:?}", e.0) }
            }
        }
    }

    async fn receive_travellers(tick: Option<Tick>, message_stream: &mut MessageStream<'_, DefaultConsumerContext>,
                                engine_travel_plan: &EngineTravelPlan) -> Vec<Citizen> {
        if tick.is_some() && tick.unwrap().hour() % 24 == 0 {
            let expected_incoming_regions = engine_travel_plan.incoming_regions_count();
            let mut received_incoming_regions = 0;
            debug!("Receiving travellers from {} regions", expected_incoming_regions);
            let mut incoming: Vec<Citizen> = Vec::new();
            while expected_incoming_regions != received_incoming_regions {
                let region_incoming = Epidemiology::receive_travellers_from_region(message_stream, engine_travel_plan).await;
                debug!("received travels: {}", region_incoming.len());
                for i in region_incoming {
                    incoming.extend(i.get_citizens());
                    received_incoming_regions += 1;
                }
            }
            incoming
        } else {
            Vec::new()
        }
    }

    async fn receive_travellers_from_region(message_stream: &mut MessageStream<'_, DefaultConsumerContext>,
                                            engine_travel_plan: &EngineTravelPlan) -> Vec<TravellersByRegion> {
        let msg = message_stream.next().await;

        travellers_consumer::read(msg).into_iter()
            .filter(|incoming| incoming.to_engine_id() == engine_travel_plan.engine_id())
            .collect()
    }

    fn apply_vaccination_intervention(vaccinations: &VaccinateIntervention, counts: &Counts,
                                      write_buffer_reference: &mut AgentLocationMap, rng: &mut RandomWrapper) {
        match vaccinations.get_vaccination_percentage(counts) {
            Some(vac_percent) => {
                info!("Vaccination");
                Epidemiology::vaccinate(*vac_percent, write_buffer_reference, rng);
            }
            _ => {}
        };
    }

    fn vaccinate(vaccination_percentage: f64, write_buffer_reference: &mut AgentLocationMap, rng: &mut RandomWrapper) {
        for (_v, agent) in write_buffer_reference.agent_cell.iter_mut() {
            if agent.state_machine.is_susceptible() && rng.get().gen_bool(vaccination_percentage) {
                agent.set_vaccination(true);
            }
        }
    }

    fn simulate(mut csv_record: &mut Counts, simulation_hour: i32, read_buffer: &AgentLocationMap,
                write_buffer: &mut AgentLocationMap, grid: &Grid, listeners: &mut Listeners,
                rng: &mut RandomWrapper, disease: &Disease, engine_travel_plan: &mut EngineTravelPlan) {
        write_buffer.agent_cell.clear();
        for (cell, agent) in read_buffer.agent_cell.iter() {
            let mut current_agent = *agent;
            let infection_status = current_agent.state_machine.is_infected();
            let point = current_agent.perform_operation(*cell, simulation_hour, &grid, read_buffer, &mut csv_record, rng, disease);

            if infection_status == false && current_agent.state_machine.is_infected() == true {
                listeners.citizen_got_infected(&cell);
            }

            let agent_option = write_buffer.agent_cell.get(&point);
            let new_location = match agent_option {
                Some(mut _agent) => cell, //occupied
                _ => &point
            };

            if simulation_hour % 24 == 0 && current_agent.can_move()
                && rng.get().gen_bool(engine_travel_plan.percent_outgoing()) {
                engine_travel_plan.add_outgoing(current_agent, *new_location);
            }

            write_buffer.agent_cell.insert(*new_location, current_agent);
            listeners.citizen_state_updated(simulation_hour, &current_agent, new_location);
        }
    }

    fn lock_city(write_buffer_reference: &mut AgentLocationMap, rng: &mut RandomWrapper, lockdown_details: &LockdownConfig) {
        info!("Locking the city");
        for (_v, agent) in write_buffer_reference.agent_cell.iter_mut() {
            if rng.get().gen_bool(1.0 - lockdown_details.essential_workers_population) {
                agent.set_isolation(true);
            }
        }
    }

    fn unlock_city(write_buffer_reference: &mut AgentLocationMap) {
        info!("unlocking city");
        for (_v, agent) in write_buffer_reference.agent_cell.iter_mut() {
            if agent.is_isolated() {
                agent.set_isolation(false);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::config::AutoPopulation;
    use crate::geography::Area;
    use crate::geography::Point;
    use crate::interventions::Intervention;
    use crate::interventions::vaccination::VaccinateConfig;

    use super::*;

    #[test]
    fn should_init() {
        let pop = AutoPopulation {
            number_of_agents: 10,
            public_transport_percentage: 1.0,
            working_percentage: 1.0,
        };
        let disease = Disease::new(0, 0, 0, 0.0, 0.0, 0.0);
        let vac = VaccinateConfig {
            at_hour: 5000,
            percent: 0.2,
        };
        let config = Config::new(Population::Auto(pop), disease, vec![], 100, 10000,
                                 vec![Intervention::Vaccinate(vac)], None);
        let epidemiology: Epidemiology = Epidemiology::new(&config, "id".to_string());
        let expected_housing_area = Area::new(Point::new(0, 0), Point::new(40, 100));
        assert_eq!(epidemiology.grid.housing_area, expected_housing_area);

        let expected_transport_area = Area::new(Point::new(40, 0), Point::new(50, 100));
        assert_eq!(epidemiology.grid.transport_area, expected_transport_area);

        let expected_work_area = Area::new(Point::new(50, 0), Point::new(70, 100));
        assert_eq!(epidemiology.grid.work_area, expected_work_area);

        let expected_hospital_area = Area::new(Point::new(70, 0), Point::new(80, 100));
        assert_eq!(epidemiology.grid.hospital_area, expected_hospital_area);

        assert_eq!(epidemiology.agent_location_map.current_population(), 10);
    }
}
