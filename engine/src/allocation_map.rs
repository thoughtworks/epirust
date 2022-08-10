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

use std::collections::hash_map::{Iter, IterMut};

use fnv::FnvHashMap;
use rand::seq::IteratorRandom;

use crate::agent::Citizen;
use crate::commute::Commuter;
use crate::constants;
use crate::custom_types::{CoOrdinate, Count, Hour, Size};
use crate::disease_state_machine::State;
use crate::geography::{Area, Grid};
use crate::geography::Point;
use crate::listeners::events::counts::Counts;
use crate::random_wrapper::RandomWrapper;
use crate::travel_plan::Migrator;

#[derive(Clone)]
pub struct AgentLocationMap {
    grid_size: Size,
    agent_cells: FnvHashMap<Point, Citizen>,
}

impl AgentLocationMap {
    pub fn init_with_capacity(&mut self, size: usize) {
        self.agent_cells = FnvHashMap::with_capacity_and_hasher(size, Default::default());
    }

    pub fn new(grid_size: Size, agent_list: &[Citizen], points: &[Point]) -> AgentLocationMap {
        debug!("{} agents and {} starting points", agent_list.len(), points.len());
        let mut map: FnvHashMap<Point, Citizen> = FnvHashMap::with_capacity_and_hasher(agent_list.len(), Default::default());
        agent_list.iter().enumerate().for_each(|(i, _)| {
            map.insert(points[i], agent_list[i].clone());
        });

        AgentLocationMap { grid_size, agent_cells: map }
    }

    pub fn move_agent(&self, old_cell: Point, new_cell: Point) -> Point {
        return if self.is_cell_vacant(&new_cell) { new_cell } else { old_cell };
    }

    pub fn goto_hospital(&self, hospital_area: &Area, cell: Point, citizen: &mut Citizen) -> (bool, Point) {
        let vacant_hospital_cell = hospital_area.iter().find(|cell| self.is_cell_vacant(cell));
        match vacant_hospital_cell {
            Some(x) => (true, self.move_agent(cell, x)),
            None => (false, self.move_agent(cell, citizen.home_location.get_random_point(&mut RandomWrapper::new()))),
        }
    }

    pub fn get_agent_for(&self, cell: &Point) -> Option<&Citizen> {
        self.agent_cells.get(cell)
    }

    pub fn is_point_in_grid(&self, point: &Point) -> bool {
        let end_coordinate_of_grid = self.grid_size as CoOrdinate;
        point.x >= 0 && point.y >= 0 && point.x < end_coordinate_of_grid && point.y < end_coordinate_of_grid
    }

    pub fn is_cell_vacant(&self, cell: &Point) -> bool {
        !self.agent_cells.contains_key(cell)
    }

    pub fn remove_migrators(&mut self, outgoing: &Vec<(Point, Migrator)>, counts: &mut Counts, grid: &mut Grid) {
        if outgoing.is_empty() {
            return;
        }
        debug!("Removing {} outgoing travellers", outgoing.len());
        for (point, migrator) in outgoing {
            AgentLocationMap::decrement_counts(&migrator.state_machine.state, counts);
            match self.agent_cells.remove(point) {
                None => {
                    panic!(
                        "Trying to remove citizen {:?} from location {:?}, but no citizen is present at this location!",
                        migrator.id, point
                    )
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

    pub fn remove_commuters(&mut self, outgoing: &Vec<(Point, Commuter)>, counts: &mut Counts) {
        if outgoing.is_empty() {
            return;
        }
        debug!("Removing {} outgoing commuters", outgoing.len());
        for (point, commuter) in outgoing {
            AgentLocationMap::decrement_counts(&commuter.state_machine.state, counts);
            match self.agent_cells.remove(point) {
                None => {
                    panic!(
                        "Trying to remove citizen {:?} from location {:?}, but no citizen is present at this location!",
                        commuter.id, point
                    )
                }
                Some(citizen) => {
                    debug!("removed the commuter successfully {:?}", citizen);
                }
            }
        }
    }

    fn decrement_counts(state: &State, counts: &mut Counts) {
        match state {
            State::Susceptible { .. } => counts.remove_susceptible(1),
            State::Exposed { .. } => counts.remove_exposed(1),
            State::Infected { .. } => counts.remove_infected(1),
            State::Recovered { .. } => counts.remove_recovered(1),
            State::Deceased { .. } => {
                panic!("Deceased agent should not travel!")
            }
        }
    }

    pub fn assimilate_migrators(
        &mut self,
        incoming: &mut Vec<Migrator>,
        grid: &mut Grid,
        counts: &mut Counts,
        rng: &mut RandomWrapper,
    ) {
        if incoming.is_empty() {
            return;
        }
        debug!("Assimilating {} incoming migrators", incoming.len());

        let migration_locations = self.select_starting_points(&grid.housing_area, incoming.len(), rng);
        if migration_locations.len() < incoming.len() {
            panic!("Not enough housing locations are available for migrators")
        };

        let mut new_citizens: Vec<Citizen> = Vec::with_capacity(incoming.len());
        for (migrator, migration_location) in incoming.iter().zip(migration_locations) {
            let house = grid.choose_house_with_free_space(rng);
            let office = if migrator.working { grid.choose_office_with_free_space(rng) } else { house.clone() };
            let citizen =
                Citizen::from_migrator(migrator, house.clone(), office.clone(), migration_location, grid.housing_area.clone());
            new_citizens.push(citizen.clone());
            grid.add_house_occupant(&house.clone());
            if migrator.working {
                grid.add_office_occupant(&office.clone())
            }

            AgentLocationMap::increment_counts(&citizen.state_machine.state, counts);
            let result = self.agent_cells.insert(migration_location, citizen);
            assert!(result.is_none());
        }
    }

    fn increment_counts(state: &State, counts: &mut Counts) {
        match state {
            State::Susceptible { .. } => counts.update_susceptible(1),
            State::Exposed { .. } => counts.update_exposed(1),
            State::Infected { .. } => counts.update_infected(1),
            State::Recovered { .. } => counts.update_recovered(1),
            State::Deceased { .. } => {
                panic!("Should not receive deceased agent!")
            }
        }
    }

    pub fn assimilate_commuters(
        &mut self,
        incoming: &mut Vec<Commuter>,
        grid: &mut Grid,
        counts: &mut Counts,
        rng: &mut RandomWrapper,
        simulation_hour: Hour,
    ) {
        if incoming.is_empty() {
            return;
        }
        debug!("Assimilating {} incoming commuters", incoming.len());

        let transport_locations = self.select_starting_points(&grid.transport_area, incoming.len(), rng);
        if transport_locations.len() < incoming.len() {
            panic!("Not enough transport location are available for commuters")
        };

        let mut new_citizens: Vec<Citizen> = Vec::with_capacity(incoming.len());
        for (commuter, transport_location) in incoming.iter().zip(transport_locations) {
            let work_area: Option<Area> = if simulation_hour == constants::ROUTINE_TRAVEL_START_TIME {
                debug!("inside if of simulation hour");
                let office = grid.choose_office_with_free_space(rng);
                debug!("got the office space - {:?}", office.clone());
                grid.add_office_occupant(&office.clone());
                debug!("added the office occupant");
                Some(office.clone())
            } else {
                None
            };

            let citizen = Citizen::from_commuter(commuter, transport_location, grid.housing_area.clone(), work_area);
            new_citizens.push(citizen.clone()); //use current area as transport area

            AgentLocationMap::increment_counts(&citizen.state_machine.state, counts);

            let result = self.insert(citizen.transport_location, citizen);
            debug!("citizen inserted");
            assert!(result.is_none());
            debug!("assert passes");
        }

        debug!("total new citizens: {}", new_citizens.len());
        debug!("For loop ended");
    }

    fn select_starting_points(&self, area: &Area, no_of_incoming: usize, rng: &mut RandomWrapper) -> Vec<Point> {
        let empty_spaces = (area.start_offset.x..area.end_offset.x).flat_map(|x| {
            (area.start_offset.y..area.end_offset.y).map(move |y| Point { x, y }).filter(|z| !self.agent_cells.contains_key(z))
        });

        empty_spaces.choose_multiple(rng.get(), no_of_incoming)
    }

    pub fn current_population(&self) -> Count {
        self.agent_cells.len() as Count
    }

    pub fn iter(&self) -> Iter<'_, Point, Citizen> {
        self.agent_cells.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Point, Citizen> {
        self.agent_cells.iter_mut()
    }

    pub fn clear(&mut self) {
        self.agent_cells.clear();
    }

    pub fn get(&self, point: &Point) -> Option<&Citizen> {
        self.agent_cells.get(point)
    }

    pub fn insert(&mut self, point: Point, citizen: Citizen) -> Option<Citizen> {
        self.agent_cells.insert(point, citizen)
    }

    // pub fn remove(&mut self, point: Point) -> Option<Citizen> {
    //     self.agent_cell.remove(&point)
    // }
}

#[cfg(test)]
mod tests {
    use crate::agent::WorkStatus;
    use crate::random_wrapper::RandomWrapper;

    use super::*;

    fn before_each() -> AgentLocationMap {
        let mut rng = RandomWrapper::new();
        let points = vec![Point { x: 0, y: 1 }, Point { x: 1, y: 0 }];
        let engine_id = "engine1".to_string();
        let home_locations = vec![
            Area::new(engine_id.clone(), Point::new(0, 0), Point::new(2, 2)),
            Area::new(engine_id.clone(), Point::new(3, 0), Point::new(4, 2)),
        ];

        let work_locations = vec![
            Area::new(engine_id.clone(), Point::new(5, 0), Point::new(6, 2)),
            Area::new(engine_id, Point::new(7, 0), Point::new(8, 2)),
        ];
        let working = WorkStatus::NA {};
        let non_working = WorkStatus::Normal {};

        let agents = vec![
            Citizen::new(home_locations[0].clone(), work_locations[0].clone(), points[0], false, non_working, &mut rng),
            Citizen::new(home_locations[1].clone(), work_locations[0].clone(), points[0], true, working, &mut rng),
        ];
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
        let working = WorkStatus::NA {};
        let non_working = WorkStatus::Normal {};
        let engine_id = "engine1".to_string();
        let home_locations = vec![
            Area::new(engine_id.clone(), Point::new(0, 0), Point::new(2, 2)),
            Area::new(engine_id.clone(), Point::new(3, 0), Point::new(4, 2)),
        ];

        let work_locations = vec![
            Area::new(engine_id.clone(), Point::new(5, 0), Point::new(6, 2)),
            Area::new(engine_id.clone(), Point::new(7, 0), Point::new(8, 2)),
        ];
        let mut citizen1 =
            Citizen::new(home_locations[0].clone(), work_locations[1].clone(), points[0], false, non_working, &mut rng);
        let citizen2 = Citizen::new(home_locations[1].clone(), work_locations[0].clone(), points[0], true, working, &mut rng);
        let agents = vec![citizen1.clone(), citizen2];
        let map = AgentLocationMap::new(5, &agents, &points);
        let hospital = Area::new(engine_id, Point::new(2, 2), Point::new(4, 4));
        let result = map.goto_hospital(&hospital, points[0], &mut citizen1);

        assert!(result.0);
        assert_eq!(result.1, Point::new(2, 2));
    }

    #[test]
    fn should_goto_home_location_when_hospital_full() {
        let mut rng = RandomWrapper::new();
        let engine_id = "engine1".to_string();
        let points = vec![Point::new(0, 0), Point::new(0, 1), Point::new(1, 0), Point::new(1, 1)];
        let home = Area::new(engine_id.clone(), Point::new(0, 0), Point::new(2, 2));
        let work_status = WorkStatus::NA {};

        let work = Area::new(engine_id.clone(), Point::new(5, 0), Point::new(6, 2));
        let citizen1 = Citizen::new(home.clone(), work.clone(), points[0], false, work_status, &mut rng);
        let citizen2 = Citizen::new(home.clone(), work.clone(), points[0], false, work_status, &mut rng);
        let citizen3 = Citizen::new(home.clone(), work.clone(), points[0], false, work_status, &mut rng);
        let citizen4 = Citizen::new(home, work, points[0], false, work_status, &mut rng);
        let agents = vec![citizen1.clone(), citizen2, citizen3, citizen4];
        let map = AgentLocationMap::new(5, &agents, &points);
        let hospital = Area::new(engine_id, Point::new(0, 0), Point::new(1, 1));

        let result = map.goto_hospital(&hospital, points[0], &mut citizen1.clone());

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
