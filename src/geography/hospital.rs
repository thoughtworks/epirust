use crate::geography::point::*;
use crate::geography::Area;
use crate::agent::Citizen;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct Hospital{
    pub start_offset: Point,
    pub end_offset: Point
}

impl Area for Hospital{
    //TODO: Pass point instead of an agent
    fn get_dimensions(&self, _: Citizen) -> Vec<Point>{
        get_points_within(self.start_offset, self.end_offset)
    }
}

impl Hospital{
    pub fn new(start: Point, end:Point) -> Hospital{
        Hospital{start_offset: start, end_offset:end}
    }
}