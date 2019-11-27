use crate::agent;
use crate::allocation_map;
use crate::epidemiology_geography;
use std::time::SystemTime;
use epidemiology_geography::Point;
use epidemiology_geography::HousingArea;
use crate::constants;

pub struct Epidemiology {
    pub agent_list: Vec<agent::Citizen>,
    pub agent_location_map: allocation_map::AgentLocationMap,
    pub housing_area: HousingArea
}

impl Epidemiology {

    pub fn new(grid_size: i32, number_of_agents: i32) -> Epidemiology {
        let points = epidemiology_geography::point_factory(grid_size, number_of_agents);
        let housing_area:HousingArea = epidemiology_geography::HousingArea::new(grid_size);
        let agent_list = agent::citizen_factory(&points);
        let agent_location_map = allocation_map::AgentLocationMap::new(grid_size, &agent_list, &points);

        Epidemiology{agent_list, agent_location_map, housing_area}
    }

    pub fn run(&mut self, simulation_life_time:i32) {
        println!("Tick 0");
//        self.agent_location_map.print();
        for i in 1..simulation_life_time {
            let start_time = SystemTime::now();
            if i % constants::NUMBER_OF_HOURS == 0{
                self.agent_location_map.update_infection_day();
                self.agent_location_map.goto_home(self.housing_area);
            }
            else{
                self.agent_location_map.move_agents();
            }
            self.agent_location_map.update_infections();
            let end_time = SystemTime::now();
            println!("Tick {}, Time taken {:?}", i, end_time.duration_since(start_time));
//            self.agent_location_map.print();
        }
    }
}

#[test]
fn init() {
    let epidemiology:Epidemiology = Epidemiology::new(3, 3);

    assert_eq!(epidemiology.agent_list.len(), 3);
}