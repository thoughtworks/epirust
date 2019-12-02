use crate::geography::point::Point;
use crate::geography::Area;
use crate::constants;
use crate::agent::Citizen;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct HousingArea{
    pub start_offset: Point,
    pub end_offset: Point
}

impl Area for HousingArea{
//TODO: Pass point instead of an agent
    fn get_dimensions(&self, agent: Citizen) -> Vec<Point>{
//        TODO: Change method call
        agent.home_location.get_neighbor_within_bounds(self.start_offset, self.end_offset)
    }
}

impl HousingArea{
    pub fn new(start: Point, end:Point) -> HousingArea{
        HousingArea{start_offset: start, end_offset:end}
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn generate_housing_area(){
        let housing_area = HousingArea::new(Point::new(0, 0), Point::new(2, 5));

        assert_eq!(housing_area.end_offset, Point{x:2, y:5});
    }

    #[test]
    fn get_house_dimensions(){
        let housing_area = HousingArea::new(Point::new(0, 0), Point::new(2, 5));
        let point_vector = housing_area.get_dimensions(Citizen::new_citizen(1, false, Point::new(2,2), Point::new(3,3)));

        assert_eq!(point_vector.len(), 5);
    }
}