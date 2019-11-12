extern crate rand;

use rand::seq::SliceRandom;
use std::cmp::max;
use std::cmp::min;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub occupied: bool
    //    TODO: remove pub usages
    //    TODO: Occupied to be specified always
}

impl Point {
    pub fn new() -> Point {
        Point { x: 0, y: 0, occupied: false}
    }

    pub fn set_coordinates(&mut self, x:i32, y:i32, occupied:bool){
        self.x = x;
        self.y = y;
        self.occupied = occupied;
    }

    pub fn set_occupancy(&mut self, occupancy:bool){
        self.occupied = occupancy;
    }
}

pub struct Grid {
    pub size: i32,
    pub cells: Vec<Vec<Point>>
}

impl Grid {

    pub fn new(size: i32) -> Grid{
        let mut cells = vec![vec![Point::new(); size as usize]; size as usize];

        for row in 0..size{
            for column in 0..size{
                cells[row as usize][column as usize].set_coordinates(row, column, false);
            }
        }

        Grid{size, cells}
    }

    pub fn get_empty_neighbor_point(&self, location:Point) -> Point{
        let neighbors = self.get_empty_neighbor_points(location);
        Grid::get_random_point_from(neighbors, location)
    }

    pub fn get_empty_cells(&self) -> Vec<Point>{
        let mut empty_cells:Vec<Point> = Vec::with_capacity(10);
        for row in self.cells.iter(){
            for column in row.iter(){
                if column.occupied == false{
                    empty_cells.push(*column);
                }
            }
        }
        empty_cells
    }

    pub fn update_points_occupancy(&mut self, points:Vec<Point>){
        for point in points{
            self.cells[point.x as usize][point.y as usize].set_occupancy(point.occupied);
        }
    }

    fn get_empty_neighbor_points(&self, location: Point) -> Vec<Point>{
        const NUMBER_OF_NEIGHBORS:i32 = 8;
        let mut neighbors_list = Vec::with_capacity(NUMBER_OF_NEIGHBORS as usize);
        let mut row_index = max(0, location.x - 1);

        loop{
            if row_index > min(location.x+1, self.size-1) {
                break;
            }
            let mut col_index = max(0, location.y - 1) ;
            loop{
                if col_index > min(location.y+1, self.size-1) {
                    break;
                }
                if (row_index == location.x && col_index == location.y) || self.cells[row_index as usize][col_index as usize].occupied {
                    col_index = col_index + 1;
                    continue;
                }
                neighbors_list.push(self.cells[row_index as usize][col_index as usize]);
                col_index = col_index + 1;
            }
            row_index = row_index + 1;
        }

        neighbors_list
    }

    //    TODO: Move this function to vector wrapper
    fn get_random_point_from(vector: Vec<Point>, location: Point) -> Point {
        let choice = vector.choose(&mut rand::thread_rng());
        match choice {
            Some(x) => return *x,
            None => return location
        }
    }
}

#[test]
fn generate_cells() {
    let grid = Grid::new(3);
    let mut point = Point::new();
    point.set_coordinates(1, 0, false);

    assert_eq!(grid.cells[1][0].x, point.x);
}

#[test]
fn get_neighbor_points(){
    let size = 5;
    let grid = Grid::new(size);
    let location:Point = Point { x: 0, y: 0, occupied: false };

    let neighbors:Vec<Point> = grid.get_empty_neighbor_points(location);
    assert_eq!(neighbors.len(), 3);
}

#[test]
fn update_points_occupancy(){
    let size = 3;
    let mut grid = Grid::new(size);

    let points = vec![Point{x:0, y:0, occupied: true}, Point{x:0, y:1, occupied: false}];
    assert_eq!(grid.cells[0][0].occupied, false);
    grid.update_points_occupancy(points);

    assert_eq!(grid.cells[0][0].occupied, true);
    assert_eq!(grid.cells[0][1].occupied, false);
}

#[test]
fn get_random_empty_neighbor(){
    let size = 5;
    let grid = Grid::new(size);
    let location:Point = Point { x: 2, y: 2, occupied: false };

    let neighbor:Point = grid.get_empty_neighbor_point(location);
    assert_eq!((1 <= neighbor.x) && (neighbor.x <= 3), true);
    assert_eq!((1 <= neighbor.y) && (neighbor.y <= 3), true);
}

#[test]
fn get_random_empty_cells(){
    let size = 3;
    let mut grid = Grid::new(size);
    grid.cells[0][0].set_occupancy(true);
    grid.cells[0][1].set_occupancy(true);
    grid.cells[0][2].set_occupancy(true);

    let neighbors = grid.get_empty_cells();
    assert_eq!(neighbors.len(), 6);
    assert_eq!(neighbors.contains(&Point{x:0, y:0, occupied: true}), false);
}