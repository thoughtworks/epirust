/*
 * EpiRust
 * Copyright (c) 2020  ThoughtWorks, Inc.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 */


use crate::agent;
use crate::geography::{Area, Grid};
use crate::geography::Point;
use crate::random_wrapper::RandomWrapper;
use crate::agent::Citizen;
use crate::disease_state_machine::State;
use crate::listeners::events::counts::Counts;
use crate::travel_plan::Traveller;
use std::collections::hash_map::{IterMut, Iter};
use fnv::FnvHashMap;

#[derive(Clone)]
pub struct AgentLocationMap {
    grid_size: i32,
    agent_cell: FnvHashMap<Point, agent::Citizen>,
}

impl AgentLocationMap {
    pub fn init_with_capacity(&mut self, size: usize) {
        self.agent_cell = FnvHashMap::with_capacity_and_hasher(size, Default::default());
    }

    pub fn new(grid_size: i32, agent_list: &[agent::Citizen], points: &[Point]) -> AgentLocationMap {
        debug!("{} agents and {} starting points", agent_list.len(), points.len());
        let mut map: FnvHashMap<Point, agent::Citizen> = FnvHashMap::with_capacity_and_hasher(agent_list.len(), Default::default());
        for i in 0..agent_list.len() {
            map.insert(points[i], agent_list[i]);
        }

        AgentLocationMap { grid_size, agent_cell: map }
    }

    pub fn move_agent(&self, old_cell: Point, new_cell: Point) -> Point {
        if self.is_cell_vacant(&new_cell) {
            return new_cell;
        }
        old_cell
    }

    pub fn goto_hospital(&self, hospital_area: &Area, cell: Point, citizen: &mut agent::Citizen) -> (bool, Point) {
        let vacant_hospital_cell = hospital_area.iter().find(|cell| {
            self.is_cell_vacant(cell)
        });
        match vacant_hospital_cell {
            Some(x) => (true, self.move_agent(cell, x)),
            None => {
                (false,
                 self.move_agent(cell, citizen.home_location.get_random_point(&mut RandomWrapper::new())))
            }
        }
    }

//    pub fn print(&self){
//        for (k,v) in self.agent_cell.iter(){
//            println!("x:{}, y:{} - id:{} infected:{} working:{} Transport:{}", k.x, k.y, v.id, v.is_infected(), v.working, v.uses_public_transport);
//        }
//    }

    pub fn get_agent_for(&self, cell: &Point) -> Option<&agent::Citizen> {
        self.agent_cell.get(cell)
    }

    pub fn is_point_in_grid(&self, point: &Point) -> bool {
        point.x >= 0 && point.y >= 0 && point.x < self.grid_size && point.y < self.grid_size
    }

    pub fn is_cell_vacant(&self, cell: &Point) -> bool {
        !self.agent_cell.contains_key(cell)
    }

    pub fn remove_citizens(&mut self, outgoing: &Vec<(Point, Traveller)>, counts: &mut Counts, grid: &mut Grid) {
        if outgoing.is_empty() {
            return;
        }
        debug!("Removing {} outgoing travellers", outgoing.len());
        for (point, traveller) in outgoing {
            match traveller.state_machine.state {
                State::Susceptible { .. } => { counts.update_susceptible(-1) },
                State::Exposed { .. } => { counts.update_exposed(-1) }
                State::Infected { .. } => { counts.update_infected(-1) },
                State::Recovered { .. } => { counts.update_recovered(-1) },
                State::Deceased { .. } => { panic!("Deceased agent should not travel!") },
            }
            match self.agent_cell.remove(point) {
                None => {
                    panic!("Trying to remove citizen {:?} from location {:?}, but no citizen is present at this location!",
                           traveller.id, point)
                }
                Some(citizen) => {
                    grid.remove_house_occupant(&citizen.home_location);
                    if citizen.is_working() {
                        grid.remove_office_occupant(&citizen.work_location);
                    }
                }
            }
        }
    }

    pub fn assimilate_citizens(&mut self, incoming: &mut Vec<Traveller>, grid: &mut Grid, counts: &mut Counts,
                               rng: &mut RandomWrapper) {
        if incoming.is_empty() {
            return;
        }
        debug!("Assimilating {} incoming travellers", incoming.len());
        let mut new_citizens: Vec<Citizen> = Vec::with_capacity(incoming.len());
        for traveller in incoming {
            let house = grid.choose_house_with_free_space(rng);
            let office = if traveller.working {
                grid.choose_office_with_free_space(rng)
            } else {
                house
            };
            let transport_location = house.get_random_point(rng); // Fixme
            new_citizens.push(Citizen::from_traveller(traveller, house, office, transport_location, grid.housing_area));
            grid.add_house_occupant(&house);
            if traveller.working {
                grid.add_office_occupant(&office)
            }
        }
        for c in new_citizens {
            match c.state_machine.state {
                State::Susceptible { .. } => { counts.update_susceptible(1) }
                State::Exposed { .. } => { counts.update_exposed(1) }
                State::Infected { .. } => { counts.update_infected(1) }
                State::Recovered { .. } => { counts.update_recovered(1) }
                State::Deceased { .. } => { panic!("Should not receive deceased agent!") }
            }
            let p = self.random_starting_point(&grid.housing_area, rng);
            let result = self.agent_cell.insert(p, c);
            assert!(result.is_none());
        }
    }

    fn random_starting_point(&self, area: &Area, rng: &mut RandomWrapper) -> Point {
        loop {
            let point = area.get_random_point(rng);
            if !self.agent_cell.contains_key(&point) {
                return point
            }
        }
    }

    pub fn current_population(&self) -> i32 {
        self.agent_cell.len() as i32
    }

    pub fn iter(&self) -> Iter<'_, Point, Citizen> {
        self.agent_cell.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Point, Citizen> {
        self.agent_cell.iter_mut()
    }

    pub fn clear(&mut self) {
        self.agent_cell.clear();
    }

    pub fn get(&self, point: &Point) -> Option<&Citizen> {
        self.agent_cell.get(point)
    }

    pub fn insert(&mut self, point: Point, citizen: Citizen) -> Option<Citizen> {
        self.agent_cell.insert(point, citizen)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::random_wrapper::RandomWrapper;
    use crate::agent::WorkStatus;

    fn before_each() -> AgentLocationMap {
        let mut rng = RandomWrapper::new();
        let points = vec![Point { x: 0, y: 1 }, Point { x: 1, y: 0 }];
        let home_locations = vec![Area::new(Point::new(0, 0), Point::new(2, 2)), Area::new(Point::new(3, 0), Point::new(4, 2))];

        let work_locations = vec![Area::new(Point::new(5, 0), Point::new(6, 2)), Area::new(Point::new(7, 0), Point::new(8, 2))];
        let work_status = WorkStatus::NA {};

        let agents = vec![agent::Citizen::new(home_locations[0], work_locations[0], points[0], false, false, work_status, &mut rng),
                          agent::Citizen::new(home_locations[1], work_locations[0], points[0], true, true, work_status, &mut rng)];
        AgentLocationMap::new(5, &agents, &points)
    }

    #[test]
    fn new() {
        let map = before_each();

        assert_eq!(map.grid_size, 5);
    }

    #[test]
    fn should_goto_hospital() {
        let mut rng = RandomWrapper::new();
        let points = vec![Point { x: 0, y: 1 }, Point { x: 1, y: 0 }];
        let work_status = WorkStatus::Normal {};
        let home_locations = vec![Area::new(Point::new(0, 0), Point::new(2, 2)), Area::new(Point::new(3, 0), Point::new(4, 2))];

        let work_locations = vec![Area::new(Point::new(5, 0), Point::new(6, 2)), Area::new(Point::new(7, 0), Point::new(8, 2))];
        let mut citizen1 = agent::Citizen::new(home_locations[0], work_locations[1], points[0], false, false, work_status, &mut rng);
        let citizen2 = agent::Citizen::new(home_locations[1], work_locations[0], points[0], true, true, work_status, &mut rng);
        let agents = vec![citizen1, citizen2];
        let map = AgentLocationMap::new(5, &agents, &points);
        let hospital = Area::new(Point::new(2, 2), Point::new(4, 4));
        let result = map.goto_hospital(&hospital, points[0], &mut citizen1);

        assert!(result.0);
        assert_eq!(result.1, Point::new(2, 2));
    }

    #[test]
    fn should_goto_home_location_when_hospital_full() {
        let mut rng = RandomWrapper::new();
        let points = vec![Point::new(0, 0), Point::new(0, 1), Point::new(1, 0), Point::new(1, 1)];
        let home = Area::new(Point::new(0, 0), Point::new(2, 2));
        let work_status = WorkStatus::NA {};

        let work = Area::new(Point::new(5, 0), Point::new(6, 2));
        let mut citizen1 = agent::Citizen::new(home, work, points[0], false, false, work_status, &mut rng);
        let citizen2 = agent::Citizen::new(home, work, points[0], false, false, work_status, &mut rng);
        let citizen3 = agent::Citizen::new(home, work, points[0], false, false, work_status, &mut rng);
        let citizen4 = agent::Citizen::new(home, work, points[0], false, false, work_status, &mut rng);
        let agents = vec![citizen1, citizen2, citizen3, citizen4];
        let map = AgentLocationMap::new(5, &agents, &points);
        let hospital = Area::new(Point::new(0, 0), Point::new(1, 1));

        let result = map.goto_hospital(&hospital, points[0], &mut citizen1);

        assert!(!result.0);
        assert!(citizen1.home_location.contains(&result.1));
    }

    #[test]
    fn should_return_true_when_point_is_in_grid() {
        let map = before_each();
        let points = vec![Point::new(0, 0), Point::new(4, 4), Point::new(2, 2)];
        for point in points {
            assert!(map.is_point_in_grid(&point))
        }
    }

    #[test]
    fn should_return_false_when_point_is_out_of_grid() {
        let map = before_each();
        let points = vec![Point::new(-1, -1), Point::new(5, 5), Point::new(2, 12)];
        for point in points {
            assert!(!map.is_point_in_grid(&point))
        }
    }
}
