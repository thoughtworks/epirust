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

use rand::Rng;

use crate::geography::Point;
use crate::random_wrapper::RandomWrapper;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct Area {
    pub start_offset: Point,
    pub end_offset: Point,
    iter_index: Point,
}

impl Area {
    pub fn new(start_offset: Point, end_offset: Point) -> Area {
        Area { start_offset, end_offset, iter_index: Point::new(start_offset.x - 1, start_offset.y) }
    }

    pub fn get_neighbors_of(&self, point: Point) -> impl Iterator<Item=Point> + '_ {
        point.neighbor_iterator().filter(move |p| {
            self.contains(p)
        })
    }

    //TODO improve randomness
    pub fn random_points(&self, number_of_points: i32, rng: &mut RandomWrapper) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::with_capacity(number_of_points as usize);
        let rng = rng.get();
        while points.len() != (number_of_points as usize) {
            let rand_x = rng.gen_range(self.start_offset.x, self.end_offset.x);
            let rand_y = rng.gen_range(self.start_offset.y, self.end_offset.y);
            let mut is_duplicate = false;
            for point in points.iter_mut() {
                if *point == (Point::new(rand_x, rand_y)) {
//                println!("Duplicate");
                    is_duplicate = true;
                }
            }
            if !is_duplicate {
                points.push(Point { x: rand_x, y: rand_y });
            }
        }
        points
    }

    pub fn get_random_point(&self, rng: &mut RandomWrapper) -> Point {
        let rand_x = rng.get().gen_range(self.start_offset.x, self.end_offset.x);
        let rand_y = rng.get().gen_range(self.start_offset.y, self.end_offset.y);

        Point::new(rand_x, rand_y)
    }

    pub fn contains(&self, point: &Point) -> bool {
        self.start_offset.x <= point.x && self.end_offset.x >= point.x
            && self.start_offset.y <= point.y && self.end_offset.y >= point.y
    }
}

pub fn area_factory(start_point: Point, end_point: Point, size: i32) -> Vec<Area> {
    let feasible_houses_in_x_dim = (end_point.x - start_point.x) / size;
    let feasible_houses_in_y_dim = (end_point.y - start_point.y) / size;

    let mut areas = Vec::with_capacity((feasible_houses_in_y_dim * feasible_houses_in_x_dim) as usize);
    let mut current_start_point = start_point;

    for _i in 0..feasible_houses_in_y_dim {
        for _j in 0..feasible_houses_in_x_dim {
            let current_end_point: Point = Point::new(current_start_point.x + size, current_start_point.y + size);

            areas.push(Area::new(current_start_point, current_end_point));

            current_start_point.x = current_start_point.x + size;
        }

        current_start_point.x = start_point.x;
        current_start_point.y = current_start_point.y + size;
    }

    areas
}

impl Iterator for Area {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let mut new_x = self.iter_index.x + 1;
        let mut new_y = self.iter_index.y;
        if new_x > self.end_offset.x {
            new_x = self.start_offset.x;
            new_y += 1;
        }
        self.iter_index = Point::new(new_x, new_y);
        if new_y > self.end_offset.y {
            return Option::None;
        }
        Option::Some(self.iter_index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_points() {
        let area = Area::new(Point { x: 0, y: 0 }, Point { x: 5, y: 5 });
        let points: Vec<Point> = area.random_points(10, &mut RandomWrapper::new());

        assert_eq!(points.len(), 10);
    }

    #[test]
    fn should_iterate_over_points_in_area() {
        let area = Area::new(Point { x: 0, y: 0 }, Point { x: 2, y: 2 });
        let x: Vec<Point> = area.collect();
        assert_eq!(x, vec![Point::new(0, 0), Point::new(1, 0), Point::new(2, 0),
                           Point::new(0, 1), Point::new(1, 1), Point::new(2, 1),
                           Point::new(0, 2), Point::new(1, 2), Point::new(2, 2)]);

        let area = Area::new(Point { x: 1, y: 1 }, Point { x: 2, y: 2 });
        let x: Vec<Point> = area.collect();
        assert_eq!(x, vec![Point::new(1, 1), Point::new(2, 1),
                           Point::new(1, 2), Point::new(2, 2)])
    }

    #[test]
    fn should_create_areas() {
        let buildings = area_factory(Point::new(10, 0), Point::new(21, 10), 3);

        assert_eq!(buildings.len(), 9);
        assert_eq!(buildings.get(0).unwrap().start_offset, Point::new(10, 0));
        assert_eq!(buildings.get(0).unwrap().end_offset, Point::new(13, 3));
        assert_eq!(buildings.get(5).unwrap().start_offset, Point::new(16, 3));
        assert_eq!(buildings.get(5).unwrap().end_offset, Point::new(19, 6));
        assert_eq!(buildings.last().unwrap().start_offset, Point::new(16, 6));
        assert_eq!(buildings.last().unwrap().end_offset, Point::new(19, 9));
    }
}
