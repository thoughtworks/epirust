use crate::geography::point::Point;
use crate::agent;
use crate::constants;
use std::cmp::{max, min};

pub mod point;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct Area {
    start_offset: Point,
    end_offset: Point,
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
}

pub struct Grid {
    pub housing_area: Area,
    pub work_area: Area,
    pub transport_area: Area,
    pub hospital: Area,
}

impl Grid {
    pub fn generate_population(&self, number_of_agents: i32, public_transport_percentage: f64, working_percentage: f64)
                               -> (Vec<Point>, Vec<agent::Citizen>) {
        let home_locations = point::point_factory(self.housing_area.start_offset,
                                                  self.housing_area.end_offset, number_of_agents);
        let scaling_factor = self.housing_area.end_offset.x + self.transport_area.end_offset.x;
        let work_locations = home_locations.iter()
            .map(|x| *x + point::Point::new(scaling_factor, 0)).collect();
//        TODO: fix the hack
        let number_of_agents_using_public_transport = number_of_agents as f64 * (public_transport_percentage + 0.1) * (working_percentage + 0.1);
        let transport_locations = point::point_factory(self.transport_area.start_offset, self.transport_area.end_offset, number_of_agents_using_public_transport as i32);
        let agent_list = agent::citizen_factory(&home_locations, &work_locations, &transport_locations, public_transport_percentage, working_percentage);
        (home_locations, agent_list)
    }
}

pub fn define_geography(grid_size: i32) -> Grid {
    let bound = grid_size - 1;
    let x_offset_for_home = (bound as f32 * constants::HOUSE_AREA_RELATIVE_SIZE).ceil() as i32 - 1;
    let x_offset_for_transport = x_offset_for_home + (bound as f32 * constants::TRANSPORT_AREA_RELATIVE_SIZE).ceil() as i32;
    let x_offset_for_hospital = x_offset_for_transport + (bound as f32 * constants::HOSPITAL_RELATIVE_SIZE).ceil() as i32;

    let housing_area = Area::new(Point::new(0, 0), Point::new(x_offset_for_home, bound));
    let transport_area = Area::new(Point::new(x_offset_for_home + 1, 0), Point::new(x_offset_for_transport, bound));
    let hospital = Area::new(Point::new(x_offset_for_transport + 1, 0), Point::new(x_offset_for_hospital, bound));
    let work_area = Area::new(Point::new(x_offset_for_hospital + 1, 0), Point::new(bound, bound));
    Grid { housing_area, transport_area, hospital, work_area }
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
}
