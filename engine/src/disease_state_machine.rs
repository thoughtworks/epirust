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
use common::models::custom_types::{Day, Hour};
use common::utils::RandomWrapper;

use crate::allocation_map::CitizenLocationMap;
use crate::citizen::Citizen;
use crate::geography::Point;
use crate::state_machine::{DiseaseHandler, State};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiseaseStateMachine {
    pub state: State,
}

impl DiseaseStateMachine {
    pub fn new() -> Self {
        DiseaseStateMachine { state: State::Susceptible }
    }

    pub fn get_infection_day(self) -> Day {
        match self.state {
            State::Infected { infection_day, .. } => infection_day,
            _ => 0,
        }
    }

    pub fn expose(&mut self, current_hour: Hour) {
        match self.state {
            State::Susceptible => self.state = State::expose(current_hour),
            _ => {
                panic!("Invalid state transition!")
            }
        }
    }

    pub fn next<T: DiseaseHandler>(
        &self,
        sim_hr: Hour,
        cell: Point,
        citizen: &Citizen,
        map: &CitizenLocationMap,
        rng: &mut RandomWrapper,
        disease_handler: &T,
    ) -> State {
        match self.state {
            State::Susceptible => disease_handler.on_susceptible(sim_hr, cell, citizen, map, rng).unwrap_or(self.state),
            State::Exposed { at_hour } => disease_handler.on_exposed(at_hour, sim_hr, rng).unwrap_or(self.state),
            State::Infected { infection_day, severity } => {
                disease_handler.on_infected(sim_hr, infection_day, severity, rng).unwrap_or(self.state)
            }
            state => state,
        }
    }

    pub fn decease<T: DiseaseHandler>(&mut self, rng: &mut RandomWrapper, disease_handler: &T) {
        let state_op = disease_handler.on_routine_end(&self.state, rng);
        if let Some(state) = state_op {
            self.state = state
        };
    }

    pub(crate) fn is_to_be_hospitalized<T: DiseaseHandler>(&self, immunity: i32, disease_handler: &T) -> bool {
        disease_handler.is_to_be_hospitalize(&self.state, immunity)
    }

    pub fn is_susceptible(&self) -> bool {
        matches!(self.state, State::Susceptible)
    }

    #[cfg(test)]
    pub fn is_exposed(&self) -> bool {
        matches!(self.state, State::Exposed { .. })
    }

    pub fn is_infected(&self) -> bool {
        matches!(self.state, State::Infected { .. })
    }

    pub fn is_symptomatic(&self) -> bool {
        self.state.is_mild_symptomatic() || self.state.is_infected_severe()
    }

    pub fn is_deceased(&self) -> bool {
        matches!(self.state, State::Deceased {})
    }

    pub fn increment_infection_day(&mut self) {
        self.state.update_infection_day();
    }

    // should be called only during initialization
    pub fn set_mild_asymptomatic(&mut self) {
        self.state = State::asymptomatic(1);
    }

    // should be called only during initialization
    pub fn set_mild_symptomatic(&mut self) {
        self.state = State::mild_infected(1);
    }

    // should be called only during initialization
    pub fn set_severe_infected(&mut self) {
        self.state = State::severe(1);
    }

    #[cfg(test)]
    pub fn is_mild_asymptomatic(&self) -> bool {
        self.state.is_mild_asymptomatic()
    }

    pub fn is_mild_symptomatic(&self) -> bool {
        self.state.is_mild_symptomatic()
    }

    pub fn is_infected_severe(&self) -> bool {
        self.state.is_infected_severe()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::disease::Disease;

    #[test]
    fn should_initialize() {
        let machine = DiseaseStateMachine::new();

        let result = matches!(machine.state, State::Susceptible);
        assert!(result);
        assert_eq!(machine.get_infection_day(), 0);
    }

    //Todo: move it into state_machine test
    // #[test]
    // fn should_infect() {
    //     let mut machine = DiseaseStateMachine::new();
    //     let disease = Disease::new(10, 20, 40, 9, 12, 0.025, 0.25, 0.02, 0.3, 0.3, 24, 24);
    //     machine.expose(100);
    //     machine.infect(&mut RandomWrapper::new(), 140, &disease);
    //
    //     let result = matches!(
    //         machine.state,
    //         State::Infected { symptoms: false, severity: InfectionSeverity::Mild {} }
    //             | State::Infected { symptoms: true, severity: InfectionSeverity::Pre { at_hour: 140 } }
    //     );
    //
    //     assert!(result);
    // }

    //Todo: move it into state_machine test
    // #[test]
    // fn should_not_infect() {
    //     let mut machine = DiseaseStateMachine::new();
    //     let disease = Disease::new(10, 20, 40, 9, 12, 0.025, 0.25, 0.02, 0.3, 0.3, 24, 24);
    //
    //     machine.expose(100);
    //     machine.infect(&mut RandomWrapper::new(), 110, &disease);
    //
    //     let result = matches!(machine.state, State::Exposed { .. });
    //
    //     assert!(result);
    // }

    #[test]
    #[should_panic]
    fn should_panic() {
        let disease = Disease::init("config/diseases.yaml", &String::from("small_pox"));
        let machine = DiseaseStateMachine::new();
        machine.is_to_be_hospitalized(2, &disease);
    }

    //Todo: move it into state_machine test
    // #[test]
    // fn should_change_infection_severity() {
    //     let mut machine = DiseaseStateMachine::new();
    //     let disease = Disease::new(10, 20, 40, 9, 12, 0.025, 0.25, 0.02, 0.3, 0.3, 24, 24);
    //     let mut rng = RandomWrapper::new();
    //
    //     machine.state = State::Infected { symptoms: true, severity: InfectionSeverity::Pre { at_hour: 100 } };
    //
    //     machine.change_infection_severity(140, &mut rng, &disease);
    //
    //     let result = match machine.state {
    //         State::Infected { symptoms: true, severity } => !matches!(severity, InfectionSeverity::Pre { .. }),
    //         _ => false,
    //     };
    //
    //     assert!(result);
    // }

    //Todo: move it into state_machine test
    // #[test]
    // fn should_not_change_infection_severity() {
    //     let mut machine = DiseaseStateMachine::new();
    //     let disease = Disease::new(10, 20, 40, 9, 12, 0.025, 0.25, 0.02, 0.3, 0.3, 24, 24);
    //     let mut rng = RandomWrapper::new();
    //
    //     machine.state = State::Infected { symptoms: true, severity: InfectionSeverity::Pre { at_hour: 100 } };
    //
    //     machine.change_infection_severity(120, &mut rng, &disease);
    //
    //     let result = match machine.state {
    //         State::Infected { symptoms: true, severity } => matches!(severity, InfectionSeverity::Pre { at_hour: 100 }),
    //         _ => false,
    //     };
    //
    //     assert!(result);
    // }

    //Todo: move it into state_machine test
    // #[test]
    // fn should_check_if_pre_symptomatic() {
    //     let mut machine = DiseaseStateMachine::new();
    //
    //     machine.state = State::Infected { symptoms: true, severity: InfectionSeverity::Pre { at_hour: 100 } };
    //     assert!(machine.is_pre_symptomatic());
    //
    //     machine.state = State::Infected { symptoms: true, severity: InfectionSeverity::Mild {} };
    //     assert_eq!(machine.is_pre_symptomatic(), false);
    // }

    //Todo: move it into state_machine test

    // #[test]
    // fn should_set_mild_asymptomatic() {
    //     let mut machine = DiseaseStateMachine::new();
    //     machine.set_mild_asymptomatic();
    //     assert_eq!(machine.state, State::Infected { symptoms: false, severity: InfectionSeverity::Mild });
    //     assert_eq!(machine.infection_day, 1);
    // }

    //Todo: move it into state_machine test
    // #[test]
    // fn should_set_mild_symptomatic() {
    //     let mut machine = DiseaseStateMachine::new();
    //     machine.set_mild_symptomatic();
    //     assert_eq!(machine.state, State::Infected { symptoms: true, severity: InfectionSeverity::Mild });
    //     assert_eq!(machine.infection_day, 1);
    // }

    //Todo: move it into state_machine test
    // #[test]
    // fn should_set_severe_infected() {
    //     let mut machine = DiseaseStateMachine::new();
    //     machine.set_severe_infected();
    //     assert_eq!(machine.state, State::Infected { symptoms: true, severity: InfectionSeverity::Severe });
    //     assert_eq!(machine.infection_day, 1);
    // }

    //Todo: move it into state_machine test
    // #[test]
    // fn should_check_if_symptomatic() {
    //     let mut machine = DiseaseStateMachine::new();
    //
    //     machine.state = State::Infected { symptoms: true, severity: InfectionSeverity::Mild };
    //     assert!(machine.is_symptomatic());
    //
    //     machine.state = State::Infected { symptoms: true, severity: InfectionSeverity::Severe };
    //     assert!(machine.is_symptomatic());
    //
    //     machine.state = State::Infected { symptoms: false, severity: InfectionSeverity::Mild };
    //     assert!(!machine.is_symptomatic());
    //
    //     machine.state = State::Infected { symptoms: true, severity: InfectionSeverity::Pre { at_hour: 100 } };
    //     assert!(!machine.is_symptomatic());
    // }
}
