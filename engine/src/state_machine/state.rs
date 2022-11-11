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

use common::disease::Disease;
use common::models::custom_types::{Day, Hour};

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

    pub fn update_infection_day(&mut self) {
        if let State::Infected { infection_day, .. } = self {
            *infection_day += 1
        }
    }
}
