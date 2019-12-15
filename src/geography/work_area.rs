use crate::geography::point::Point;
use crate::geography::Area;
use crate::agent;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct WorkArea{
    pub start_offset: Point,
    pub end_offset: Point
}

impl Area for WorkArea{

    fn get_dimensions(&self, agent: agent::Citizen) -> Vec<Point> {
        agent.work_location.get_neighbor_within_bounds(self.start_offset, self.end_offset)
    }
}

impl WorkArea{
    pub fn new(start: Point, end:Point) -> WorkArea{
        WorkArea{start_offset: start, end_offset:end}
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn generate_housing_area(){
        let work_area = WorkArea::new(Point::new(3, 0), Point::new(5, 5));
        assert_eq!(work_area.start_offset, Point{x: 3, y:0});
        assert_eq!(work_area.end_offset, Point{x:5, y:5});
    }

    #[test]
    fn get_work_dimensions(){
        let work_area = WorkArea::new(Point::new(3, 0), Point::new(5, 5));
        let point = Point::new(2,2);
        let point_vector = work_area.get_dimensions(agent::Citizen::new_citizen(1, point, point, false, false));

        assert_eq!(point_vector.len(), 3);
    }

}