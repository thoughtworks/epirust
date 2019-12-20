use crate::disease::small_pox;
use rand::seq::SliceRandom;
use crate::geography::point::Point;
use crate::constants;
use rand::thread_rng;
use rand::Rng;

#[derive(Copy, Clone, PartialEq)]
pub enum State{
    Susceptible{},
    Infected{},
    Quarantined{},
    Recovered{},
    Deceased{}
}

#[derive(Copy, Clone, PartialEq)]
pub struct StateMachine{
    state: State,
    infection_day: i32
}

impl StateMachine{
    fn new() -> Self{
        StateMachine{
            state: State::Susceptible{},
            infection_day: 0
        }
    }

//    pub fn next(&mut self){
//        match self.state {
//            State::Susceptible{} => self.state = State::Infected{},
//            State::Infected {} => {
//                if small_pox::to_be_quarantined(self.infection_day){
//                    self.state = State::Quarantined {}
//                }
//            }
//        }
//    }

    pub fn get_infection_day(self) -> i32{
        match self.state {
            State::Infected{} => {
                self.infection_day
            },
            _ => 0
        }
    }
}

#[derive(Copy, Clone)]
pub struct Citizen {
    pub id: i32,
    pub immunity: i32,
    pub home_location: Point,
    pub work_location: Point,
    pub vaccinated: bool,
    pub uses_public_transport: bool,
    pub working: bool,
    pub hospitalized: bool,
    pub transport_location: Point,
    state_machine: StateMachine
}

impl Citizen {
    pub fn new_citizen(id: i32, home_location: Point, work_location: Point, transport_location: Point, uses_public_transport: bool, working: bool) -> Citizen {
        let disease_randomness_factor = Citizen::generate_disease_randomness_factor();

        Citizen{id, immunity: disease_randomness_factor, home_location, work_location, transport_location, vaccinated: false,
            uses_public_transport, working, hospitalized: false, state_machine:StateMachine::new()}
    }

    pub fn get_infection_transmission_rate(&self) -> f64{
        small_pox::get_current_transmission_rate(self.state_machine.get_infection_day() + self.immunity)
    }

    pub fn set_vaccination(&mut self, vaccinated: bool){
        self.vaccinated = vaccinated;
    }

    pub fn infect(&mut self) -> i32{
        match self.state_machine.state {
            State::Susceptible{} => {
                    self.state_machine.state = State::Infected {};
                    1
                },
            _ => {
                panic!("Invalid state transition!")
            }
        }
    }

    pub fn quarantine(&mut self) -> i32{
        match self.state_machine.state{
            State::Infected {} => {
                if small_pox::to_be_quarantined(self.state_machine.infection_day + self.immunity) {
                    self.state_machine.state = State::Quarantined {};
                    return 1
                }
                0
            },
            _ => {
                panic!("Invalid state transition!")
            }
        }
    }

    pub fn decease(&mut self) -> (i32, i32){
        match self.state_machine.state{
            State::Quarantined {} => {
                if self.state_machine.infection_day == small_pox::get_disease_last_day(){
                    self.hospitalized = false;

                    if small_pox::to_be_deceased(){
                        self.state_machine.state = State::Deceased {};
                        return (1,0)
                    }
                    self.state_machine.state = State::Recovered {};
                    return (0, 1)
                }
            },
            _ => {
                panic!("Invalid state transition!")
            }
        }
        (0, 0)
    }

    pub fn is_quarantined(&self) -> bool {
        match self.state_machine.state {
            State::Quarantined {} => {
                true
            },
            _ => false
        }
    }

    pub fn is_susceptible(&self) -> bool {
        match self.state_machine.state {
            State::Susceptible {} => {
                true
            },
            _ => false
        }
    }

    pub fn is_infected(&self) -> bool {
        match self.state_machine.state {
            State::Infected{} => {
                true
            },
            _ => false
        }
    }

    pub fn is_deceased(&self) -> bool {
        match self.state_machine.state {
            State::Deceased {} => {
                true
            },
            _ => false
        }
    }

    pub fn increment_infection_day(&mut self){
        self.state_machine.infection_day += 1;
    }

    pub fn can_move(&self) -> bool{
        if self.is_quarantined() || self.hospitalized || self.is_deceased(){
            return false
        }
        true
    }

    pub fn get_infection_day(&self) -> i32{
        return self.state_machine.get_infection_day();
    }

    fn generate_disease_randomness_factor() -> i32{
        let option = constants::IMMUNITY_RANGE.choose(&mut rand::thread_rng());
        *option.unwrap()
    }
}

pub fn citizen_factory(home_locations: &[Point], work_locations: &Vec<Point>, public_transport_locations: &[Point],
                       percentage_public_transport: f64, working_percentage:f64) -> Vec<Citizen>{
    let mut public_transport_range = thread_rng();
    let mut working_range = thread_rng();
    let mut agent_list = Vec::with_capacity(home_locations.len());
    let mut public_transport_counter = 0;

    for i in 0..home_locations.len(){
        let mut public_transport_location = Point::new(0,0);
        let uses_public_transport_probability = public_transport_range.gen_bool(percentage_public_transport);
        let working_agent = working_range.gen_bool(working_percentage);
        let uses_public_transport = uses_public_transport_probability && working_agent;

        if uses_public_transport{
            public_transport_location = public_transport_locations[0];
            public_transport_counter += 1;
        } else{
            public_transport_location = home_locations[i];
        }

        let agent = Citizen::new_citizen(i as i32, home_locations[i], work_locations[i], public_transport_location, uses_public_transport_probability && working_agent, working_agent);
        agent_list.push(agent);
    }
//TODO: pass number of infected as parameter
    agent_list.last_mut().as_mut().unwrap().infect();
    agent_list
}

#[cfg(test)]
mod tests{
    use super::*;

    fn before_each() -> Vec<Citizen>{
        let home_locations = vec![Point::new(0, 0), Point::new(0, 1), Point::new(1, 0)];
        let work_locations = vec![Point::new(0, 0), Point::new(0, 1), Point::new(1, 0)];
        let public_transport_location = vec![Point::new(0, 0), Point::new(0, 1), Point::new(1, 0)];

        citizen_factory(&home_locations, &work_locations, &public_transport_location, 1.0, 1.0)
    }

    #[test]
    fn generate_citizen(){
        let citizen_list = before_each();
        assert_eq!(citizen_list.len(), 3);
        assert_eq!(citizen_list[1].home_location, Point::new(0, 1));
        assert_eq!(citizen_list[1].uses_public_transport, true);
        assert_eq!(citizen_list[1].working, true);
        assert_eq!(citizen_list.last().unwrap().is_infected(), true);
    }

    #[test]
    fn should_infect(){
        let mut citizen_list = before_each();

        assert_eq!(citizen_list[0].infect(), 1);
    }

    #[test]
    #[should_panic]
    fn should_panic(){
        let mut citizen_list = before_each();

        citizen_list[0].quarantine();
    }
}