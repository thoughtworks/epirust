use crate::disease::small_pox;
use rand::seq::SliceRandom;

#[derive(Copy, Clone)]
pub struct Citizen {
    pub id: i32,
    pub infected: bool,
    pub disease_randomness_factor: i32,
    pub infection_day: i32
}

impl Citizen {
    pub fn new() -> Citizen {
        Citizen{ id:-1, infected: false, disease_randomness_factor: 0, infection_day: 0}
    }

    pub fn new_citizen(id: i32, infected: bool) -> Citizen {
        let disease_randomness_factor = Citizen::generate_disease_randomness_factor();
        Citizen{id, infected, disease_randomness_factor, infection_day: 0 }
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