use crate::agent;
use crate::allocation_map;
use std::time::SystemTime;
use crate::constants;
use crate::geography::point;
use crate::geography::housing_area::HousingArea;
use crate::geography::work_area::WorkArea;
use crate::geography::point::Point;

pub struct Epidemiology {
    pub agent_list: Vec<agent::Citizen>,
    pub agent_location_map: allocation_map::AgentLocationMap,
    pub housing_area: HousingArea,
    pub work_area: WorkArea
}

impl Epidemiology {

    pub fn new(grid_size: i32, number_of_agents: i32) -> Epidemiology {
//        TODO: Change allocation strategy
        let x_offset_for_home = grid_size / constants::HOUSE_AREA_RELATED_SIZE;
        let home_locations = point::point_factory(Point{x:0, y:0}, Point{x: x_offset_for_home, y: grid_size}, number_of_agents);
        let work_locations = home_locations.iter().map(|x| *x + point::Point::new(x_offset_for_home, 0))
            .collect();
        let agent_list = agent::citizen_factory(&home_locations, &work_locations);

        let housing_area:HousingArea = HousingArea::new(Point::new(0, 0), Point::new(x_offset_for_home, grid_size));
        let work_area:WorkArea = WorkArea::new(Point::new(x_offset_for_home+1, 0), Point::new(grid_size, grid_size));

        let agent_location_map = allocation_map::AgentLocationMap::new(grid_size, &agent_list, &home_locations);
        Epidemiology{agent_list, agent_location_map, housing_area, work_area}
    }

    pub fn run(&mut self, simulation_life_time:i32) {
        println!("Tick 0");
        self.agent_location_map.goto(self.housing_area);
        for i in 1..simulation_life_time {
            let start_time = SystemTime::now();
            match i% constants::NUMBER_OF_HOURS {
                constants::ROUTINE_START_TIME => {
                    self.agent_location_map.update_infections();
                    self.agent_location_map.update_infection_day();
                },
                constants::ROUTINE_WORK_TIME => self.agent_location_map.goto(self.work_area),
                constants::ROUTINE_WORK_END_TIME => self.agent_location_map.goto(self.housing_area),
                _ => self.agent_location_map.move_agents()
            }

            let end_time = SystemTime::now();
            println!("Tick {}, Time taken {:?}", i, end_time.duration_since(start_time));
//            self.agent_location_map.print();
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn init() {
        let epidemiology:Epidemiology = Epidemiology::new(3, 3);

        assert_eq!(epidemiology.agent_list.len(), 3);
    }
}