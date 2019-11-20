extern crate rand;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32
    //    TODO: remove pub usages
}

impl Point {
    pub fn translate_to_coordinate_system(size: i32, point: i32) -> Point{
        Point{x: point/size, y: point%size}
    }
}

pub fn point_factory(size:i32, number_of_points:i32) -> Vec<Point>{
    let mut points:Vec<Point> = Vec::with_capacity(number_of_points as usize);
    for i in 0..number_of_points{
        points.push(Point::translate_to_coordinate_system(size, i));
    }
    points
}

#[test]
fn generate_points(){
    let points:Vec<Point> = point_factory(5, 10);

    assert_eq!(points.len(), 10);
}