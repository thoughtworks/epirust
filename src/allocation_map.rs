use hashbrown::HashMap;

use rand::thread_rng;
use rand::Rng;

use crate::agent;

use std::prelude::v1::Vec;
use crate::geography::point::Point;
use crate::utils;
use crate::geography::Area;

pub struct AgentLocationMap {
    pub grid_size: i32,
    pub agent_cell: HashMap<Point, agent::Citizen>
}

impl AgentLocationMap {

    pub fn new(size: i32, agent_list: &Vec<agent::Citizen>, points: &Vec<Point>) -> AgentLocationMap {
        let mut map:HashMap<Point, agent::Citizen> = HashMap::new();
        for i in 0..agent_list.len(){
            map.insert(points[i], agent_list[i]);
        }

        AgentLocationMap {grid_size: size, agent_cell:map}
    }

    pub fn move_agents(&mut self){
        let keys:Vec<Point> = self.agent_cell.keys().cloned().collect();
        for cell in keys {
            self.move_agent_from(&cell);
        }
    }

    fn move_agent(&mut self, agent: agent::Citizen, old_cell: Point, new_cell: Point){
        if self.agent_cell.contains_key(&new_cell){
            println!("Clash: Returning");
            return;
        }
        self.agent_cell.remove(&old_cell);
        self.agent_cell.insert(new_cell, agent);
    }

    pub fn update_infections(&mut self) {
        let keys: Vec<Point> = self.agent_cell.keys().cloned().collect();
        for cell in keys {
            self.update_infection(&cell)
        }
    }

//TODO: Validate for fallback value and movement
    pub fn goto<T: Area>(&mut self, area: T){
        let keys: Vec<Point> = self.agent_cell.keys().cloned().collect();
        for cell in keys {
            let agent = self.get_agent(&cell);
            let area_dimensions = area.get_dimensions(agent);
            let vacant_cells = self.get_empty_cells_from(area_dimensions);

            self.move_agent(agent, cell, utils::get_random_element_from(&vacant_cells, agent.home_location));
        }
    }

    fn get_agent(&mut self, cell: &Point) -> agent::Citizen {
        *self.agent_cell.get(&cell).unwrap()
    }

    fn move_agent_from(&mut self, cell: &Point) {
        let agent = self.get_agent(&cell);
        if agent.quarantined{
            return;
        }
        let neighbor_cells:Vec<Point> = cell.get_neighbor_cells(self.grid_size);
        let new_cell: Point = utils::get_random_element_from(&self.get_empty_cells_from(neighbor_cells), *cell);
        self.move_agent(agent, *cell, new_cell);
    }

    fn update_infection(&mut self, cell: &Point) -> () {
        if self.get_agent(&cell).infected {
            return;
        }
        let neighbors = self.get_agents_from(cell.get_neighbor_cells(self.grid_size));
        let infected_neighbors: Vec<agent::Citizen> = neighbors.into_iter().filter(|agent| agent.infected).collect();
        for neighbor in infected_neighbors {
            let mut rng = thread_rng();
            if rng.gen_bool(neighbor.get_infection_transmission_rate()) {
                println!("Infection rate {}", neighbor.get_infection_transmission_rate());
                self.agent_cell.get_mut(&cell).unwrap().infected = true;
            }
        }
    }

    pub fn update_infection_day(&mut self) {
        for (_, citizen) in self.agent_cell.iter_mut(){
            if citizen.infected{
                citizen.increment_infection_day();
                citizen.set_quarantined();
            }
        }
    }

    pub fn print(&self){
        for (k,v) in self.agent_cell.iter(){
            println!("x:{}, y:{} - id:{} infected:{}", k.x, k.y, v.id, v.infected);
        }
    }

    fn get_empty_cells_from(&self, neighbors:Vec<Point>) -> Vec<Point>{
        neighbors.into_iter().filter(|key| !self.agent_cell.contains_key(key)).collect()
    }

    fn get_agents_from(&self, neighbors:Vec<Point>) -> Vec<agent::Citizen> {
        let mut agent_list = Vec::new();
        for neighbor in neighbors{
            let agent = self.agent_cell.get(&neighbor);
            match agent {
                Some(x) => agent_list.push(*x),
                _ => {}
            }
        }
        agent_list
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    fn before_each() -> AgentLocationMap {
        let points = vec![Point { x: 0, y: 1 }, Point { x: 1, y: 0 }];
        let agents = vec![agent::Citizen::new_citizen(1, true, points[0], points[1]), agent::Citizen::new_citizen(2, false, points[1], points[0])];
        let map = AgentLocationMap::new(5, &agents, &points);
        map
    }

    #[test]
    fn new(){
        let map = before_each();
        let citizen_option = map.agent_cell.get(&Point{x:0, y:1});
        let mut actual_citizen = agent::Citizen::new();
        match citizen_option{
            Some(x) => {
                actual_citizen = *x;
            }
            _ => {}
        }
        assert_eq!(map.grid_size, 5);
        assert_eq!(actual_citizen.id, 1);
    }

    #[test]
    fn move_agent(){
        let mut map = before_each();

        map.move_agents();

        let citizen_option = map.agent_cell.get(&Point{x:0, y:1});

        let mut actual_citizen = agent::Citizen::new();

        match citizen_option {
            Some(x) => { actual_citizen = *x},
            None => {}
        }

        assert_ne!(actual_citizen.id, 1);
    }

    #[test]
    fn get_empty_cells(){
        let map = before_each();

        let empty_cells = map.get_empty_cells_from(Point{x: 0, y: 1}.get_neighbor_cells(5));
        assert_eq!(empty_cells.len(), 4);
    }

    #[test]
    fn get_neighbor_agents(){
        let map = before_each();

        let neighbor_agents= map.get_agents_from(Point{x: 0, y: 1}.get_neighbor_cells(5));
        assert_eq!(neighbor_agents.len(), 1);
    }

    #[test]
    fn update_infection_day(){
        let mut map = before_each();
        assert_eq!(map.agent_cell.get(&Point{x:0, y:1}).unwrap().infection_day, 0);

        map.update_infection_day();
        assert_eq!(map.agent_cell.get(&Point{x:0, y:1}).unwrap().infection_day, 1);
        assert_eq!(map.agent_cell.get(&Point{x:1, y:0}).unwrap().infection_day, 0);
    }
}