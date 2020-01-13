use crate::agent;
use crate::allocation_map;
use std::time::SystemTime;
use crate::constants;
use crate::geography::point;
use crate::geography::housing_area::HousingArea;
use crate::geography::work_area::WorkArea;
use crate::geography::point::Point;
use crate::csv_service;
use crate::geography::transport_area::TransportArea;
use crate::geography::hospital::Hospital;
use crate::csv_service::Row;
use hashbrown::HashMap;
use core::borrow::BorrowMut;
use crate::allocation_map::AgentLocationMap;
use core::borrow::Borrow;
use rand::thread_rng;
use rand::Rng;

pub struct Epidemiology {
    pub agent_location_map: allocation_map::AgentLocationMap,
    pub write_agent_location_map: allocation_map::AgentLocationMap,
    pub housing_area: HousingArea,
    pub work_area: WorkArea,
    pub transport_area: TransportArea,
    pub hospital: Hospital,
}

impl Epidemiology {

    pub fn new(grid_size: i32, number_of_agents: i32, public_transport_percentage: f64, working_percentage:f64) -> Epidemiology {
        let bound = grid_size - 1;
        let x_offset_for_home = (bound as f32 * constants::HOUSE_AREA_RELATIVE_SIZE).ceil() as i32 - 1;
        let x_offset_for_transport = x_offset_for_home + (bound as f32 * constants::TRANSPORT_AREA_RELATIVE_SIZE).ceil() as i32;
        let x_offset_for_hospital = x_offset_for_transport + (bound as f32 * constants::HOSPITAL_RELATIVE_SIZE).ceil() as i32 ;

        let (housing_area, transport_area, hospital, work_area) = Epidemiology::define_geography(bound, x_offset_for_home, x_offset_for_transport, x_offset_for_hospital);
        let (home_locations, agent_list) = Epidemiology::generate_population(number_of_agents,housing_area, transport_area, public_transport_percentage, working_percentage);

        let agent_location_map = allocation_map::AgentLocationMap::new(grid_size, &agent_list, &home_locations);
        let write_agent_location_map = allocation_map::AgentLocationMap::new(grid_size, &agent_list, &home_locations);
        Epidemiology{agent_location_map, write_agent_location_map, housing_area, work_area, transport_area, hospital}
    }

    fn define_geography(bound: i32, x_offset_for_home: i32, x_offset_for_transport:i32, x_offset_for_hospital: i32) -> (HousingArea, TransportArea, Hospital, WorkArea) {
        let housing_area = HousingArea::new(Point::new(0, 0), Point::new(x_offset_for_home, bound));
        let transport_area = TransportArea::new(Point::new(x_offset_for_home + 1, 0), Point::new(x_offset_for_transport, bound));
        let hospital = Hospital::new(Point::new(x_offset_for_transport+1, 0), Point::new(x_offset_for_hospital, bound));
        let work_area = WorkArea::new(Point::new(x_offset_for_hospital + 1, 0), Point::new(bound, bound));
        (housing_area, transport_area, hospital, work_area)
    }

    fn generate_population(number_of_agents: i32, home_area: HousingArea, transport_area: TransportArea,
                           public_transport_percentage: f64, working_percentage:f64) -> (Vec<Point>, Vec<agent::Citizen>) {

        let home_locations = point::point_factory(home_area.start_offset,
                                                  home_area.end_offset, number_of_agents);
        let scaling_factor = home_area.end_offset.x + transport_area.end_offset.x;
        let work_locations = home_locations.iter()
            .map(|x| *x + point::Point::new(scaling_factor, 0)).collect();
//        TODO: fix the hack
        let number_of_agents_using_public_transport = number_of_agents as f64 * (public_transport_percentage + 0.1) * (working_percentage + 0.1);
        let transport_locations = point::point_factory(transport_area.start_offset, transport_area.end_offset, number_of_agents_using_public_transport as i32);
        let agent_list = agent::citizen_factory(&home_locations, &work_locations, &transport_locations, public_transport_percentage, working_percentage);
        (home_locations, agent_list)
    }

    fn stop_simulation(row: csv_service::Row) -> bool{
        if row.get_infected() == 0 && row.get_quarantined() == 0{
            return true
        }
        false
    }

    pub fn run_based_on_agents(&mut self, simulation_life_time:i32, vaccination_time:i32,
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

            Epidemiology::executor(&mut csv_record, simulation_hour, read_buffer_reference, write_buffer_reference, self.housing_area, &self.hospital, self.transport_area, self.work_area);
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

    fn executor(mut csv_record: &mut Row, simulation_hour: i32, read_buffer: &AgentLocationMap, write_buffer: &mut AgentLocationMap, housing_area: HousingArea, hospital: &Hospital, transport_area: TransportArea, work_area: WorkArea) {
        write_buffer.agent_cell.clear();
        for (cell, agent) in read_buffer.agent_cell.iter() {
            let mut updated_agent = agent.clone();
            let point = updated_agent.perform_operation(*cell, simulation_hour, &housing_area, &hospital,
                                                        &transport_area, &work_area, read_buffer, &mut csv_record);

            let agent_option = write_buffer.agent_cell.get(&point);
            match agent_option {
                Some(mut _agent) => {
                    write_buffer.agent_cell.insert(*cell, updated_agent);
                },
                _ => { write_buffer.agent_cell.insert(point, updated_agent); }
            }
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn should_init() {
        let epidemiology:Epidemiology = Epidemiology::new(20, 10, 1.0, 1.0);
        assert_eq!(epidemiology.housing_area.start_offset, Point::new(0, 0));
        assert_eq!(epidemiology.housing_area.end_offset, Point::new(7, 19));

        assert_eq!(epidemiology.transport_area.start_offset, Point::new(8, 0));
        assert_eq!(epidemiology.transport_area.end_offset, Point::new(9, 19));

        assert_eq!(epidemiology.hospital.start_offset, Point::new(10, 0));
        assert_eq!(epidemiology.hospital.end_offset, Point::new(11, 19));

        assert_eq!(epidemiology.work_area.start_offset, Point::new(12, 0));
        assert_eq!(epidemiology.work_area.end_offset, Point::new(19, 19));

        assert_eq!(epidemiology.agent_location_map.agent_cell.len(), 10);
    }
}