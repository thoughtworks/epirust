/*
 * EpiRust
 * Copyright (c) 2020  ThoughtWorks, Inc.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

use rand::Rng;
use rand::seq::IteratorRandom;
use rand::seq::SliceRandom;

use crate::allocation_map::AgentLocationMap;
use crate::constants;
use crate::geography::{Area, Grid, Point};
use crate::random_wrapper::RandomWrapper;
use crate::disease::Disease;
use serde::{Deserializer, Deserialize, de};
use serde::de::Unexpected;
use crate::listeners::events::counts::Counts;
use crate::disease_state_machine::DiseaseStateMachine;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct PopulationRecord {
    //TODO move to a better place
    pub ind: i32,
    pub age: String,
    #[serde(deserialize_with = "bool_from_string")]
    pub working: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub pub_transport: bool,
}

/// Deserialize bool from String with custom value mapping
fn bool_from_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
{
    match String::deserialize(deserializer)?.as_ref() {
        "True" => Ok(true),
        "False" => Ok(false),
        other => Err(de::Error::invalid_value(
            Unexpected::Str(other),
            &"True or False",
        )),
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Citizen {
    pub id: Uuid,
    immunity: i32,
    pub home_location: Area,
    pub work_location: Area,
    vaccinated: bool,
    pub uses_public_transport: bool,
    working: bool,
    hospitalized: bool,
    pub transport_location: Point,
    pub state_machine: DiseaseStateMachine,
    quarantined: bool,
    isolated: bool,
    current_area: Area,
}

impl Citizen {
    pub fn new(home_location: Area, work_location: Area, transport_location: Point,
               uses_public_transport: bool, working: bool, rng: &mut RandomWrapper) -> Citizen {
        Citizen::new_with_id(Uuid::new_v4(), home_location, work_location, transport_location, uses_public_transport,
                             working, rng)
    }

    pub fn new_with_id(id: Uuid, home_location: Area, work_location: Area, transport_location: Point,
                       uses_public_transport: bool, working: bool, rng: &mut RandomWrapper) -> Citizen {
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
            state_machine: DiseaseStateMachine::new(),
            quarantined: false,
            isolated: false,
            current_area: home_location,
        }
    }

    pub fn from_record(record: PopulationRecord, home_location: Area, work_location: Area,
                       transport_location: Point, rng: &mut RandomWrapper) -> Citizen {
        let disease_randomness_factor = Citizen::generate_disease_randomness_factor(rng);

        Citizen {
            id: Uuid::new_v4(),
            immunity: disease_randomness_factor,
            home_location,
            work_location,
            transport_location,
            vaccinated: false,
            uses_public_transport: record.pub_transport,
            working: record.working,
            hospitalized: false,
            state_machine: DiseaseStateMachine::new(),
            quarantined: false,
            isolated: false,
            current_area: home_location,
        }
    }

    pub fn get_infection_transmission_rate(&self, disease: &Disease) -> f64 {
        disease.get_current_transmission_rate(self.state_machine.get_infection_day() + self.immunity)
    }

    pub fn set_vaccination(&mut self, vaccinated: bool) {
        self.vaccinated = vaccinated;
    }

    pub fn is_quarantined(&self) -> bool {
        self.quarantined
    }

    pub fn can_move(&self) -> bool {
        if self.is_quarantined() || self.hospitalized || self.state_machine.is_deceased() || self.isolated {
            return false;
        }
        true
    }

    pub fn set_isolation(&mut self, state: bool) {
        self.isolated = state;
    }

    pub fn is_isolated(&self) -> bool {
        self.isolated
    }

    pub fn is_working(&self) -> bool {
        self.working
    }

    fn generate_disease_randomness_factor(rng: &mut RandomWrapper) -> i32 {
        let option = constants::IMMUNITY_RANGE.choose(rng.get());
        *option.unwrap()
    }

    pub fn perform_operation(&mut self, cell: Point, simulation_hour: i32, grid: &Grid, map: &AgentLocationMap,
                             counts: &mut Counts, rng: &mut RandomWrapper, disease: &Disease) -> Point {
        self.routine(cell, simulation_hour, grid, map, counts, rng, disease)
    }

    fn routine(&mut self, cell: Point, simulation_hour: i32, grid: &Grid, map: &AgentLocationMap, counts: &mut Counts,
               rng: &mut RandomWrapper, disease: &Disease) -> Point {
        let mut new_cell = cell;
        match simulation_hour % constants::NUMBER_OF_HOURS {
            constants::ROUTINE_START_TIME => {
                self.update_infection_day();
                new_cell = self.quarantine_all(cell, &grid.hospital_area, map, counts, disease);
            }
            constants::SLEEP_START_TIME..=constants::SLEEP_END_TIME => {
                self.current_area = grid.housing_area;
            }
            constants::ROUTINE_TRAVEL_START_TIME | constants::ROUTINE_TRAVEL_END_TIME => {
                new_cell = self.goto_area(grid.transport_area, map, cell, rng);
                self.current_area = grid.transport_area;
                self.update_infection(cell, map, counts, rng, disease);
            }
            constants::ROUTINE_WORK_TIME => {
                new_cell = self.goto_area(self.work_location, map, cell, rng);
                self.current_area = grid.work_area;
                self.update_infection(cell, map, counts, rng, disease);
            }
            constants::ROUTINE_WORK_END_TIME => {
                new_cell = self.goto_area(self.home_location, map, cell, rng);
                self.current_area = grid.housing_area;
                self.update_infection(cell, map, counts, rng, disease);
            }
            constants::ROUTINE_END_TIME => {
                new_cell = self.deceased(map, cell, counts, rng, disease)
            }
            _ => {
                new_cell = self.move_agent_from(map, cell, rng);
                self.update_infection(cell, map, counts, rng, disease);
            }
        }
        new_cell
    }

    fn update_infection_day(&mut self) {
        if self.state_machine.is_infected() || self.is_quarantined() {
            self.state_machine.increment_infection_day();
        }
    }

    fn quarantine_all(&mut self, cell: Point, hospital: &Area, map: &AgentLocationMap, counts: &mut Counts,
                      disease: &Disease) -> Point {
        let mut new_cell = cell;
        if self.state_machine.is_infected() && !self.is_quarantined() {
            let number_of_quarantined = self.state_machine.quarantine(disease, self.immunity);
            if number_of_quarantined {
                self.quarantined = true;
                new_cell = AgentLocationMap::goto_hospital(map, hospital, cell, self);
                if new_cell != cell {
                    self.hospitalized = true;
                }
                counts.update_quarantined(1);
                counts.update_infected(-1);
            }
        }
        new_cell
    }

    fn update_infection(&mut self, cell: Point, map: &AgentLocationMap, counts: &mut Counts, rng: &mut RandomWrapper,
                        disease: &Disease) {
        if self.state_machine.is_susceptible() && !self.vaccinated {
            let neighbours = self.current_area.get_neighbors_of(cell);

            let neighbor_that_spreads_infection = neighbours
                .filter(|p| map.is_point_in_grid(p))
                .filter_map(|cell| { map.get_agent_for(&cell) })
                .filter(|agent| (agent.state_machine.is_infected() || agent.is_quarantined()) && !agent.hospitalized)
                .find(|neighbor| rng.get().gen_bool(neighbor.get_infection_transmission_rate(disease)));

            if neighbor_that_spreads_infection.is_some() {
                self.state_machine.infect();
                counts.update_infected(1);
                counts.update_susceptible(-1);
            }
        }
    }

    fn goto_area(&mut self, target_area: Area, map: &AgentLocationMap, cell: Point, rng: &mut RandomWrapper) -> Point {
        if !self.can_move() {
            return cell;
        }
        if self.working {
            let mut new_cell: Point = target_area.get_random_point(rng);
            if !map.is_cell_vacant(&new_cell) {
                new_cell = cell;
            }

            return map.move_agent(cell, new_cell);
        }
        self.move_agent_from(map, cell, rng)
    }

    fn deceased(&mut self, map: &AgentLocationMap, cell: Point, counts: &mut Counts, rng: &mut RandomWrapper,
                disease: &Disease) -> Point {
        let mut new_cell = cell;
        if self.is_quarantined() {
            let result = self.state_machine.decease(rng, disease);
            if result != (0,0) {
                self.hospitalized = false;
                self.quarantined = false;
            }
            if result.1 == 1 {
                new_cell = map.move_agent(cell, self.home_location.get_random_point(rng));
            }
            counts.update_deceased(result.0);
            counts.update_recovered(result.1);
            counts.update_quarantined(-(result.0 + result.1));
        }
        new_cell
    }

    fn move_agent_from(&mut self, map: &AgentLocationMap, cell: Point, rng: &mut RandomWrapper) -> Point {
        let new_cell = self.current_area.get_neighbors_of(cell)
            .filter(|p| map.is_point_in_grid(p))
            .filter(|p| map.is_cell_vacant(p))
            .choose(rng.get())
            .unwrap_or(cell);
        map.move_agent(cell, new_cell)
    }
}

pub fn citizen_factory(number_of_agents: i32, home_locations: &Vec<Area>, work_locations: &Vec<Area>, public_transport_locations: &Vec<Point>,
                       percentage_public_transport: f64, working_percentage: f64, rng: &mut RandomWrapper) -> Vec<Citizen> {
    let mut agent_list = Vec::with_capacity(home_locations.len());

    for i in 0..number_of_agents as usize {
        let is_a_working_citizen = rng.get().gen_bool(working_percentage);

        let total_home_locations = home_locations.len();
        let total_work_locations = work_locations.len();

        let home_location = home_locations[(i % total_home_locations)];
        let work_location = work_locations[(i % total_work_locations)];

        let uses_public_transport = rng.get().gen_bool(percentage_public_transport)
            && is_a_working_citizen
            && i < public_transport_locations.len();
        //TODO: Check the logic - Jayanta
        let public_transport_location: Point = if uses_public_transport { public_transport_locations[i] } else {
            home_location.get_random_point(rng)
        };

        let work_location = if is_a_working_citizen { work_location } else {
            home_location
        };

        let agent = Citizen::new(home_location, work_location, public_transport_location,
                                 uses_public_transport, is_a_working_citizen, rng);
        agent_list.push(agent);
    }
//TODO: pass number of infected as parameter
    agent_list.last_mut().as_mut().unwrap().state_machine.infect();
    agent_list
}

#[cfg(test)]
mod tests {
    use super::*;

    fn before_each() -> Vec<Citizen> {
        let mut rng = RandomWrapper::new();
        let home_locations = vec![Area::new(Point::new(0, 0), Point::new(2, 2)), Area::new(Point::new(3, 0), Point::new(4, 2))];

        let work_locations = vec![Area::new(Point::new(5, 0), Point::new(6, 2)), Area::new(Point::new(7, 0), Point::new(8, 2))];

        let public_transport_location = vec![Point::new(5, 0), Point::new(5, 1), Point::new(5, 2), Point::new(5, 3)];
        citizen_factory(4, &home_locations, &work_locations, &public_transport_location, 0.5, 0.5, &mut rng)
    }

    #[test]
    fn generate_citizen() {
        let citizen_list = before_each();
        let expected_home_locations = vec![Area::new(Point::new(0, 0), Point::new(2, 2)), Area::new(Point::new(3, 0), Point::new(4, 2))];

        assert_eq!(citizen_list.len(), 4);
        assert_eq!(citizen_list.last().unwrap().state_machine.is_infected(), true);

        for citizen in &citizen_list {
            assert!(expected_home_locations.contains(&citizen.home_location));
        }
    }
}
