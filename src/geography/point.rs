extern crate rand;
use rand::Rng;
use std::cmp::max;
use std::cmp::min;
use std::ops::Add;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32
    //    TODO: remove pub usages
}

impl Point {

    pub fn new(x: i32, y:i32) -> Point{
        Point{x, y}
    }

    pub fn get_neighbor_cells(&self, size:i32) -> Vec<Point>{
        const NUMBER_OF_NEIGHBORS:i32 = 8;
        let mut neighbors_list = Vec::with_capacity(NUMBER_OF_NEIGHBORS as usize);
        let mut row_index = max(0, self.x - 1);

        loop{
            if row_index > min(self.x+1, size-1) {
                break;
            }
            let mut col_index = max(0, self.y - 1) ;
            loop{
                if col_index > min(self.y+1, size-1) {
                    break;
                }
                if row_index == self.x && col_index == self.y {
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

    pub fn get_neighbor_within_bounds(&self, start: Point, end: Point) -> Vec<Point>{
        const NUMBER_OF_NEIGHBORS:i32 = 8;
        let mut neighbors_list = Vec::with_capacity(NUMBER_OF_NEIGHBORS as usize);
        let mut row_index = max(start.x, self.x - 1);

        loop{
            if row_index > min(self.x+1, end.x) {
                break;
            }
            let mut col_index = max(start.y, self.y - 1) ;
            loop{
                if col_index > min(self.y+1, end.y) {
                    break;
                }
                if row_index == self.x && col_index == self.y {
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
}

impl Add for Point{
    type Output = Self;

    fn add(self, second_point: Self) -> Self {
        Self{x: self.x + second_point.x, y: self.y+second_point.y}
    }
}

//TODO: Improve randomness generation
pub fn point_factory(start:Point, end:Point, number_of_points:i32) -> Vec<Point>{
    let mut points:Vec<Point> = Vec::with_capacity(number_of_points as usize);
    let mut rng = rand::thread_rng();
    while points.len() != (number_of_points as usize) {
        let rand_x = rng.gen_range(start.x, end.x);
        let rand_y = rng.gen_range(start.y, end.y);
        let mut is_duplicate = false;
        for point in points.iter_mut(){
            if *point == (Point::new(rand_x, rand_y)) {
//                println!("Duplicate");
                is_duplicate = true;
            }
        }
        if !is_duplicate{
            points.push(Point{x:rand_x, y:rand_y});
        }
    }
    points
}

//TODO: Add tests
pub fn get_points_within(start: Point, end: Point) -> Vec<Point>{
    let mut point_vec:Vec<Point> = Vec::new();

    for x in start.x..end.x+1{
        for y in start.y..end.y+1{
            point_vec.push(Point::new(x, y));
        }
    }
    point_vec
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn get_neighbor_cells(){
        let point = Point::new(1,1);
        let point_vector = point.get_neighbor_cells(5);

        assert_eq!(point_vector.len(), 8);
        assert_eq!(point_vector.contains(&Point::new(0, 0)), true);
        assert_eq!(point_vector.contains(&Point::new(2, 2)), true);
        assert_eq!(point_vector.contains(&Point::new(3, 3)), false);
    }

    #[test]
    fn get_neighbor_within(){
        let point1 = Point::new(1,1);
        let point2 = Point::new(1,2);
        let point_vector1 = point1.get_neighbor_within_bounds(Point::new(0, 0), Point::new(2, 2));
        let point_vector2 = point2.get_neighbor_within_bounds(Point::new(0, 0), Point::new(2, 2));

        assert_eq!(point_vector1.len(), 8);
        assert_eq!(point_vector2.len(), 5);
    }

    #[test]
    fn generate_points(){
        let points:Vec<Point> = point_factory(Point{x:0, y:0}, Point{x:5, y:5}, 10);

        assert_eq!(points.len(), 10);
    }

    #[test]
    fn add(){
        let point = Point::new(1,1);
        let second_point= Point::new(1,1);

        let output = point + second_point;
        assert_eq!(output, Point::new(2, 2));

    }
}