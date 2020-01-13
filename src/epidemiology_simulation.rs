use crate::allocation_map;
use std::time::SystemTime;
use crate::csv_service;
use crate::csv_service::Row;
use hashbrown::HashMap;
use core::borrow::BorrowMut;
use crate::allocation_map::AgentLocationMap;
use core::borrow::Borrow;
use rand::thread_rng;
use rand::Rng;
use crate::geography;
use crate::geography::Grid;

pub struct Epidemiology {
    pub agent_location_map: allocation_map::AgentLocationMap,
    pub write_agent_location_map: allocation_map::AgentLocationMap,
    pub grid: Grid
}

impl Epidemiology {

    pub fn new(grid_size: i32, number_of_agents: i32, public_transport_percentage: f64, working_percentage:f64) -> Epidemiology {
        let grid = geography::define_geography(grid_size);
        let (home_locations, agent_list) = grid.generate_population(number_of_agents, public_transport_percentage, working_percentage);
        let agent_location_map = allocation_map::AgentLocationMap::new(grid_size, &agent_list, &home_locations);
        let write_agent_location_map = allocation_map::AgentLocationMap::new(grid_size, &agent_list, &home_locations);

        Epidemiology{agent_location_map, write_agent_location_map, grid}
    }

    fn stop_simulation(row: csv_service::Row) -> bool{
        row.get_infected() == 0 && row.get_quarantined() == 0
    }

    pub fn run(&mut self, simulation_life_time:i32, vaccination_time:i32,
               vaccination_percentage:f64, output_file_name: &str){
        let mut records: Vec<csv_service::Row> = Vec::new();
        let mut csv_record = Row::new((self.agent_location_map.agent_cell.len() - 1) as i32, 1);
        let start_time = SystemTime::now();
        self.write_agent_location_map.agent_cell = HashMap::with_capacity(self.agent_location_map.agent_cell.len());

        for simulation_hour in 1..simulation_life_time {
//            println!("Tick {}", simulation_hour);
            csv_record.increment_hour();

            let mut read_buffer_reference = self.agent_location_map.borrow();
            let mut write_buffer_reference = self.write_agent_location_map.borrow_mut();

            if simulation_hour % 2 == 0{
                read_buffer_reference = self.write_agent_location_map.borrow();
                write_buffer_reference = self.agent_location_map.borrow_mut();
            }

            Epidemiology::simulate(&mut csv_record, simulation_hour, read_buffer_reference, write_buffer_reference, &self.grid);
            records.push(csv_record);

            if simulation_hour == vaccination_time{
                println!("Vaccination");
                Epidemiology::vaccinate(vaccination_percentage, &mut write_buffer_reference);
            }

            if Epidemiology::stop_simulation(csv_record) {
                break;
            }
        }
        let end_time = SystemTime::now();
        println!("Number of iterations: {}, Total Time taken {:?}", csv_record.get_hour(), end_time.duration_since(start_time));
        let _result = csv_service::write(output_file_name, &records);
    }

    fn vaccinate(vaccination_percentage: f64, write_buffer_reference: &mut AgentLocationMap) {
        let mut rng = thread_rng();
        for (_v, agent) in write_buffer_reference.agent_cell.iter_mut() {
            if agent.is_susceptible() && rng.gen_bool(vaccination_percentage) {
                agent.set_vaccination(true);
            }
        }
    }

    fn simulate(mut csv_record: &mut Row, simulation_hour: i32, read_buffer: &AgentLocationMap, write_buffer: &mut AgentLocationMap, grid: &Grid) {
        write_buffer.agent_cell.clear();
        for (cell, agent) in read_buffer.agent_cell.iter() {
            let mut current_agent = *agent;
            let point = current_agent.perform_operation(*cell, simulation_hour, &grid, read_buffer, &mut csv_record);

            let agent_option = write_buffer.agent_cell.get(&point);
            match agent_option {
                Some(mut _agent) => {
                    write_buffer.agent_cell.insert(*cell, current_agent);
                },
                _ => { write_buffer.agent_cell.insert(point, current_agent); }
            }
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use geography::point::Point;

    #[test]
    fn should_init() {
        let epidemiology:Epidemiology = Epidemiology::new(20, 10, 1.0, 1.0);
        assert_eq!(epidemiology.grid.housing_area.start_offset, Point::new(0, 0));
        assert_eq!(epidemiology.grid.housing_area.end_offset, Point::new(7, 19));

        assert_eq!(epidemiology.grid.transport_area.start_offset, Point::new(8, 0));
        assert_eq!(epidemiology.grid.transport_area.end_offset, Point::new(9, 19));

        assert_eq!(epidemiology.grid.hospital.start_offset, Point::new(10, 0));
        assert_eq!(epidemiology.grid.hospital.end_offset, Point::new(11, 19));

        assert_eq!(epidemiology.grid.work_area.start_offset, Point::new(12, 0));
        assert_eq!(epidemiology.grid.work_area.end_offset, Point::new(19, 19));

        assert_eq!(epidemiology.agent_location_map.agent_cell.len(), 10);
    }
}