/*
 * EpiRust
 * Copyright (c) 2022  ThoughtWorks, Inc.
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

mod citizen_data;
mod citizen_factory;
mod population_record;
mod work_status;

pub use citizen_data::CitizensData;
pub use citizen_factory::{citizen_factory, set_starting_infections};
pub use population_record::PopulationRecord;
pub use work_status::WorkStatus;

use common::config::TravelPlanConfig;
use common::disease::Disease;
use common::models::custom_types::{Day, Hour, Percentage};
use common::utils::RandomWrapper;
use rand::seq::IteratorRandom;
use rand::seq::SliceRandom;
use rand::Rng;
use serde::Deserialize;
use uuid::Uuid;

use crate::allocation_map::CitizenLocationMap;
use crate::disease_state_machine::DiseaseStateMachine;
use crate::geography::{Area, Grid, Point};
use crate::models::constants;
use crate::state_machine::{DiseaseHandler, State};
use crate::travel::commute::Commuter;
use crate::travel::migration::Migrator;

#[derive(Clone, Debug, Serialize, Deserialize, Copy)]
pub struct Citizen {
    pub id: Uuid,
    immunity: i32,
    pub home_location: Area,
    pub work_location: Area,
    vaccinated: bool,
    pub uses_public_transport: bool,
    hospitalized: bool,
    pub transport_location: Point,
    pub state_machine: DiseaseStateMachine,
    isolated: bool,
    pub current_area: Area,
    work_status: WorkStatus,
    pub work_quarantined: bool,
}

impl Citizen {
    pub fn new(
        home_location: Area,
        work_location: Area,
        transport_location: Point,
        uses_public_transport: bool,
        work_status: WorkStatus,
        rng: &mut RandomWrapper,
    ) -> Citizen {
        Citizen::new_with_id(
            Uuid::new_v4(),
            home_location,
            work_location,
            transport_location,
            uses_public_transport,
            work_status,
            rng,
        )
    }

    pub fn new_with_id(
        id: Uuid,
        home_location: Area,
        work_location: Area,
        transport_location: Point,
        uses_public_transport: bool,
        work_status: WorkStatus,
        rng: &mut RandomWrapper,
    ) -> Citizen {
        let disease_randomness_factor = Citizen::generate_disease_randomness_factor(rng);

        Citizen {
            id,
            immunity: disease_randomness_factor,
            home_location: home_location.clone(),
            work_location,
            transport_location,
            vaccinated: false,
            uses_public_transport,
            hospitalized: false,
            state_machine: DiseaseStateMachine::new(),
            isolated: false,
            current_area: home_location,
            work_status,
            work_quarantined: false,
        }
    }

    pub fn from_migrator(
        migrator: &Migrator,
        home_location: Area,
        work_location: Area,
        transport_location: Point,
        current_area: Area,
    ) -> Citizen {
        Citizen {
            id: migrator.id,
            immunity: migrator.immunity,
            home_location,
            work_location,
            vaccinated: migrator.vaccinated,
            uses_public_transport: migrator.uses_public_transport,
            hospitalized: false,
            transport_location,
            state_machine: migrator.state_machine,
            isolated: false,
            current_area,
            work_status: WorkStatus::NA,
            work_quarantined: false,
        }
    }

    pub fn from_commuter(commuter: &Commuter, transport_location: Point, current_area: Area, work_area: Option<Area>) -> Citizen {
        Citizen {
            id: commuter.id,
            immunity: commuter.immunity,
            home_location: commuter.home_location.clone(),
            work_location: if work_area == None { commuter.work_location.clone() } else { work_area.unwrap() },
            vaccinated: commuter.vaccinated,
            uses_public_transport: commuter.uses_public_transport,
            hospitalized: false,
            transport_location,
            state_machine: commuter.state_machine,
            isolated: false,
            current_area,
            work_status: WorkStatus::Normal,
            work_quarantined: false,
        }
    }

    pub fn from_record(
        record: PopulationRecord,
        home_location: Area,
        work_location: Area,
        transport_location: Point,
        rng: &mut RandomWrapper,
    ) -> Citizen {
        let disease_randomness_factor = Citizen::generate_disease_randomness_factor(rng);
        let work_status = Citizen::derive_work_status(record.working, rng);

        Citizen {
            id: Uuid::new_v4(),
            immunity: disease_randomness_factor,
            home_location: home_location.clone(),
            work_location,
            transport_location,
            vaccinated: false,
            uses_public_transport: record.pub_transport,
            hospitalized: false,
            state_machine: DiseaseStateMachine::new(),
            isolated: false,
            current_area: home_location,
            work_status,
            work_quarantined: false,
        }
    }

    pub fn get_infection_transmission_rate(&self, disease: &Disease) -> Percentage {
        // why is there addition of infection day and immunity
        disease.get_current_transmission_rate((self.state_machine.get_infection_day() as i32 + self.immunity) as Day)
    }

    pub fn get_immunity(&self) -> i32 {
        self.immunity
    }

    pub fn set_vaccination(&mut self, vaccinated: bool) {
        self.vaccinated = vaccinated;
    }

    pub fn set_isolation(&mut self, state: bool) {
        self.isolated = state;
    }

    fn update_infection_dynamics<T: DiseaseHandler>(
        &mut self,
        cell: Point,
        map: &CitizenLocationMap,
        sim_hr: Hour,
        rng: &mut RandomWrapper,
        disease_handler: &T,
    ) {
        self.state_machine.state = self.state_machine.next(sim_hr, cell, self, map, rng, disease_handler);
    }

    fn generate_disease_randomness_factor(rng: &mut RandomWrapper) -> i32 {
        let option = constants::IMMUNITY_RANGE.choose(rng.get());
        *option.unwrap()
    }

    pub fn perform_operation<T: DiseaseHandler>(
        &mut self,
        cell: Point,
        simulation_hour: Hour,
        grid: &Grid,
        map: &CitizenLocationMap,
        rng: &mut RandomWrapper,
        disease_handler: &T,
    ) -> Point {
        self.routine(cell, simulation_hour, grid, map, rng, disease_handler)
    }

    fn routine<T: DiseaseHandler>(
        &mut self,
        cell: Point,
        simulation_hour: Hour,
        grid: &Grid,
        map: &CitizenLocationMap,
        rng: &mut RandomWrapper,
        disease_handler: &T,
    ) -> Point {
        let mut new_cell = cell;

        let current_hour = simulation_hour % constants::NUMBER_OF_HOURS;
        match current_hour {
            constants::ROUTINE_START_TIME => {
                self.state_machine.increment_infection_day();
                new_cell = self.hospitalize(cell, &grid.hospital_area, map, disease_handler);
            }
            constants::SLEEP_START_TIME..=constants::SLEEP_END_TIME => {
                if !self.is_hospital_staff() {
                    self.current_area = self.home_location.clone();
                }
            }
            constants::ROUTINE_END_TIME => new_cell = self.deceased(map, cell, rng, disease_handler),
            _ => {
                new_cell = self.perform_movements(cell, current_hour, simulation_hour, grid, map, rng, disease_handler);
            }
        }
        new_cell
    }

    fn perform_movements<T: DiseaseHandler>(
        &mut self,
        cell: Point,
        hour_of_day: Hour,
        simulation_hr: Hour,
        grid: &Grid,
        map: &CitizenLocationMap,
        rng: &mut RandomWrapper,
        disease_handler: &T,
    ) -> Point {
        let mut new_cell = cell;
        match self.work_status {
            WorkStatus::Normal | WorkStatus::Essential => {
                match hour_of_day {
                    constants::ROUTINE_TRAVEL_START_TIME | constants::ROUTINE_TRAVEL_END_TIME => {
                        if self.uses_public_transport {
                            new_cell = self.goto_area(grid.transport_area.clone(), map, cell, rng);
                            self.current_area = grid.transport_area.clone();
                        } else {
                            new_cell = self.move_agent_from(map, cell, rng);
                        }
                    }
                    constants::ROUTINE_WORK_TIME => {
                        new_cell = self.goto_area(self.work_location.clone(), map, cell, rng);
                        self.current_area = self.work_location.clone();
                    }
                    constants::ROUTINE_WORK_END_TIME => {
                        new_cell = self.goto_area(self.home_location.clone(), map, cell, rng);
                        self.current_area = self.home_location.clone();
                    }
                    _ => new_cell = self.move_agent_from(map, cell, rng),
                }
                self.update_infection_dynamics(new_cell, map, simulation_hr, rng, disease_handler);
            }

            WorkStatus::HospitalStaff { work_start_at } => {
                // info!("simulation_hr : {}, works_starts_at: {}", simulation_hr, work_start_at);
                // why we are subtracting work start hour
                if simulation_hr.saturating_sub(work_start_at) == (constants::HOURS_IN_A_DAY * constants::QUARANTINE_DAYS) {
                    self.work_quarantined = true;
                    return new_cell;
                }

                if simulation_hr.saturating_sub(work_start_at) == (constants::HOURS_IN_A_DAY * constants::QUARANTINE_DAYS * 2) {
                    new_cell = self.goto_area(self.home_location.clone(), map, cell, rng);
                    self.current_area = self.home_location.clone();
                    self.work_status = WorkStatus::HospitalStaff {
                        work_start_at: (simulation_hr + constants::HOURS_IN_A_DAY * constants::QUARANTINE_DAYS),
                    };
                    return new_cell;
                }

                match hour_of_day {
                    constants::ROUTINE_WORK_TIME => {
                        if self.current_area != grid.hospital_area && work_start_at <= simulation_hr {
                            new_cell = self.goto_area(grid.hospital_area.clone(), map, cell, rng);
                            self.current_area = grid.hospital_area.clone();
                            self.work_status = WorkStatus::HospitalStaff { work_start_at: simulation_hr };
                        }
                        self.work_quarantined = false;
                    }
                    constants::ROUTINE_WORK_END_TIME => {
                        self.work_quarantined = true;
                    }
                    _ => {
                        if !self.work_quarantined && self.can_move() {
                            new_cell = self.move_agent_from(map, cell, rng);
                        }
                    }
                }
                self.update_infection_dynamics(new_cell, map, simulation_hr, rng, disease_handler);
            }

            WorkStatus::NA => {
                match hour_of_day {
                    constants::ROUTINE_WORK_TIME => {
                        new_cell = self.goto_area(grid.housing_area.clone(), map, cell, rng);
                        self.current_area = grid.housing_area.clone();
                    }
                    constants::NON_WORKING_TRAVEL_END_TIME => {
                        new_cell = self.goto_area(self.home_location.clone(), map, cell, rng);
                        self.current_area = self.home_location.clone();
                    }

                    _ => {
                        new_cell = self.move_agent_from(map, cell, rng);
                    }
                }
                self.update_infection_dynamics(new_cell, map, simulation_hr, rng, disease_handler);
            }
        }
        new_cell
    }

    fn hospitalize<T: DiseaseHandler>(
        &mut self,
        cell: Point,
        hospital: &Area,
        map: &CitizenLocationMap,
        disease_handler: &T,
    ) -> Point {
        let mut new_cell = cell;
        if !self.hospitalized && self.state_machine.is_to_be_hospitalized(self.immunity, disease_handler) {
            let (is_hospitalized, new_location) = CitizenLocationMap::goto_hospital(map, hospital, cell, self);
            new_cell = new_location;
            self.hospitalized = is_hospitalized;
        }
        new_cell
    }

    fn goto_area(&self, target_area: Area, map: &CitizenLocationMap, cell: Point, rng: &mut RandomWrapper) -> Point {
        //TODO: Refactor - Jayanta
        // If agent is working and current_area is work, target area is home and symptomatic then allow movement
        let mut override_movement = false;

        match self.work_status {
            WorkStatus::Normal | WorkStatus::Essential => {
                if self.work_location.contains(&cell)
                    && target_area == self.home_location
                    && (self.state_machine.is_mild_symptomatic() || self.state_machine.is_infected_severe())
                {
                    override_movement = true;
                }
            }
            _ => {}
        }
        if !self.can_move() && !override_movement {
            return cell;
        }
        if self.is_working() {
            let mut new_cell: Point = target_area.get_random_point(rng);
            if !map.is_cell_vacant(&new_cell) {
                new_cell = cell;
            }

            return map.move_agent(cell, new_cell);
        }
        self.move_agent_from(map, cell, rng)
    }

    fn deceased<T: DiseaseHandler>(
        &mut self,
        map: &CitizenLocationMap,
        cell: Point,
        rng: &mut RandomWrapper,
        disease_handler: &T,
    ) -> Point {
        let mut new_cell = cell;
        self.state_machine.decease(rng, disease_handler);
        if self.state_machine.state == State::Recovered {
            new_cell = map.move_agent(cell, self.home_location.get_random_point(rng));
        }
        if (self.state_machine.state == State::Recovered || self.state_machine.state == State::Deceased) && self.hospitalized {
            self.hospitalized = false;
        }
        new_cell
    }

    fn move_agent_from(&self, map: &CitizenLocationMap, cell: Point, rng: &mut RandomWrapper) -> Point {
        if !self.can_move() {
            return cell;
        }
        let mut current_location = cell;
        if !self.current_area.contains(&cell) {
            current_location = self.current_area.get_random_point(rng);
        }

        let new_cell = self
            .current_area
            .get_neighbors_of(current_location)
            .filter(|p| map.is_point_in_grid(p))
            .filter(|p| map.is_cell_vacant(p))
            .choose(rng.get())
            .unwrap_or(cell);
        map.move_agent(cell, new_cell)
    }

    pub fn assign_essential_worker(&mut self, essential_workers_percentage: f64, rng: &mut RandomWrapper) {
        if let WorkStatus::Normal = self.work_status {
            if rng.get().gen_bool(essential_workers_percentage) {
                self.work_status = WorkStatus::Essential;
            }
        }
    }

    fn derive_work_status(is_working: bool, rng: &mut RandomWrapper) -> WorkStatus {
        if is_working {
            if rng.get().gen_bool(constants::HOSPITAL_STAFF_PERCENTAGE) {
                return WorkStatus::HospitalStaff { work_start_at: constants::ROUTINE_WORK_TIME };
            }
            return WorkStatus::Normal;
        }
        WorkStatus::NA
    }

    fn can_move(&self) -> bool {
        !(self.state_machine.is_symptomatic() || self.hospitalized || self.state_machine.is_deceased() || self.isolated)
    }

    pub fn can_migrate(&self, region_id: &String, simulation_hour: Hour, travel_plan: &TravelPlanConfig) -> bool {
        let start_migration_hour = travel_plan.get_start_migration_hour();
        let end_migration_hour = travel_plan.get_end_migration_hour();

        simulation_hour % 24 == 0
            && simulation_hour > start_migration_hour
            && simulation_hour < end_migration_hour
            && self.work_location.location_id == *region_id
            && self.home_location.location_id == *region_id
            && self.can_move()
    }

    fn is_hospital_staff(&self) -> bool {
        matches!(self.work_status, WorkStatus::HospitalStaff { .. })
    }

    pub fn is_vaccinated(&self) -> bool {
        self.vaccinated
    }

    pub fn is_isolated(&self) -> bool {
        self.isolated
    }

    pub fn is_working(&self) -> bool {
        !matches!(self.work_status, WorkStatus::NA)
    }

    pub fn is_essential_worker(&self) -> bool {
        matches!(self.work_status, WorkStatus::Essential)
    }

    pub fn is_commuter(&self, region_id: &String, simulation_hour: Hour) -> bool {
        (simulation_hour % 24 == constants::ROUTINE_TRAVEL_START_TIME
            && self.can_move()
            && self.work_location.location_id != *region_id)
            || (simulation_hour % 24 == constants::ROUTINE_TRAVEL_END_TIME
                && self.can_move()
                && self.home_location.location_id != *region_id)
    }

    pub fn is_hospitalized(&self) -> bool {
        self.hospitalized
    }

    #[cfg(test)]
    pub fn is_exposed(&self) -> bool {
        self.state_machine.is_exposed()
    }

    #[cfg(test)]
    pub fn is_mild_asymptomatic(&self) -> bool {
        self.state_machine.is_mild_asymptomatic()
    }

    #[cfg(test)]
    pub fn is_mild_symptomatic(&self) -> bool {
        self.state_machine.is_mild_symptomatic()
    }

    #[cfg(test)]
    pub fn is_infected_severe(&self) -> bool {
        self.state_machine.is_infected_severe()
    }
}

#[cfg(test)]
mod test {
    use crate::citizen::work_status::WorkStatus;
    use crate::citizen::Citizen;
    use crate::geography::{Area, Point};
    use common::utils::RandomWrapper;

    #[test]
    fn should_check_if_agent_is_working() {
        let engine_id = "engine1".to_string();

        let home_location = Area::new(&engine_id, Point::new(0, 0), Point::new(10, 10));
        let work_location = Area::new(&engine_id, Point::new(11, 0), Point::new(20, 20));
        let mut rng = RandomWrapper::new();
        let working_citizen =
            Citizen::new(home_location.clone(), work_location.clone(), Point::new(2, 2), false, WorkStatus::Normal, &mut rng);
        let non_working_citizen = Citizen::new(home_location, work_location, Point::new(2, 2), false, WorkStatus::NA, &mut rng);

        assert_eq!(working_citizen.is_working(), true);
        assert_eq!(non_working_citizen.is_working(), false);
    }
}
