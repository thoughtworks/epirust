use crate::disease::small_pox;
use rand::seq::SliceRandom;
use crate::geography::point::Point;

#[derive(Copy, Clone)]
pub struct Citizen {
    pub id: i32,
    pub infected: bool,
    pub disease_randomness_factor: i32,
    pub infection_day: i32,
    pub home_location: Point
}

impl Citizen {
    pub fn new() -> Citizen {
        Citizen{ id:-1, infected: false, disease_randomness_factor: 0, infection_day: 0, home_location:Point::new(-1, -1)}
    }

    pub fn new_citizen(id: i32, infected: bool, home_location: Point) -> Citizen {
        let disease_randomness_factor = Citizen::generate_disease_randomness_factor();
        Citizen{id, infected, disease_randomness_factor, infection_day: 0, home_location}
    }

    pub fn increment_infection_day(&mut self){
        self.infection_day += 1;
    }

    pub fn get_infection_transmission_rate(&self) -> f64{
        small_pox::get_current_transmission_rate(self.infection_day + self.disease_randomness_factor)
    }

    fn generate_disease_randomness_factor() -> i32{
        let option = [-2, -1, 0, 1, 2].choose(&mut rand::thread_rng());
        *option.unwrap()
    }
}

pub fn citizen_factory(points: &Vec<Point>) -> Vec<Citizen>{
    let mut agent_list = Vec::with_capacity(points.len());
    let mut agent_id = 0;
    for point in points{
        let agent = Citizen::new_citizen(agent_id, false, *point);
        agent_list.push(agent);
        agent_id += 1;
    }

    agent_list.last_mut().as_mut().unwrap().infected = true;
    agent_list
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn generate_citizen(){
        let points = vec![Point::new(0, 0), Point::new(0, 1), Point::new(1, 0)];

        let citizen_list = citizen_factory(&points);
        assert_eq!(citizen_list.len(), 3);
        assert_eq!(citizen_list[1].home_location, Point::new(0, 1));
        assert_eq!(citizen_list.last().unwrap().infected, true);
    }
}