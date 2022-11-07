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
pub struct Infected {
    infection_day: Day,
    pub severity: Severity,
}

impl From<Infected> for State {
    fn from(infected: Infected) -> Self {
        State::Infected(infected)
    }
}

impl Infected {
    pub fn newly_infected(severity: Severity) -> Self {
        Infected { infection_day: 0, severity }
    }

    fn new(infection_day: Day, severity: Severity) -> Self {
        Infected { infection_day, severity }
    }

    pub fn get_infection_day(&self) -> Day {
        self.infection_day
    }

    pub fn update_infection_day(&mut self) {
        self.infection_day += 1
    }

    fn next_state(&self, sim_hr: Hour, disease: &Disease, rng: &mut RandomWrapper) -> State {
        let severity = match self.severity {
            Severity::Pre { at_hour } if sim_hr - at_hour >= disease.get_pre_symptomatic_duration() => {
                let is_severe = rng.get().gen_bool(disease.get_percentage_severe_infected_population());
                if is_severe {
                    Severity::Severe
                } else {
                    Severity::Mild
                }
            }
            severity => severity,
        };

        State::Infected(Infected::new(self.infection_day, severity))
    }

    fn deceased(&self, disease: &Disease, rng: &mut RandomWrapper) -> State {
        match self.severity {
            Severity::Asymptomatic if self.infection_day == constants::ASYMPTOMATIC_LAST_DAY => State::Recovered,
            Severity::Mild if self.infection_day == constants::MILD_INFECTED_LAST_DAY => State::Recovered,
            Severity::Severe if self.infection_day == disease.get_disease_last_day() => {
                if disease.is_to_be_deceased(rng) {
                    State::Deceased
                } else {
                    State::Recovered
                }
            }
            _ => (*self).into(),
        }
    }

    fn is_to_be_hospitalize(&self, disease: &Disease, immunity: i32) -> bool {
        match self.severity {
            Severity::Severe => disease.is_to_be_hospitalized((self.infection_day as i32 + immunity) as Day),
            _ => false,
        }
    }
}

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
    Infected(Infected),
    Recovered,
    Deceased,
}

impl State {
    #[cfg(test)]
    pub fn is_mild_asymptomatic(&self) -> bool {
        matches!(self, State::Infected(Infected { severity: Severity::Asymptomatic, .. }))
    }

    pub fn is_mild_symptomatic(&self) -> bool {
        matches!(self, State::Infected(Infected { severity: Severity::Mild, .. }))
    }

    pub fn is_infected_severe(&self) -> bool {
        matches!(self, State::Infected(Infected { severity: Severity::Severe, .. }))
    }

    pub fn asymptomatic(infection_day: Day) -> State {
        Infected { infection_day, severity: Severity::Asymptomatic }.into()
    }

    pub fn mild_infected(infection_day: Day) -> State {
        Infected { infection_day, severity: Severity::Mild }.into()
    }

    pub fn severe(infection_day: Day) -> State {
        Infected { infection_day, severity: Severity::Severe }.into()
    }

    pub fn expose(at_hour: Hour) -> State {
        State::Exposed { at_hour }
    }

    pub fn is_to_be_hospitalize(&self, disease: &Disease, immunity: i32) -> bool {
        match self {
            State::Infected(infected) => infected.is_to_be_hospitalize(disease, immunity),
            _ => false,
        }
    }

    pub fn next_state(
        &self,
        sim_hr: Hour,
        cell: Point,
        citizen: &Citizen,
        disease: &Disease,
        map: &CitizenLocationMap,
        rng: &mut RandomWrapper,
    ) -> State {
        match self {
            State::Susceptible => self.susceptible_next_state(sim_hr, cell, citizen, disease, map, rng),
            State::Exposed { at_hour } => self.exposed_next_state(*at_hour, sim_hr, disease, rng),
            State::Infected(infected) => infected.next_state(sim_hr, disease, rng),
            // State::Infected(Infected { infection_day, severity: Severity::Pre(pre) }) => pre.next_state(sim_hr, *infection_day, disease, rng),
            state => *state,
        }
    }

    fn exposed_next_state(&self, at_hour: Hour, sim_hr: Hour, disease: &Disease, rng: &mut RandomWrapper) -> State {
        let option = constants::RANGE_FOR_EXPOSED.choose(rng.get());
        let random_factor = *option.unwrap();
        if sim_hr - at_hour >= (disease.get_exposed_duration() as i32 + random_factor) as Hour {
            let symptoms = rng.get().gen_bool(1.0 - disease.get_percentage_asymptomatic_population());
            let severity = if !symptoms { Severity::Asymptomatic } else { Severity::Pre { at_hour: sim_hr } };
            Infected::newly_infected(severity).into()
        } else {
            *self
        }
    }

    fn susceptible_next_state(
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

    pub fn deceased(&self, disease: &Disease, rng: &mut RandomWrapper) -> State {
        if let State::Infected(infected) = self {
            infected.deceased(disease, rng)
        } else {
            *self
        }
    }

    pub fn update_infection_day(&mut self) {
        if let State::Infected(inf) = self {
            inf.update_infection_day()
        }
    }
}
