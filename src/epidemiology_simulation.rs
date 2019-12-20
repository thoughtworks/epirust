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

pub struct Epidemiology {
    pub agent_location_map: allocation_map::AgentLocationMap,
    pub housing_area: HousingArea,
    pub work_area: WorkArea,
    pub transport_area: TransportArea,
    pub hospital: Hospital
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
        Epidemiology{agent_location_map, housing_area, work_area, transport_area, hospital}
    }

    pub fn run(&mut self, simulation_life_time:i32, vaccination_time:i32, vaccination_percentage:f64, output_file_name: &str) {
        let mut records: Vec<csv_service::Row> = Vec::new();
//        println!("Tick 0");
        let mut count = 0;

        let start_time = SystemTime::now();
        self.agent_location_map.goto(self.housing_area);
        for i in 1..simulation_life_time {
            self.agent_location_map.counts.increment_hour();

            self.routine(i);

            if i == vaccination_time{
                self.agent_location_map.vaccinate(vaccination_percentage);
            }

            let row = self.agent_location_map.get_record();
            records.push(row);

            if Epidemiology::stop_simulation(row){
                break;
            }
            count = i;
//            self.agent_location_map.print();
        }
        let end_time = SystemTime::now();
        println!("Number of iterations: {}, Total Time taken {:?}", count, end_time.duration_since(start_time));
        let result = csv_service::write(output_file_name, &records);
        match result{
            Ok(_) => {},
            Err(e) => {println!("Error occurred while writing data to csv {:?}", e)}
        }
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

    fn routine(&mut self, i: i32) {
        match i % constants::NUMBER_OF_HOURS {
            constants::ROUTINE_START_TIME => {
                self.agent_location_map.update_infection_day();
                self.agent_location_map.quarantine(self.hospital);
            },
            constants::SLEEP_START_TIME..=constants::SLEEP_END_TIME => (),
            constants::ROUTINE_TRAVEL_START_TIME => {
                self.agent_location_map.goto(self.transport_area);
                self.agent_location_map.update_infections();
            },
            constants::ROUTINE_WORK_TIME => {
                self.agent_location_map.goto(self.work_area);
                self.agent_location_map.update_infections();
            },
            constants::ROUTINE_TRAVEL_END_TIME => {
                self.agent_location_map.goto(self.transport_area);
                self.agent_location_map.update_infections();
            },
            constants::ROUTINE_WORK_END_TIME => {
                self.agent_location_map.goto(self.housing_area);
                self.agent_location_map.update_infections();
            },
            constants::ROUTINE_END_TIME => self.agent_location_map.deceased(),
            _ => {
                self.agent_location_map.move_agents();
                self.agent_location_map.update_infections();
            }
        }
    }

    fn stop_simulation(row: csv_service::Row) -> bool{
        if row.get_infected() == 0 && row.get_quarantined() == 0{
            return true
        }
        false
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