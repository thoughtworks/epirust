use rand::Rng;
use rand::seq::IteratorRandom;
use rand::seq::SliceRandom;

use crate::allocation_map::AgentLocationMap;
use crate::constants;
use crate::csv_service::Row;
use crate::disease::small_pox;
use crate::geography::{Area, Grid, Point};
use crate::random_wrapper::RandomWrapper;

#[derive(Copy, Clone, PartialEq)]
pub enum State {
    Susceptible {},
    Infected {},
    Recovered {},
    Deceased {},
}

#[derive(Copy, Clone, PartialEq)]
pub struct StateMachine {
    state: State,
    infection_day: i32,
}

impl StateMachine {
    fn new() -> Self {
        StateMachine {
            state: State::Susceptible {},
            infection_day: 0,
        }
    }

    pub fn get_infection_day(self) -> i32 {
        match self.state {
            State::Infected {} => {
                self.infection_day
            }
            _ => 0
        }
    }
}

#[derive(Copy, Clone)]
pub struct Citizen {
    pub id: i32,
    immunity: i32,
    pub home_location: Point,
    pub work_location: Point,
    vaccinated: bool,
    pub uses_public_transport: bool,
    working: bool,
    hospitalized: bool,
    pub transport_location: Point,
    state_machine: StateMachine,
    quarantined: bool,
}

impl Citizen {
    pub fn new_citizen(id: i32, home_location: Point, work_location: Point, transport_location: Point, uses_public_transport: bool, working: bool, rng: &mut RandomWrapper) -> Citizen {
        let disease_randomness_factor = Citizen::generate_disease_randomness_factor(rng);

        Citizen {
            id,
            immunity: disease_randomness_factor,
            home_location,
            work_location,
            transport_location,
            vaccinated: false,
            uses_public_transport,
            working,
            hospitalized: false,
            state_machine: StateMachine::new(),
            quarantined: false,
        }
    }

    pub fn get_infection_transmission_rate(&self) -> f64 {
        small_pox::get_current_transmission_rate(self.state_machine.get_infection_day() + self.immunity)
    }

    pub fn set_vaccination(&mut self, vaccinated: bool) {
        self.vaccinated = vaccinated;
    }

    pub fn infect(&mut self) -> i32 {
        match self.state_machine.state {
            State::Susceptible {} => {
                self.state_machine.state = State::Infected {};
                1
            }
            _ => {
                panic!("Invalid state transition!")
            }
        }
    }

    pub fn quarantine(&mut self) -> i32 {
        match self.state_machine.state {
            State::Infected {} => {
                if small_pox::to_be_quarantined(self.state_machine.infection_day + self.immunity) {
                    self.quarantined = true;
                    return 1;
                }
                0
            }
            _ => {
                panic!("Invalid state transition!")
            }
        }
    }

    pub fn decease(&mut self, rng: &mut RandomWrapper) -> (i32, i32) {
        match self.state_machine.state {
            State::Infected {} => {
                if self.state_machine.infection_day == small_pox::get_disease_last_day() {
                    self.hospitalized = false;
                    self.quarantined = false;

                    if small_pox::to_be_deceased(rng) {
                        self.state_machine.state = State::Deceased {};
                        return (1, 0);
                    }
                    self.state_machine.state = State::Recovered {};
                    return (0, 1);
                }
            }
            _ => {
                panic!("Invalid state transition!")
            }
        }
        (0, 0)
    }

    pub fn is_quarantined(&self) -> bool {
        self.quarantined
    }

    pub fn is_susceptible(&self) -> bool {
        match self.state_machine.state {
            State::Susceptible {} => {
                true
            }
            _ => false
        }
    }

    pub fn is_infected(&self) -> bool {
        match self.state_machine.state {
            State::Infected {} => {
                true
            }
            _ => false
        }
    }

    pub fn is_deceased(&self) -> bool {
        match self.state_machine.state {
            State::Deceased {} => {
                true
            }
            _ => false
        }
    }

    pub fn increment_infection_day(&mut self) {
        self.state_machine.infection_day += 1;
    }

    pub fn can_move(&self) -> bool {
        if self.is_quarantined() || self.hospitalized || self.is_deceased() {
            return false;
        }
        true
    }

    fn generate_disease_randomness_factor(rng: &mut RandomWrapper) -> i32 {
        let option = constants::IMMUNITY_RANGE.choose(rng.get());
        *option.unwrap()
    }

    pub fn perform_operation(&mut self, cell: Point, simulation_hour: i32, grid: &Grid, map: &AgentLocationMap,
                             counts: &mut Row, rng: &mut RandomWrapper) -> Point {
        self.routine(cell, simulation_hour, grid, map, counts, rng)
    }

    fn routine(&mut self, cell: Point, simulation_hour: i32, grid: &Grid, map: &AgentLocationMap, counts: &mut Row, rng: &mut RandomWrapper) -> Point {
        let mut new_cell = cell;
        match simulation_hour % constants::NUMBER_OF_HOURS {
            constants::ROUTINE_START_TIME => {
                self.update_infection_day();
                new_cell = self.quarantine_all(cell, &grid.hospital, map, counts);
            }
            constants::SLEEP_START_TIME..=constants::SLEEP_END_TIME => {}
            constants::ROUTINE_TRAVEL_START_TIME | constants::ROUTINE_TRAVEL_END_TIME => {
                new_cell = self.goto(self.transport_location, grid.transport_area, map, cell, rng);
                self.update_infection(cell, map, counts, rng);
            }
            constants::ROUTINE_WORK_TIME => {
                new_cell = self.goto(self.work_location, grid.work_area, map, cell, rng);
                self.update_infection(cell, map, counts, rng);
            }
            constants::ROUTINE_WORK_END_TIME => {
                new_cell = self.goto(self.home_location, grid.housing_area, map, cell, rng);
                self.update_infection(cell, map, counts, rng);
            }
            constants::ROUTINE_END_TIME => {
                new_cell = self.deceased(map, cell, counts, rng)
            }
            _ => {
                new_cell = self.move_agent_from(map, cell, rng);
                self.update_infection(cell, map, counts, rng);
            }
        }
        new_cell
    }

    fn update_infection_day(&mut self) {
        if self.is_infected() || self.is_quarantined() {
            self.increment_infection_day();
        }
    }

    fn quarantine_all(&mut self, cell: Point, hospital: &Area, map: &AgentLocationMap, counts: &mut Row) -> Point {
        let mut new_cell = cell;
        if self.is_infected() && !self.is_quarantined() {
            let number_of_quarantined = self.quarantine();
            if number_of_quarantined > 0 {
                new_cell = AgentLocationMap::goto_hospital(map, hospital, cell, self);
                if new_cell != cell {
                    self.hospitalized = true;
                }
                counts.update_quarantined(number_of_quarantined);
                counts.update_infected(-number_of_quarantined);
            }
        }
        new_cell
    }

    fn update_infection(&mut self, cell: Point, map: &AgentLocationMap, counts: &mut Row, rng: &mut RandomWrapper) {
        if self.is_susceptible() && !self.vaccinated {
            let neighbor_that_spreads_infection = cell.neighbor_iterator()
                .filter(|p| map.is_point_in_grid(p))
                .filter_map(|cell| { map.get_agent_for(&cell) })
                .filter(|agent| (agent.is_infected() || agent.is_quarantined()) && !agent.hospitalized)
                .find(|neighbor| rng.get().gen_bool(neighbor.get_infection_transmission_rate()));

            if neighbor_that_spreads_infection.is_some() {
                self.infect();
                counts.update_infected(1);
                counts.update_susceptible(-1);
            }
        }
    }

    fn goto(&mut self, target_location: Point, target_area: Area, map: &AgentLocationMap, cell: Point, rng: &mut RandomWrapper) -> Point {
        if !self.can_move() {
            return cell;
        }
        if self.working {
            let new_cell: Point = target_area.get_neighbors_of(target_location)
                .filter(|p| map.is_cell_vacant(p))
                .choose(rng.get())
                .unwrap_or(self.home_location);
            return map.move_agent(cell, new_cell);
        }
        self.move_agent_from(map, cell, rng)
    }

    fn deceased(&mut self, map: &AgentLocationMap, cell: Point, counts: &mut Row, rng: &mut RandomWrapper) -> Point {
        let mut new_cell = cell;
        if self.is_quarantined() {
            let result = self.decease(rng);
            if result.1 == 1 {
                new_cell = map.move_agent(cell, self.home_location);
            }
            counts.update_deceased(result.0);
            counts.update_recovered(result.1);
            counts.update_quarantined(-(result.0 + result.1));
        }
        new_cell
    }

    fn move_agent_from(&mut self, map: &AgentLocationMap, cell: Point, rng: &mut RandomWrapper) -> Point {
        let new_cell = cell.neighbor_iterator()
            .filter(|p| map.is_point_in_grid(p))
            .filter(|p| map.is_cell_vacant(p))
            .choose(rng.get())
            .unwrap_or(cell);
        map.move_agent(cell, new_cell)
    }
}

pub fn citizen_factory(home_locations: &Vec<Point>, work_locations: &Vec<Point>, public_transport_locations: &Vec<Point>,
                       percentage_public_transport: f64, working_percentage: f64, rng: &mut RandomWrapper) -> Vec<Citizen> {
    let mut agent_list = Vec::with_capacity(home_locations.len());

    for i in 0..home_locations.len() {
        let is_a_working_citizen = rng.get().gen_bool(working_percentage);

        let uses_public_transport = rng.get().gen_bool(percentage_public_transport)
            && is_a_working_citizen
            && i < public_transport_locations.len();

        let public_transport_location: Point = if uses_public_transport { public_transport_locations[i] } else {
            home_locations[i]
        };

        let work_location = if is_a_working_citizen { work_locations[i] } else {
            home_locations[i]
        };

        let agent = Citizen::new_citizen(i as i32, home_locations[i], work_location,
                                         public_transport_location, uses_public_transport, is_a_working_citizen, rng);
        agent_list.push(agent);
    }
//TODO: pass number of infected as parameter
    agent_list.last_mut().as_mut().unwrap().infect();
    agent_list
}

#[cfg(test)]
mod tests {
    use super::*;

    fn before_each() -> Vec<Citizen> {
        let mut rng = RandomWrapper::new();
        let home_locations = vec![Point::new(0, 0), Point::new(0, 1), Point::new(0, 2), Point::new(0, 3)];
        let work_locations = vec![Point::new(1, 0), Point::new(1, 1), Point::new(1, 2), Point::new(1, 3)];
        let public_transport_location = vec![Point::new(2, 0), Point::new(2, 1), Point::new(2, 2), Point::new(2, 3)];

        citizen_factory(&home_locations, &work_locations, &public_transport_location, 0.5, 0.5, &mut rng)
    }

    #[test]
    fn generate_citizen() {
        let mut rng = RandomWrapper::new();
        let home_locations = vec![Point::new(0, 0), Point::new(0, 1), Point::new(0, 2), Point::new(0, 3)];
        let work_locations = vec![Point::new(1, 0), Point::new(1, 1), Point::new(1, 2), Point::new(1, 3)];
        let public_transport_location = vec![Point::new(2, 0), Point::new(2, 1), Point::new(2, 2), Point::new(2, 3)];
        let citizen_list = citizen_factory(&home_locations, &work_locations, &public_transport_location, 0.5, 0.5, &mut rng);

        assert_eq!(citizen_list.len(), 4);
        assert_eq!(citizen_list.last().unwrap().is_infected(), true);

        for citizen in &citizen_list {
            assert!(home_locations.contains(&citizen.home_location));
        }
    }

    #[test]
    fn should_infect() {
        let mut citizen_list = before_each();

        assert_eq!(citizen_list[0].infect(), 1);
    }

    #[test]
    #[should_panic]
    fn should_panic() {
        let mut citizen_list = before_each();

        citizen_list[0].quarantine();
    }
}
