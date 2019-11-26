use hashbrown::HashMap;

use rand::thread_rng;
use rand::seq::SliceRandom;
use rand::Rng;

use std::cmp::max;
use std::cmp::min;

use crate::epidemiology_geography::Point;
use crate::agent;

use std::prelude::v1::Vec;

pub struct AgentLocationMap {
    pub size: i32,
    pub agent_cell: HashMap<Point, agent::Citizen>
}

impl AgentLocationMap {

    pub fn new(size: i32, agent_list: &Vec<agent::Citizen>, points: &Vec<Point>) -> AgentLocationMap {
        let mut map:HashMap<Point, agent::Citizen> = HashMap::new();
        for i in 0..agent_list.len(){
            map.insert(points[i], agent_list[i]);
        }

        AgentLocationMap {size, agent_cell:map}
    }

    pub fn move_agents(&mut self){
        let keys:Vec<Point> = self.agent_cell.keys().cloned().collect();
        for cell in keys {
            let agent = self.get_agent(&cell);
            let neighbor_cells = self.get_neighbor_cells(cell);
            let new_cell: Point = AgentLocationMap::get_random_point_from(self.get_empty_cells_from(neighbor_cells), cell);
            self.agent_cell.remove(&cell);
            self.agent_cell.insert(new_cell, agent);
        }
    }

    fn get_agent(&mut self, cell: &Point) -> agent::Citizen {
        *self.agent_cell.get(&cell).unwrap()
    }

    pub fn update_infections(&mut self) {
        let keys: Vec<Point> = self.agent_cell.keys().cloned().collect();
        for cell in keys {
            if self.get_agent(&cell).infected{
                continue;
            }
            let neighbors = self.get_agents_from(self.get_neighbor_cells(cell));
            let infected_neighbors:Vec<agent::Citizen> = neighbors.into_iter().filter(|agent| agent.infected).collect();
            for neighbor in infected_neighbors {
                let mut rng = thread_rng();
                if rng.gen_bool(neighbor.get_infection_transmission_rate()) {
                    self.agent_cell.get_mut(&cell).unwrap().infected = true;
                }
            }
        }
    }

    pub fn update_infection_day(&mut self) {
        for (_, citizen) in self.agent_cell.iter_mut(){
            if citizen.infected{
                citizen.increment_infection_day()
            }
        }
    }

    pub fn print(&self){
        for (k,v) in self.agent_cell.iter(){
            println!("x:{}, y:{} - id:{} infected:{}", k.x, k.y, v.id, v.infected);
        }
    }

    fn get_neighbor_cells(&self, location: Point) -> Vec<Point>{
        const NUMBER_OF_NEIGHBORS:i32 = 8;
        let mut neighbors_list = Vec::with_capacity(NUMBER_OF_NEIGHBORS as usize);
        let mut row_index = max(0, location.x - 1);

        loop{
            if row_index > min(location.x+1, self.size-1) {
                break;
            }
            let mut col_index = max(0, location.y - 1) ;
            loop{
                if col_index > min(location.y+1, self.size-1) {
                    break;
                }
                if row_index == location.x && col_index == location.y {
                    col_index = col_index + 1;
                    continue;
                }
                neighbors_list.push(Point{x:row_index, y:col_index});
                col_index = col_index + 1;
            }
            row_index = row_index + 1;
        }

        neighbors_list
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

    fn get_random_point_from(vector: Vec<Point>, location: Point) -> Point {
        let choice = vector.choose(&mut rand::thread_rng());
        match choice {
            Some(x) => return *x,
            None => return location
        }
    }
}

#[test]
fn new_allocation_map(){
    let points = vec![Point{x:0, y:1}, Point{x:1, y:0}];
    let agents = vec![agent::Citizen::new_citizen(1, true), agent::Citizen::new_citizen(2, false)];
    let map = AgentLocationMap::new(2, &agents, &points);
    let citizen_option = map.agent_cell.get(&Point{x:0, y:1});
    let mut actual_citizen = agent::Citizen::new();
    match citizen_option{
        Some(x) => {
            actual_citizen = *x;
        }
        _ => {}
    }
    assert_eq!(map.size, 2);
    assert_eq!(actual_citizen.id, 1);
}

#[test]
fn move_agent(){
    let points = vec![Point{x:0, y:1}, Point{x:1, y:0}];
    let agents = vec![agent::Citizen::new_citizen(1, true), agent::Citizen::new_citizen(2, false)];
    let mut map = AgentLocationMap::new(3,&agents, &points);

    map.move_agents();

    let citizen_option = map.agent_cell.get(&Point{x:0, y:1});

    let mut actual_citizen = agent::Citizen::new();

    match citizen_option {
        Some(x) => { actual_citizen = *x},
        None => {}
    }
    
    assert_ne!(actual_citizen.id, 1)

}

#[test]
fn get_empty_cells(){
    let points = vec![Point{x:0, y:1}, Point{x:1, y:0}];
    let agents = vec![agent::Citizen::new_citizen(1, true), agent::Citizen::new_citizen(2, false)];
    let map = AgentLocationMap::new(5,&agents, &points);

    let empty_cells = map.get_empty_cells_from(map.get_neighbor_cells(points[0]));
    assert_eq!(empty_cells.len(), 4);
}

#[test]
fn get_neighbor_agents(){
    let points = vec![Point{x:0, y:1}, Point{x:1, y:0}];
    let agents = vec![agent::Citizen::new_citizen(1, true), agent::Citizen::new_citizen(2, false)];
    let map = AgentLocationMap::new(5,&agents, &points);

    let neighbor_agents= map.get_agents_from(map.get_neighbor_cells(points[0]));
    assert_eq!(neighbor_agents.len(), 1);
}

//TODO: Fix the test by introducing mocking framework
//#[test]
//fn update_infections(){
//    let points = vec![Point{x:0, y:1}, Point{x:1, y:0}];
//    let agents = vec![agent::Citizen::new_citizen(1, true), agent::Citizen::new_citizen(2, false)];
//
//    let mut map = AgentLocationMap::new(3,&agents, &points);
//
//    map.update_infections();
//
//    assert_eq!(map.agent_cell.get(&Point{x:1, y:0}).unwrap().infected, true);
//}

#[test]
fn update_infection_day(){
    let points = vec![Point{x:0, y:1}, Point{x:1, y:0}];
    let agents = vec![agent::Citizen::new_citizen(1, true), agent::Citizen::new_citizen(2, false)];
    let mut map = AgentLocationMap::new(5,&agents, &points);
    assert_eq!(map.agent_cell.get(&Point{x:0, y:1}).unwrap().infection_day, 0);

    map.update_infection_day();
    assert_eq!(map.agent_cell.get(&Point{x:0, y:1}).unwrap().infection_day, 1);
    assert_eq!(map.agent_cell.get(&Point{x:1, y:0}).unwrap().infection_day, 0);
}