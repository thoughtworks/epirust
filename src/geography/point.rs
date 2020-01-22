extern crate rand;

use std::cmp::max;
use std::cmp::min;
use std::ops::Add;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    //    TODO: remove pub usages
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    pub fn get_neighbor_cells(self, size: i32) -> Vec<Point> {
        const NUMBER_OF_NEIGHBORS: i32 = 8;
        let mut neighbors_list = Vec::with_capacity(NUMBER_OF_NEIGHBORS as usize);
        let mut row_index = max(0, self.x - 1);

        loop {
            if row_index > min(self.x + 1, size - 1) {
                break;
            }
            let mut col_index = max(0, self.y - 1);
            loop {
                if col_index > min(self.y + 1, size - 1) {
                    break;
                }
                if row_index == self.x && col_index == self.y {
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
}

impl Add for Point {
    type Output = Self;

    fn add(self, second_point: Self) -> Self {
        Self { x: self.x + second_point.x, y: self.y + second_point.y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_neighbor_cells() {
        let point = Point::new(1, 1);
        let point_vector = point.get_neighbor_cells(5);

        assert_eq!(point_vector.len(), 8);
        assert_eq!(point_vector.contains(&Point::new(0, 0)), true);
        assert_eq!(point_vector.contains(&Point::new(2, 2)), true);
        assert_eq!(point_vector.contains(&Point::new(3, 3)), false);
    }

    #[test]
    fn add() {
        let point = Point::new(1, 1);
        let second_point = Point::new(1, 1);

        let output = point + second_point;
        assert_eq!(output, Point::new(2, 2));
    }
}
