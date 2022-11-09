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
use common::disease::Disease;
use common::models::custom_types::{Day, Hour};
use common::utils::RandomWrapper;
use rand::seq::SliceRandom;
use rand::Rng;

use crate::models::constants;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum Severity {
    Pre { at_hour: Hour },
    Asymptomatic,
    Mild,
    Severe,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum State {
    Susceptible,
    Exposed { at_hour: Hour },
    Infected { infection_day: Day, severity: Severity },
    Recovered,
    Deceased,
}

impl State {
    #[cfg(test)]
    pub fn is_mild_asymptomatic(&self) -> bool {
        matches!(self, State::Infected { severity: Severity::Asymptomatic, .. })
    }

    pub fn is_mild_symptomatic(&self) -> bool {
        matches!(self, State::Infected { severity: Severity::Mild, .. })
    }

    pub fn is_infected_severe(&self) -> bool {
        matches!(self, State::Infected { severity: Severity::Severe, .. })
    }

    pub fn asymptomatic(infection_day: Day) -> State {
        State::Infected { infection_day, severity: Severity::Asymptomatic }
    }

    pub fn mild_infected(infection_day: Day) -> State {
        State::Infected { infection_day, severity: Severity::Mild }
    }

    pub fn severe(infection_day: Day) -> State {
        State::Infected { infection_day, severity: Severity::Severe }
    }

    pub fn expose(at_hour: Hour) -> State {
        State::Exposed { at_hour }
    }

    pub fn is_to_be_hospitalize(&self, disease: &Disease, immunity: i32) -> bool {
        match self {
            State::Infected { infection_day, severity: Severity::Severe } => {
                disease.is_to_be_hospitalized((*infection_day as i32 + immunity) as Day)
            }
            _ => false,
        }
    }

    pub fn on_infected(
        &self,
        sim_hr: Hour,
        infection_day: Day,
        severity: Severity,
        disease: &Disease,
        rng: &mut RandomWrapper,
    ) -> State {
        match severity {
            Severity::Pre { at_hour } if sim_hr - at_hour >= disease.get_pre_symptomatic_duration() => {
                let is_severe = rng.get().gen_bool(disease.get_percentage_severe_infected_population());
                let severity = if is_severe { Severity::Severe } else { Severity::Mild };
                State::Infected { infection_day, severity }
            }
            _ => *self,
        }
    }

    pub fn on_exposed(&self, at_hour: Hour, sim_hr: Hour, disease: &Disease, rng: &mut RandomWrapper) -> State {
        let option = constants::RANGE_FOR_EXPOSED.choose(rng.get());
        let random_factor = *option.unwrap();
        if sim_hr - at_hour >= (disease.get_exposed_duration() as i32 + random_factor) as Hour {
            let symptoms = rng.get().gen_bool(1.0 - disease.get_percentage_asymptomatic_population());
            let severity = if !symptoms { Severity::Asymptomatic } else { Severity::Pre { at_hour: sim_hr } };
            State::Infected { infection_day: 0, severity }
        } else {
            *self
        }
    }

    pub fn on_susceptible(
        &self,
        sim_hr: Hour,
        cell: Point,
        citizen: &Citizen,
        disease: &Disease,
        map: &CitizenLocationMap,
        rng: &mut RandomWrapper,
    ) -> State {
        if !citizen.work_quarantined && !citizen.is_vaccinated() {
            let neighbours = citizen.current_area.get_neighbors_of(cell);

            let neighbor_that_spreads_infection = neighbours
                .filter(|p| map.is_point_in_grid(p))
                .filter_map(|cell| map.get_agent_for(&cell))
                .filter(|agent| agent.state_machine.is_infected() && !agent.is_hospitalized())
                .find(|neighbor| rng.get().gen_bool(neighbor.get_infection_transmission_rate(disease)));

            if neighbor_that_spreads_infection.is_some() {
                return State::Exposed { at_hour: sim_hr };
            }
        };
        *self
    }

    pub fn at_end_of_the_day(&self, disease: &Disease, rng: &mut RandomWrapper) -> State {
        if let State::Infected { infection_day, severity } = self {
            match severity {
                Severity::Asymptomatic if *infection_day == constants::ASYMPTOMATIC_LAST_DAY => State::Recovered,
                Severity::Mild if *infection_day == constants::MILD_INFECTED_LAST_DAY => State::Recovered,
                Severity::Severe if *infection_day == disease.get_disease_last_day() => {
                    if disease.is_to_be_deceased(rng) {
                        State::Deceased
                    } else {
                        State::Recovered
                    }
                }
                _ => *self,
            }
        } else {
            *self
        }
    }

    pub fn update_infection_day(&mut self) {
        if let State::Infected { infection_day, .. } = self {
            *infection_day += 1
        }
    }
}
