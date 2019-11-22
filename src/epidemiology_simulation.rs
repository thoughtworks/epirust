use crate::agent;
use crate::allocation_map;
use crate::epidemiology_geography;
use std::time::SystemTime;

pub struct Epidemiology {
    pub agent_list: Vec<agent::Citizen>,
    pub agent_location_map: allocation_map::AgentLocationMap
}

const NUMBER_OF_HOURS:i32 = 24;

impl Epidemiology {

    pub fn new(grid_size: i32, number_of_agents: i32) -> Epidemiology {
        let agent_list = Epidemiology::create_citizen(number_of_agents);
        let points = epidemiology_geography::point_factory(grid_size, number_of_agents);
        let agent_location_map = allocation_map::AgentLocationMap::new(grid_size, &agent_list, &points);

        Epidemiology{agent_list, agent_location_map}
    }

    pub fn run(&mut self, simulation_life_time:i32) {
        for i in 1..simulation_life_time {
            let start_time = SystemTime::now();
            if i % NUMBER_OF_HOURS == 0{
                self.agent_location_map.update_infection_day();
            }
            self.agent_location_map.move_agents();
            self.agent_location_map.update_infections();
            let end_time = SystemTime::now();
//            self.agent_location_map.print();
            println!("Tick {}, Time taken {:?}", i, end_time.duration_since(start_time));
        }
    }

//    TODO: Pull it into factory
    fn create_citizen(number_of_agents: i32) -> Vec<agent::Citizen>{
        let mut agent_list = Vec::with_capacity(number_of_agents as usize);

        for agent_id in 0..number_of_agents-1 {
            let agent = agent::Citizen::new_citizen(agent_id, false);
            agent_list.push(agent);
        }
        agent_list.push(agent::Citizen::new_citizen(number_of_agents-1, true));
        agent_list
    }
}

#[test]
fn init() {
    let epidemiology:Epidemiology = Epidemiology::new(3, 5);

    assert_eq!(epidemiology.agent_list.len(), 5);
}