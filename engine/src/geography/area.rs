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
use std::slice::IterMut;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug, Serialize)]
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


/// loops through points in multiple areas
pub struct AreaPointIterator<'a> {
    areas_iter: IterMut<'a, Area>,
    current_area: Area,
}

impl AreaPointIterator<'_> {
    pub fn init(areas: &mut Vec<Area>) -> AreaPointIterator {
        let mut areas_iter = areas.into_iter();
        let current_area = *areas_iter.next().unwrap();
        AreaPointIterator { areas_iter, current_area }
    }
}

impl Iterator for AreaPointIterator<'_> {
    type Item = (Area, Point);

    fn next(&mut self) -> Option<Self::Item> {
        let maybe_point = self.current_area.next();
        if maybe_point.is_some() {
            return Some((self.current_area, maybe_point.unwrap()));
        }
        //no Points left in current area, pick up next area
        let next_area = self.areas_iter.next();
        if next_area.is_some() {
            self.current_area = *next_area.unwrap();
            let point = self.current_area.next().unwrap();
            return Some((self.current_area, point));
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_area() -> Area {
        Area::new(Point { x: 0, y: 0 }, Point { x: 5, y: 5 })
    }

    #[test]
    fn generate_points() {
        let area = get_area();
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

    #[test]
    fn should_iterate_over_points_in_multiple_areas() {
        let area1 = Area::new(Point { x: 0, y: 0 }, Point { x: 1, y: 1 });
        let area2 = Area::new(Point { x: 2, y: 0 }, Point { x: 3, y: 1 });
        let mut areas = vec![area1, area2];
        let areas_points_iter = AreaPointIterator::init(&mut areas);

        let mut points = Vec::new();
        for (_area, point) in areas_points_iter {
            points.push(point);
        }

        assert_eq!(points, vec![
            Point::new(0,0), Point::new(1,0), Point::new(0,1), Point::new(1,1),
            Point::new(2,0), Point::new(3,0), Point::new(2,1), Point::new(3,1),
        ])
    }


    #[test]
    fn should_get_neighbor_of(){
        let area = get_area();
        let neighbors: Vec<Point> = area.get_neighbors_of(Point::new(4,5)).collect();
        assert_eq!(neighbors.len(), 5);
        assert_eq!(neighbors.contains(&Point::new(4, 4)), true);
        assert_eq!(neighbors.contains(&Point::new(4, 6)), false);
    }

    #[test]
    fn should_return_true_if_area_has_point(){
        let area = get_area();

        let is_inside_area = area.contains(&Point::new(2, 3));
        assert_eq!(is_inside_area, true);
    }

    #[test]
    fn should_return_false_if_area_does_not_contain_point(){
        let area = get_area();

        let is_inside_area = area.contains(&Point::new(20, 3));
        assert_eq!(is_inside_area, false);
    }

    #[test]
    fn should_get_random_point(){
        let area = get_area();

        let random_point = area.get_random_point(&mut RandomWrapper::new());
        assert_eq!(area.contains(&random_point), true);
    }
}
