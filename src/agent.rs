use rand::Rng;
use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::allocation_map::AgentLocationMap;
use crate::constants;
use crate::csv_service::Row;
use crate::disease::small_pox;
use crate::geography::{Area, Grid};
use crate::geography::point::Point;
use crate::utils;

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
    pub fn new_citizen(id: i32, home_location: Point, work_location: Point, transport_location: Point, uses_public_transport: bool, working: bool) -> Citizen {
        let disease_randomness_factor = Citizen::generate_disease_randomness_factor();

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

    pub fn decease(&mut self) -> (i32, i32) {
        match self.state_machine.state {
            State::Infected {} => {
                if self.state_machine.infection_day == small_pox::get_disease_last_day() {
                    self.hospitalized = false;
                    self.quarantined = false;

                    if small_pox::to_be_deceased() {
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

    fn generate_disease_randomness_factor() -> i32 {
        let option = constants::IMMUNITY_RANGE.choose(&mut rand::thread_rng());
        *option.unwrap()
    }

    pub fn perform_operation(&mut self, cell: Point, simulation_hour: i32, grid: &Grid, map: &AgentLocationMap, counts: &mut Row) -> Point {
        self.routine(cell, simulation_hour, grid, map, counts)
    }

    fn routine(&mut self, cell: Point, simulation_hour: i32, grid: &Grid, map: &AgentLocationMap, counts: &mut Row) -> Point {
        let mut new_cell = cell;
        let mut vacant_cells: Vec<Point> = Vec::with_capacity(constants::NEIGHBORS);
        match simulation_hour % constants::NUMBER_OF_HOURS {
            constants::ROUTINE_START_TIME => {
                self.update_infection_day();
                new_cell = self.quarantine_all(cell, &grid.hospital, map, counts);
            }
            constants::SLEEP_START_TIME..=constants::SLEEP_END_TIME => {}
            constants::ROUTINE_TRAVEL_START_TIME | constants::ROUTINE_TRAVEL_END_TIME => {
                let area_bounds = grid.transport_area.get_neighbors_of(self.transport_location);
                new_cell = self.goto(area_bounds, map, cell, &mut vacant_cells);
                self.update_infection(cell, map, counts);
            }
            constants::ROUTINE_WORK_TIME => {
                let area_bounds = grid.work_area.get_neighbors_of(self.work_location);
                new_cell = self.goto(area_bounds, map, cell, &mut vacant_cells);
                self.update_infection(cell, map, counts);
            }
            constants::ROUTINE_WORK_END_TIME => {
                let area_bounds = grid.housing_area.get_neighbors_of(self.home_location);
                new_cell = self.goto(area_bounds, map, cell, &mut vacant_cells);
                self.update_infection(cell, map, counts);
            }
            constants::ROUTINE_END_TIME => {
                new_cell = self.deceased(map, cell, counts)
            }
            _ => {
                new_cell = self.move_agent_from(map, cell, &mut vacant_cells);
                self.update_infection(cell, map, counts);
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
                new_cell = AgentLocationMap::goto_hospital(map, *hospital, cell, self);
                if new_cell != cell {
                    self.hospitalized = true;
                }
                counts.update_quarantined(number_of_quarantined);
                counts.update_infected(-number_of_quarantined);
            }
        }
        new_cell
    }

    fn update_infection(&mut self, cell: Point, map: &AgentLocationMap, counts: &mut Row) {
        if self.is_susceptible() && !self.vaccinated {
            let infected_neighbors: Vec<&Citizen> = map.get_agents_from(&cell.get_neighbor_cells(map.grid_size)).into_iter().
                filter(|agent| (agent.is_infected() || agent.is_quarantined()) && !agent.hospitalized).collect();
            for neighbor in infected_neighbors {
                let mut rng = thread_rng();
                let transmission_rate = neighbor.get_infection_transmission_rate();
                if rng.gen_bool(transmission_rate) {
//                    println!("Infection rate {}", transmission_rate);
                    self.infect();
                    counts.update_infected(1);
                    counts.update_susceptible(-1);
                    return;
                }
            }
        }
    }

    fn goto(&mut self, area_dimensions: Vec<Point>, map: &AgentLocationMap, cell: Point, vacant_cells: &mut Vec<Point>) -> Point {
        if !self.can_move() {
            return cell;
        }
        if self.working {
            self.get_empty_cells_from(&area_dimensions, map, vacant_cells);
            let new_cell = utils::get_random_element_from(&vacant_cells, self.home_location);
            return map.move_agent(cell, new_cell);
        }
        self.move_agent_from(map, cell, vacant_cells)
    }

    fn deceased(&mut self, map: &AgentLocationMap, cell: Point, counts: &mut Row) -> Point {
        let mut new_cell = cell;
        if self.is_quarantined() {
            let result = self.decease();
            if result.1 == 1 {
                new_cell = map.move_agent(cell, self.home_location);
            }
            counts.update_deceased(result.0);
            counts.update_recovered(result.1);
            counts.update_quarantined(-(result.0 + result.1));
        }
        new_cell
    }

    fn move_agent_from(&mut self, map: &AgentLocationMap, cell: Point, vacant_cells: &mut Vec<Point>) -> Point {
        let neighbor_cells: Vec<Point> = cell.get_neighbor_cells(map.grid_size);
        self.get_empty_cells_from(&neighbor_cells, map, vacant_cells);
        let new_cell: Point = utils::get_random_element_from(&vacant_cells, cell);
        map.move_agent(cell, new_cell)
    }

    fn get_empty_cells_from(&self, neighbors: &Vec<Point>, map: &AgentLocationMap, vacant_cells: &mut Vec<Point>) {
//        neighbors.into_iter().filter(|key| !map.agent_cell.contains_key(*key)).collect()

        for neighbor in neighbors {
            if !map.agent_cell.contains_key(neighbor) {
                vacant_cells.push(*neighbor);
            }
        }
    }
}

pub fn citizen_factory(home_locations: &[Point], work_locations: &Vec<Point>, public_transport_locations: &[Point],
                       percentage_public_transport: f64, working_percentage: f64) -> Vec<Citizen> {
    let mut public_transport_range = thread_rng();
    let mut working_range = thread_rng();
    let mut agent_list = Vec::with_capacity(home_locations.len());

    for i in 0..home_locations.len() {
        let uses_public_transport_probability = public_transport_range.gen_bool(percentage_public_transport);
        let working_agent = working_range.gen_bool(working_percentage);
        let uses_public_transport = uses_public_transport_probability && working_agent;

        let public_transport_location: Point = if uses_public_transport { public_transport_locations[0] } else {
            home_locations[i]
        };

        let agent = Citizen::new_citizen(i as i32, home_locations[i], work_locations[i], public_transport_location, uses_public_transport_probability && working_agent, working_agent);
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
        let home_locations = vec![Point::new(0, 0), Point::new(0, 1), Point::new(1, 0)];
        let work_locations = vec![Point::new(0, 0), Point::new(0, 1), Point::new(1, 0)];
        let public_transport_location = vec![Point::new(0, 0), Point::new(0, 1), Point::new(1, 0)];

        citizen_factory(&home_locations, &work_locations, &public_transport_location, 1.0, 1.0)
    }

    #[test]
    fn generate_citizen() {
        let citizen_list = before_each();
        assert_eq!(citizen_list.len(), 3);
        assert_eq!(citizen_list[1].home_location, Point::new(0, 1));
        assert_eq!(citizen_list[1].uses_public_transport, true);
        assert_eq!(citizen_list[1].working, true);
        assert_eq!(citizen_list.last().unwrap().is_infected(), true);
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
