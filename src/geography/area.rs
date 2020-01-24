use crate::geography::Point;
use crate::constants;
use std::cmp::{max, min};
use rand::Rng;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct Area {
    pub start_offset: Point,
    pub end_offset: Point,
}

impl Area {
    pub fn new(start_offset: Point, end_offset: Point) -> Area {
        Area { start_offset, end_offset }
    }

    pub fn get_points_within(&self) -> Vec<Point> {
        let mut point_vec: Vec<Point> = Vec::new();

        for x in self.start_offset.x..=self.end_offset.x + 1 {
            for y in self.start_offset.y..=self.end_offset.y + 1 {
                point_vec.push(Point::new(x, y));
            }
        }
        point_vec
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_points_within() {
        let point1 = Point::new(1, 1);
        let point2 = Point::new(2, 2);
        let area = Area::new(point1, point2);

        let points = area.get_points_within();
        assert_eq!(points.len(), 9);
        assert_eq!(points[0], Point { x: 1, y: 1 });
        assert_eq!(points[2], Point { x: 1, y: 3 });
        assert_eq!(points[8], Point { x: 3, y: 3 });
    }

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
}
