use crate::geography::point::Point;
use crate::agent;

pub mod point;
pub mod housing_area;
pub mod work_area;
pub mod transport_area;
pub mod hospital;

pub trait Area{
    fn get_dimensions(&self, agent:agent::Citizen) -> Vec<Point>;
}