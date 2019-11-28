use crate::geography::point::Point;
use crate::constants;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct HousingArea{
    pub start_offset: Point,
    pub end_offset: Point
}

impl HousingArea{
    pub fn new(grid_size: i32) -> HousingArea{
        HousingArea{start_offset: Point::new(0, 0), end_offset:Point::new(grid_size/ constants::HOUSE_RELATED_SIZE, grid_size)}
    }

    pub fn get_house_dimensions(&self, house_location: Point) -> Vec<Point>{
        house_location.get_neighbor_within_bounds(self.start_offset, self.end_offset)
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn generate_housing_area(){
        let housing_area = HousingArea::new(5);

        assert_eq!(housing_area.end_offset, Point{x:2, y:5});
    }

    #[test]
    fn get_house_dimensions(){
        let housing_area = HousingArea::new(5);
        let point = Point::new(2,2);
        let point_vector = housing_area.get_house_dimensions(point);

        assert_eq!(point_vector.len(), 5);
    }
}