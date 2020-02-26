use core::borrow::Borrow;
use core::borrow::BorrowMut;
use std::time::Instant;

use fxhash::{FxBuildHasher, FxHashMap};
use rand::Rng;

use crate::{allocation_map, events};
use crate::allocation_map::AgentLocationMap;
use crate::csv_service::CsvListener;
use crate::disease::Disease;
use crate::disease_tracker::Hotspot;
use crate::events::{Counts, Listener};
use crate::geography;
use crate::geography::{Grid, Point};
use crate::kafka_service::KafkaService;
use crate::random_wrapper::RandomWrapper;

pub struct Epidemiology {
    pub agent_location_map: allocation_map::AgentLocationMap,
    pub write_agent_location_map: allocation_map::AgentLocationMap,
    pub grid: Grid,
    pub disease: Disease,
}

impl Epidemiology {
    pub fn new(grid_size: i32, number_of_agents: i32, public_transport_percentage: f64, working_percentage: f64) -> Epidemiology {
        let start = Instant::now();
        let disease = Disease::init("config/diseases.yaml", "small_pox");
        let grid = geography::define_geography(grid_size);
        let mut rng = RandomWrapper::new();
        let (start_locations, agent_list) = grid.generate_population(number_of_agents, public_transport_percentage, working_percentage, &mut rng);
        let agent_location_map = allocation_map::AgentLocationMap::new(grid_size, &agent_list, &start_locations);
        let write_agent_location_map = allocation_map::AgentLocationMap::new(grid_size, &agent_list, &start_locations);

        println!("Initialization completed in {} seconds", start.elapsed().as_secs_f32());
        Epidemiology { agent_location_map, write_agent_location_map, grid, disease }
    }

    fn stop_simulation(row: Counts) -> bool {
        row.get_infected() == 0 && row.get_quarantined() == 0
    }

    pub fn run(&mut self, simulation_life_time: i32, vaccination_time: i32,
               vaccination_percentage: f64, output_file_name: &str) {
        let csv_listener = CsvListener::new(output_file_name);
        let kafka_listener = KafkaService::new();
        let hotspot_tracker = Hotspot::new();
        let mut listeners = Listeners::from(vec![Box::new(csv_listener), Box::new(kafka_listener), Box::new(hotspot_tracker)]);

        let mut counts_at_hr = Counts::new((self.agent_location_map.agent_cell.len() - 1) as i32, 1);
        let mut rng = RandomWrapper::new();
        let start_time = Instant::now();


        self.write_agent_location_map.agent_cell = FxHashMap::with_capacity_and_hasher(self.agent_location_map.agent_cell.len(), FxBuildHasher::default());

        for simulation_hour in 1..simulation_life_time {
            counts_at_hr.increment_hour();

            let mut read_buffer_reference = self.agent_location_map.borrow();
            let mut write_buffer_reference = self.write_agent_location_map.borrow_mut();

            if simulation_hour % 2 == 0 {
                read_buffer_reference = self.write_agent_location_map.borrow();
                write_buffer_reference = self.agent_location_map.borrow_mut();
            }

            Epidemiology::simulate(&mut counts_at_hr, simulation_hour, read_buffer_reference, write_buffer_reference,
                                   &self.grid, &mut listeners, &mut rng, &self.disease);
            listeners.counts_updated(counts_at_hr);

            if simulation_hour == vaccination_time {
                println!("Vaccination");
                Epidemiology::vaccinate(vaccination_percentage, &mut write_buffer_reference, &mut rng);
            }

            if Epidemiology::stop_simulation(counts_at_hr) {
                break;
            }

            if simulation_hour % 100 == 0 {
                println!("Throughput: {} iterations/sec; simulation hour {} of {}",
                         simulation_hour as f32 / start_time.elapsed().as_secs_f32(),
                         simulation_hour, simulation_life_time)
            }
        }
        let elapsed_time = start_time.elapsed().as_secs_f32();
        println!("Number of iterations: {}, Total Time taken {} seconds", counts_at_hr.get_hour(), elapsed_time);
        println!("Iterations/sec: {}", counts_at_hr.get_hour() as f32 / elapsed_time);
        listeners.simulation_ended();
    }

    fn vaccinate(vaccination_percentage: f64, write_buffer_reference: &mut AgentLocationMap, rng: &mut RandomWrapper) {
        for (_v, agent) in write_buffer_reference.agent_cell.iter_mut() {
            if agent.is_susceptible() && rng.get().gen_bool(vaccination_percentage) {
                agent.set_vaccination(true);
            }
        }
    }

    fn simulate(mut csv_record: &mut events::Counts, simulation_hour: i32, read_buffer: &AgentLocationMap,
                write_buffer: &mut AgentLocationMap, grid: &Grid, listeners: &mut Listeners,
                rng: &mut RandomWrapper, disease: &Disease) {
        write_buffer.agent_cell.clear();
        for (cell, agent) in read_buffer.agent_cell.iter() {
            let mut current_agent = *agent;
            let infection_status = current_agent.is_infected();
            let point = current_agent.perform_operation(*cell, simulation_hour, &grid, read_buffer, &mut csv_record, rng, disease);

            if infection_status == false && current_agent.is_infected() == true {
                listeners.citizen_got_infected(&cell);
            }

            let agent_option = write_buffer.agent_cell.get(&point);
            match agent_option {
                Some(mut _agent) => {
                    write_buffer.agent_cell.insert(*cell, current_agent);
                }
                _ => { write_buffer.agent_cell.insert(point, current_agent); }
            }
        }
    }
}

struct Listeners {
    listeners: Vec<Box<dyn Listener>>,
}

impl Listeners {
    fn from(listeners: Vec<Box<dyn Listener>>) -> Listeners {
        Listeners { listeners }
    }
}

impl Listener for Listeners {
    fn counts_updated(&mut self, counts: Counts) {
        self.listeners.iter_mut().for_each(|listener| { listener.counts_updated(counts) });
    }

    fn simulation_ended(&mut self) {
        self.listeners.iter_mut().for_each(|listener| { listener.simulation_ended() });
    }

    fn citizen_got_infected(&mut self, cell: &Point) {
        self.listeners.iter_mut().for_each(|listener| { listener.citizen_got_infected(cell) });
    }
}

#[cfg(test)]
mod tests {
    use crate::geography::Area;
    use crate::geography::Point;

    use super::*;

    #[test]
    fn should_init() {
        let epidemiology: Epidemiology = Epidemiology::new(20, 10, 1.0, 1.0);
        let expected_housing_area = Area::new(Point::new(0, 0), Point::new(7, 19));
        assert_eq!(epidemiology.grid.housing_area, expected_housing_area);

        let expected_transport_area = Area::new(Point::new(8, 0), Point::new(9, 19));
        assert_eq!(epidemiology.grid.transport_area, expected_transport_area);

        let expected_hospital_area = Area::new(Point::new(10, 0), Point::new(11, 19));
        assert_eq!(epidemiology.grid.hospital, expected_hospital_area);

        let expected_work_area = Area::new(Point::new(12, 0), Point::new(19, 19));
        assert_eq!(epidemiology.grid.work_area, expected_work_area);

        assert_eq!(epidemiology.agent_location_map.agent_cell.len(), 10);
    }
}
