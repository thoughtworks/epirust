#[derive(Copy, Clone)]
pub struct Citizen {
    pub id: i32,
    pub infected: bool
}

impl Citizen {
    pub fn new() -> Citizen {
        Citizen{ id:-1, infected: false}
    }

    pub fn new_citizen(id: i32, infected: bool) -> Citizen {
        Citizen{id, infected}
    }
}