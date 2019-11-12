use crate::geography;
use crate::agent;
use std::collections::HashMap;

pub struct Epidemiology {
    pub grid: geography::Grid,
    pub agent_list: Vec<agent::Citizen>,
    pub agent_location_map: HashMap<geography::Point, agent::Citizen>
}

impl Epidemiology {
    pub fn new(grid_size: i32, number_of_agents: i32) -> Epidemiology {
        let mut grid = geography::Grid::new(grid_size);
        let agent_list = Epidemiology::create_citizen(number_of_agents, grid.get_empty_cells());
        Epidemiology::allocate_agents(&mut grid, &agent_list);
        let agent_location_map: HashMap<geography::Point, agent::Citizen> = Epidemiology::populate_agent_location_map(&agent_list);

        Epidemiology{grid, agent_list, agent_location_map}
    }

    pub fn run(&mut self, simulation_life_time:i32) {
        for _ in 1..simulation_life_time {
            self.move_agents();
        }
    }

//    TODO: Pull it into factory
    fn create_citizen(number_of_agents: i32, cells: Vec<geography::Point>) -> Vec<agent::Citizen>{
        let mut agent_list = Vec::with_capacity(number_of_agents as usize);
        for agent_id in 0..number_of_agents {
            let mut point = cells[agent_id as usize];
            point.set_occupancy(true);
            let agent = agent::Citizen::new_citizen(agent_id, point);

            agent_list.push(agent);
        }
        let mut point = cells[number_of_agents as usize];
        point.set_occupancy(true);

        agent_list.push(agent::Citizen::new_citizen(number_of_agents, point));
        agent_list
    }

    fn allocate_agents(grid: &mut geography::Grid, agents: &Vec<agent::Citizen>){
        let points:Vec<geography::Point> = agents.iter().map(|agent| agent.location).collect();
        grid.update_points_occupancy(points);
    }

    //TODO: Validate move agent is working correctly
    fn move_agents(&mut self) -> () {
        for i in 0..self.agent_list.len() {
//            TODO: Change loop to iterate over agent_location_map
            let mut old_location = self.agent_list[i].location;
            let mut new_location = self.grid.get_empty_neighbor_point(old_location);

            if new_location.x != old_location.x && new_location.y != old_location.y {
                old_location.set_occupancy(false);
                new_location.set_occupancy(true);
                self.grid.cells[old_location.x as usize][old_location.y as usize].occupied = false;
                self.grid.cells[new_location.x as usize][new_location.y as usize].occupied = true;
                self.agent_list[i].set_location(new_location);
                self.agent_location_map.insert(new_location, self.agent_list[i]);
                self.agent_location_map.insert(old_location, agent::Citizen::new());
            }
        }

        self.print();
    }

    fn populate_agent_location_map(agent_list:&Vec<agent::Citizen>) -> HashMap<geography::Point, agent::Citizen>{
        let mut agent_location_map:HashMap<geography::Point, agent::Citizen> = HashMap::new();
        for i in 0..agent_list.len() {
            agent_location_map.insert(agent_list[i].location, agent_list[i]);
        }
        agent_location_map
    }

    fn print_map(&mut self, location: geography::Point) {
        let option = self.agent_location_map.get(&location);
        match option {
            Some(x) => {
                print!("{} | ", x.id);
            },
            _ => print!("")
        }
    }

    fn print(&mut self) {
        for row in 0..self.grid.size {
            println!("{}", str::repeat("----", self.grid.size as usize));
            for column in 0..self.grid.size {
                self.print_map(self.grid.cells[row as usize][column as usize]);
            }
            println!();
        }
        println!("{}", str::repeat("----", self.grid.size as usize));
        println!();
    }
}

#[test]
fn init() {
    let epidemiology:Epidemiology = Epidemiology::new(3, 5);

    assert_eq!(epidemiology.agent_list.len(), 6);
    assert_eq!(epidemiology.agent_location_map.len(), 6);
}

#[test]
fn move_agents() {
    let mut epidemiology:Epidemiology = Epidemiology::new(3, 5);

    epidemiology.move_agents();

    assert_eq!(epidemiology.agent_list.len(), 6);
}