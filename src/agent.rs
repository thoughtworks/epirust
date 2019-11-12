use crate::geography::Point;

#[derive(Copy, Clone)]
pub struct Citizen {
    pub id: i32,
    pub location: Point
}

impl Citizen {
    pub fn new() -> Citizen {
        Citizen{ id:-1, location: Point::new()}
    }

    pub fn new_citizen(id: i32, point:Point) -> Citizen {
        Citizen{ id, location: point}
    }

    pub fn set_location(&mut self, new_location:Point){
        self.location = new_location;
    }
}