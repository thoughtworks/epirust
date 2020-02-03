use crate::geography::Point;
use crate::constants;
use std::cmp::{max, min};
use rand::Rng;

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

    pub fn get_neighbors_of(&self, point: Point) -> Vec<Point> {
        let mut neighbors_list = Vec::with_capacity(constants::NEIGHBORS);
        let mut row_index = max(self.start_offset.x, point.x - 1);

        loop {
            if row_index > min(point.x + 1, self.end_offset.x) {
                break;
            }
            let mut col_index = max(self.start_offset.y, point.y - 1);
            loop {
                if col_index > min(point.y + 1, self.end_offset.y) {
                    break;
                }
                if row_index == point.x && col_index == point.y {
                    col_index += 1;
                    continue;
                }
                neighbors_list.push(Point { x: row_index, y: col_index });
                col_index += 1;
            }
            row_index += 1;
        }

        neighbors_list
    }

    //TODO improve randomness
    pub fn random_points(&self, number_of_points: i32) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::with_capacity(number_of_points as usize);
        let mut rng = rand::thread_rng();
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

    //used in test
    pub fn contains(&self, point: &Point) -> bool {
        self.start_offset.x <= point.x && self.end_offset.x >= point.x
            && self.start_offset.y <= point.y && self.end_offset.y >= point.y
    }
}

impl Iterator for Area {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let mut new_x = self.iter_index.x + 1;
        let mut new_y = self.iter_index.y;
        if new_x > self.end_offset.x {
            new_x = 0;
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
    fn get_neighbor_within() {
        let point1 = Point::new(1, 1);
        let point2 = Point::new(1, 2);
        let area1 = Area::new(Point::new(0, 0), Point::new(2, 2));
        let area2 = Area::new(Point::new(0, 0), Point::new(2, 2));
        let point_vector1 = area1.get_neighbors_of(point1);
        let point_vector2 = area2.get_neighbors_of(point2);

        assert_eq!(point_vector1.len(), 8);
        assert_eq!(point_vector2.len(), 5);
    }

    #[test]
    fn generate_points() {
        let area = Area::new(Point { x: 0, y: 0 }, Point { x: 5, y: 5 });
        let points: Vec<Point> = area.random_points(10);

        assert_eq!(points.len(), 10);
    }

    #[test]
    fn should_iterate_over_points_in_area() {
        let area = Area::new(Point { x: 0, y: 0 }, Point { x: 2, y: 2 });
        let x: Vec<Point> = area.collect();
        assert_eq!(x, vec![Point::new(0, 0), Point::new(1, 0), Point::new(2, 0),
                           Point::new(0, 1), Point::new(1, 1), Point::new(2, 1),
                           Point::new(0, 2), Point::new(1, 2), Point::new(2, 2)])
    }
}
