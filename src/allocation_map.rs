use std::collections::HashMap;

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
            let mut agent: agent::Citizen = agent::Citizen::new();
            let agent_option = self.agent_cell.get(&cell);
            match agent_option {
                Some(x) => agent = *x,
                _ => { return }
            }
            let neighbor_cells = self.get_neighbor_cells(cell);
            let new_cell: Point = AgentLocationMap::get_random_point_from(self.get_empty_cells_from(neighbor_cells), cell);
            self.agent_cell.remove(&cell);
            self.agent_cell.insert(new_cell, agent);
        }
    }

    pub fn update_infections(&mut self, disease_infection_probability:f64) {
        let keys: Vec<Point> = self.agent_cell.keys().cloned().collect();
        for cell in keys {
            let neighbors = self.get_agents_from(self.get_neighbor_cells(cell));
            let infected_neighbors:Vec<agent::Citizen> = neighbors.into_iter().filter(|agent| agent.infected).collect();
            for _ in infected_neighbors {
                let mut rng = thread_rng();
                if rng.gen_bool(disease_infection_probability) {
                    self.agent_cell.get_mut(&cell).unwrap().infected = true;
                }
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
    let agents = vec![agent::Citizen{id:1, infected: true}, agent::Citizen{id:2, infected: false}];
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
    let agents = vec![agent::Citizen{id:1, infected: true}, agent::Citizen{id:2, infected: false}];
    let mut map = AgentLocationMap::new(5,&agents, &points);

    map.move_agents();

    let citizen_option = map.agent_cell.get(&Point{x:0, y:1});

    let mut citizen_moved = false;
    match citizen_option{
        Some(_) => {}
        _ => { citizen_moved = true}
    }

    assert_eq!(citizen_moved, true);
}

#[test]
fn get_empty_cells(){
    let points = vec![Point{x:0, y:1}, Point{x:1, y:0}];
    let agents = vec![agent::Citizen{id:1, infected: true}, agent::Citizen{id:2, infected: false}];
    let map = AgentLocationMap::new(5,&agents, &points);

    let empty_cells = map.get_empty_cells_from(map.get_neighbor_cells(points[0]));
    assert_eq!(empty_cells.len(), 4);
}

#[test]
fn get_neighbor_agents(){
    let points = vec![Point{x:0, y:1}, Point{x:1, y:0}];
    let agents = vec![agent::Citizen{id:1, infected: true}, agent::Citizen{id:2, infected: false}];
    let map = AgentLocationMap::new(5,&agents, &points);

    let neighbor_agents= map.get_agents_from(map.get_neighbor_cells(points[0]));
    assert_eq!(neighbor_agents.len(), 1);
}

#[test]
fn update_infections(){
    let points = vec![Point{x:0, y:1}, Point{x:1, y:0}];
    let agents = vec![agent::Citizen{id:1, infected: true}, agent::Citizen{id:2, infected: false}];
    let mut map = AgentLocationMap::new(5,&agents, &points);

    map.update_infections(1.0);

    assert_eq!(map.agent_cell.get(&Point{x:1, y:0}).unwrap().infected, true);
}