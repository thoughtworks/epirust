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

use fxhash::FxHashMap;

use crate::agent;
use crate::geography::Area;
use crate::geography::Point;
use crate::random_wrapper::RandomWrapper;

#[derive(Clone)]
pub struct AgentLocationMap {
    pub grid_size: i32,
    pub agent_cell: FxHashMap<Point, agent::Citizen>,
    pub total_population: i32,
}

impl AgentLocationMap {
    pub fn new(size: i32, agent_list: &[agent::Citizen], points: &[Point]) -> AgentLocationMap {
        debug!("{} agents and {} starting points", agent_list.len(), points.len());
        let mut map: FxHashMap<Point, agent::Citizen> = FxHashMap::default();
        for i in 0..agent_list.len() {
            map.insert(points[i], agent_list[i]);
        }

        AgentLocationMap { grid_size: size, agent_cell: map, total_population: agent_list.len() as i32 }
    }

    pub fn move_agent(&self, old_cell: Point, new_cell: Point) -> Point {
        if self.is_cell_vacant(&new_cell) {
            return new_cell;
        }
        old_cell
    }

    pub fn goto_hospital(&self, hospital_area: &Area, cell: Point, citizen: &mut agent::Citizen) -> Point {
        let vacant_hospital_cell = hospital_area.into_iter().find(|cell| {
            self.is_cell_vacant(cell)
        });
        self.move_agent(cell, vacant_hospital_cell.unwrap_or(citizen.home_location.get_random_point(&mut RandomWrapper::new())))
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::random_wrapper::RandomWrapper;

    fn before_each() -> AgentLocationMap {
        let mut rng = RandomWrapper::new();
        let points = vec![Point { x: 0, y: 1 }, Point { x: 1, y: 0 }];
        let home_locations = vec![Area::new(Point::new(0, 0), Point::new(2, 2)), Area::new(Point::new(3, 0), Point::new(4, 2))];

        let work_locations = vec![Area::new(Point::new(5, 0), Point::new(6, 2)), Area::new(Point::new(7, 0), Point::new(8, 2))];

        let agents = vec![agent::Citizen::new_citizen(1, home_locations[0], work_locations[0], points[0], false, false, &mut rng),
                          agent::Citizen::new_citizen(2, home_locations[1], work_locations[0], points[0], true, true, &mut rng)];
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
        let home_locations = vec![Area::new(Point::new(0, 0), Point::new(2, 2)), Area::new(Point::new(3, 0), Point::new(4, 2))];

        let work_locations = vec![Area::new(Point::new(5, 0), Point::new(6, 2)), Area::new(Point::new(7, 0), Point::new(8, 2))];
        let mut citizen1 = agent::Citizen::new_citizen(1, home_locations[0], work_locations[1], points[0], false, false, &mut rng);
        let citizen2 = agent::Citizen::new_citizen(2, home_locations[1], work_locations[0], points[0], true, true, &mut rng);
        let agents = vec![citizen1, citizen2];
        let map = AgentLocationMap::new(5, &agents, &points);
        let hospital = Area::new(Point::new(2, 2), Point::new(4, 4));

        assert_eq!(map.goto_hospital(&hospital, points[0], &mut citizen1), Point::new(2, 2));
    }

    #[test]
    fn should_goto_home_location_when_hospital_full() {
        let mut rng = RandomWrapper::new();
        let points = vec![Point::new(0, 0), Point::new(0, 1), Point::new(1, 0), Point::new(1, 1)];
        let home = Area::new(Point::new(0, 0), Point::new(2, 2));

        let work = Area::new(Point::new(5, 0), Point::new(6, 2));
        let mut citizen1 = agent::Citizen::new_citizen(1, home, work, points[0], false, false, &mut rng);
        let citizen2 = agent::Citizen::new_citizen(2, home, work, points[0], false, false, &mut rng);
        let citizen3 = agent::Citizen::new_citizen(3, home, work, points[0], false, false, &mut rng);
        let citizen4 = agent::Citizen::new_citizen(4, home, work, points[0], false, false, &mut rng);
        let agents = vec![citizen1, citizen2, citizen3, citizen4];
        let map = AgentLocationMap::new(5, &agents, &points);
        let hospital = Area::new(Point::new(0, 0), Point::new(1, 1));

        assert_eq!(citizen1.clone().home_location.contains(&map.goto_hospital(&hospital, points[0], &mut citizen1)), true);
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
