use crate::geography::point::Point;
use crate::agent;

pub mod point;
pub mod housing_area;
pub mod work_area;
pub mod transport_area;

pub trait Area{
    fn get_dimensions(&self, agent:agent::Citizen) -> Vec<Point>;
}

#[cfg(test)]
mod tests{

    use super::*;
    #[test]
    pub fn get_dimensions(){
        let area = housing_area::HousingArea::new(Point::new(0, 0), Point::new(1, 3));
        area.get_dimensions(agent::Citizen::new());
    }
}