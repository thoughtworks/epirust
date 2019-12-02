use crate::disease::small_pox;
use rand::seq::SliceRandom;
use crate::geography::point::Point;
use crate::constants;

#[derive(Copy, Clone)]
pub struct Citizen {
    pub id: i32,
    pub infected: bool,
    pub immunity: i32,
    pub infection_day: i32,
    pub home_location: Point,
    pub work_location: Point,
}

impl Citizen {
    pub fn new() -> Citizen {
        Citizen{ id:-1, infected: false, immunity: 0, infection_day: 0, home_location:Point::new(-1, -1), work_location:Point::new(-1, -1)}
    }

    pub fn new_citizen(id: i32, infected: bool, home_location: Point, work_location: Point) -> Citizen {
        let disease_randomness_factor = Citizen::generate_disease_randomness_factor();
        Citizen{id, infected, immunity: disease_randomness_factor, infection_day: 0, home_location, work_location}
    }

    pub fn increment_infection_day(&mut self){
        self.infection_day += 1;
    }

    pub fn get_infection_transmission_rate(&self) -> f64{
        small_pox::get_current_transmission_rate(self.infection_day + self.immunity)
    }

    fn generate_disease_randomness_factor() -> i32{
        let option = constants::IMMUNITY_RANGE.choose(&mut rand::thread_rng());
        *option.unwrap()
    }
}

pub fn citizen_factory(home_locations: &Vec<Point>, work_locations: &Vec<Point>) -> Vec<Citizen>{
    let mut agent_list = Vec::with_capacity(home_locations.len());

    for i in 0..home_locations.len(){
        let agent = Citizen::new_citizen(i as i32, false, home_locations[i], work_locations[i]);
        agent_list.push(agent);
    }

    agent_list.last_mut().as_mut().unwrap().infected = true;
    agent_list
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn generate_citizen(){
        let home_locations = vec![Point::new(0, 0), Point::new(0, 1), Point::new(1, 0)];
        let work_locations = vec![Point::new(0, 0), Point::new(0, 1), Point::new(1, 0)];

        let citizen_list = citizen_factory(&home_locations, &work_locations);
        assert_eq!(citizen_list.len(), 3);
        assert_eq!(citizen_list[1].home_location, Point::new(0, 1));
        assert_eq!(citizen_list.last().unwrap().infected, true);
    }
}