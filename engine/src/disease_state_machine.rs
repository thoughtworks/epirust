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

use crate::disease::Disease;
use crate::random_wrapper::RandomWrapper;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum State {
    Susceptible {},
    Infected {},
    Recovered {},
    Deceased {},
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct DiseaseStateMachine {
    pub state: State,
    infection_day: i32,
}

impl DiseaseStateMachine {
    pub fn new() -> Self {
        DiseaseStateMachine {
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

    pub fn infect(&mut self) -> i32 {
        match self.state {
            State::Susceptible {} => {
                self.state = State::Infected {};
                1
            }
            _ => {
                panic!("Invalid state transition!")
            }
        }
    }

    pub fn quarantine(&mut self, disease: &Disease, immunity: i32) -> bool {
        match self.state {
            State::Infected {} =>
                return disease.to_be_quarantined(self.infection_day + immunity),
            _ => {
                panic!("Invalid state transition!")
            }
        }
    }

    pub fn decease(&mut self, rng: &mut RandomWrapper, disease: &Disease) -> (i32, i32) {
        match self.state {
            State::Infected {} => {
                if self.infection_day == disease.get_disease_last_day() {
                    if disease.to_be_deceased(rng) {
                        self.state = State::Deceased {};
                        return (1, 0);
                    }
                    self.state = State::Recovered {};
                    return (0, 1);
                }
            }
            State::Susceptible {} => {
                println!("Susceptible");
            }
            State::Recovered {} => {
                println!("Recovered");
            }
            _ => {
                panic!("Invalid state transition!")
            }
        }
        (0, 0)
    }

    pub fn is_susceptible(&self) -> bool {
        match self.state {
            State::Susceptible {} => {
                true
            }
            _ => false
        }
    }

    pub fn is_infected(&self) -> bool {
        match self.state {
            State::Infected {} => {
                true
            }
            _ => false
        }
    }

    pub fn is_deceased(&self) -> bool {
        match self.state {
            State::Deceased {} => {
                true
            }
            _ => false
        }
    }

    pub fn increment_infection_day(&mut self) {
        self.infection_day += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_initialize() {
        let machine = DiseaseStateMachine::new();

        let result = match machine.state {
            State::Susceptible {} => true,
            _ => false
        };
        assert_eq!(result, true);
        assert_eq!(machine.get_infection_day(), 0);
    }

    #[test]
    fn should_infect() {
        let mut machine = DiseaseStateMachine::new();
        machine.infect();

        let result = match machine.state {
            State::Infected {} => true,
            _ => false
        };

        assert_eq!(result, true);
    }

    #[test]
    #[should_panic]
    fn should_panic() {
        let disease = Disease::init("config/diseases.yaml", &String::from("small_pox"));
        let mut machine = DiseaseStateMachine::new();
        machine.quarantine(&disease, 2);
    }
}