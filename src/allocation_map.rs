use hashbrown::HashMap;

use rand::thread_rng;
use rand::Rng;

use crate::agent;

use std::prelude::v1::Vec;
use crate::geography::point::Point;
use crate::utils;
use crate::geography::Area;
use crate::csv_service::Row;
use crate::geography::transport_area::TransportArea;
use crate::geography::hospital::Hospital;

pub struct AgentLocationMap {
    pub grid_size: i32,
    pub agent_cell: HashMap<Point, agent::Citizen>,
    pub counts: Row
}

impl AgentLocationMap {

    pub fn new(size: i32, agent_list: &Vec<agent::Citizen>, points: &Vec<Point>) -> AgentLocationMap {
        let mut map:HashMap<Point, agent::Citizen> = HashMap::new();
        for i in 0..agent_list.len(){
            map.insert(points[i], agent_list[i]);
        }
        let row = Row::new(agent_list.len() as i32);

        AgentLocationMap {grid_size: size, agent_cell:map, counts: row }
    }
//TODO: Pull out the common code for movement
    pub fn commute(&mut self, transport_area:TransportArea){
        let keys: Vec<Point> = self.agent_cell.keys().cloned().collect();
        for cell in keys {
            let agent = self.get_agent(&cell);
            if !agent.can_move(){
                continue;
            }
            if agent.uses_public_transport && agent.working{
                let area_dimensions = transport_area.get_dimensions(agent);
                let vacant_cells = self.get_empty_cells_from(area_dimensions);
                self.move_agent(agent, cell, utils::get_random_element_from(&vacant_cells, agent.home_location));
                continue;
            }
            self.move_agent_from(&cell);
        }
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

    pub fn update_infections(&mut self){
        let keys: Vec<Point> = self.agent_cell.keys().cloned().collect();
        for cell in keys {
            self.update_infection(&cell);
        }
    }

//TODO: Validate for fallback value and movement
    pub fn goto<T: Area>(&mut self, area: T){
        let keys: Vec<Point> = self.agent_cell.keys().cloned().collect();
        for cell in keys {
            let agent = self.get_agent(&cell);
            if !agent.can_move(){
                continue;
            }
            if agent.working{
                let area_dimensions = area.get_dimensions(agent);
                let vacant_cells = self.get_empty_cells_from(area_dimensions);
                self.move_agent(agent, cell, utils::get_random_element_from(&vacant_cells, agent.home_location));
                continue;
            }
            self.move_agent_from(&cell);
        }
    }

    pub fn vaccinate(&mut self, percentage: f64){
        let mut rng = thread_rng();
        println!("vaccination");
        for(_, agent) in self.agent_cell.iter_mut(){
            if !agent.is_infected() && rng.gen_bool(percentage){
                agent.set_vaccination(true);
                println!("Agent {} is vaccinated", agent.id);
            }
        };
    }

    pub fn get_record(&self) -> Row{
        return self.counts;
    }

    fn get_agent(&mut self, cell: &Point) -> agent::Citizen {
        *self.agent_cell.get(&cell).unwrap()
    }

    fn move_agent_from(&mut self, cell: &Point) {
        let agent = self.get_agent(&cell);
        if !agent.can_move(){
            return;
        }
        let neighbor_cells:Vec<Point> = cell.get_neighbor_cells(self.grid_size);
        let new_cell: Point = utils::get_random_element_from(&self.get_empty_cells_from(neighbor_cells), *cell);
        self.move_agent(agent, *cell, new_cell);
    }

    fn update_infection(&mut self, cell: &Point) {
        if self.get_agent(&cell).is_susceptible() && !self.get_agent(&cell).vaccinated {
            let neighbors = self.get_agents_from(cell.get_neighbor_cells(self.grid_size));
            let infected_neighbors: Vec<agent::Citizen> = neighbors.into_iter().
                filter(|agent| (agent.is_infected() || agent.is_quarantined()) && !agent.hospitalized).collect();
            for neighbor in infected_neighbors {
                let mut rng = thread_rng();
                if rng.gen_bool(neighbor.get_infection_transmission_rate()) {
                    println!("Infection rate {}", neighbor.get_infection_transmission_rate());
                    let infected = self.agent_cell.get_mut(&cell).unwrap().infect();
                    self.counts.update_infected(infected);
                    self.counts.update_susceptible(-infected);
                    return;
                }
            }
        }
    }

    pub fn update_infection_day(&mut self) {
        for (_, citizen) in self.agent_cell.iter_mut(){
            if citizen.is_infected() || citizen.is_quarantined(){
                citizen.increment_infection_day();
            }
        }
    }

    pub fn quarantine(&mut self, area: Hospital) {
        let keys: Vec<Point> = self.agent_cell.keys().cloned().collect();
        for cell in keys {
            let mut citizen = self.get_agent(&cell);
            if citizen.is_infected() && !citizen.is_quarantined(){
                let quarantined = citizen.quarantine();
                self.goto_hospital(area, &cell, &mut citizen);
                citizen.hospitalized = true;
                self.counts.update_quarantined(quarantined);
                self.counts.update_infected(-quarantined);
            }
        }
    }

    fn goto_hospital(&mut self, area: Hospital, cell: &Point, citizen: &mut agent::Citizen) {
        let area_dimensions = area.get_dimensions(*citizen);
        let vacant_cells = self.get_empty_cells_from(area_dimensions);
        self.move_agent(*citizen, *cell, utils::get_random_element_from(&vacant_cells, citizen.home_location));
    }

    pub fn deceased(&mut self) {
        let keys: Vec<Point> = self.agent_cell.keys().cloned().collect();
        for cell in keys {
            let mut citizen = self.get_agent(&cell);
            if citizen.is_quarantined(){
                let result = citizen.decease();
                if result.0 == 1 || result.1 == 1{
                    self.move_agent(citizen, cell, citizen.home_location);
                }
                self.counts.update_deceased(result.0);
                self.counts.update_recovered(result.1);
                self.counts.update_quarantined((result.0 + result.1)* -1);
            }
        }
    }

    pub fn print(&self){
        for (k,v) in self.agent_cell.iter(){
            println!("x:{}, y:{} - id:{} infected:{}", k.x, k.y, v.id, v.is_infected());
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
        let agents = vec![agent::Citizen::new_citizen(1, points[0], points[1], false, false), agent::Citizen::new_citizen(2, points[1], points[0], true, true)];
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

//    #[test]
//    fn update_infection_day(){
//        let mut map = before_each();
//        assert_eq!(map.agent_cell.get(&Point{x:0, y:1}).unwrap().infection_day, 0);
//
//        map.update_infection_day();
//        assert_eq!(map.agent_cell.get(&Point{x:0, y:1}).unwrap().infection_day, 1);
//        assert_eq!(map.agent_cell.get(&Point{x:1, y:0}).unwrap().infection_day, 0);
//    }

//    #[test]
//    fn vaccinate(){
//        let mut map = before_each();
//
//        map.vaccinate(1.0);
//
//        assert_eq!(map.agent_cell.get(&Point { x: 0, y: 1 }).unwrap().vaccinated, false);
//        assert_eq!(map.agent_cell.get(&Point { x: 1, y: 0 }).unwrap().vaccinated, true);
//    }
}