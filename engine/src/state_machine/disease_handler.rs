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

use crate::allocation_map::CitizenLocationMap;
use crate::citizen::Citizen;
use crate::geography::Point;
use crate::state_machine::{Severity, State};
use common::disease::Disease;
use common::models::custom_types::{Day, Hour};
use common::utils::RandomWrapper;

pub trait DiseaseHandler {
    fn is_to_be_hospitalize(&self, current_state: &State, disease: &Disease, immunity: i32) -> bool {
        match current_state {
            State::Infected { infection_day, severity: Severity::Severe } => {
                disease.is_to_be_hospitalized((*infection_day as i32 + immunity) as Day)
            }
            _ => false,
        }
    }

    fn on_infected(
        &self,
        sim_hr: Hour,
        infection_day: Day,
        severity: Severity,
        disease: &Disease,
        rng: &mut RandomWrapper,
    ) -> Option<State>;

    fn on_exposed(&self, at_hour: Hour, sim_hr: Hour, disease: &Disease, rng: &mut RandomWrapper) -> Option<State>;

    fn on_susceptible(
        &self,
        sim_hr: Hour,
        cell: Point,
        citizen: &Citizen,
        disease: &Disease,
        map: &CitizenLocationMap,
        rng: &mut RandomWrapper,
    ) -> Option<State>;

    fn on_routine_end(&self, current_state: &State, disease: &Disease, rng: &mut RandomWrapper) -> Option<State>;
}
