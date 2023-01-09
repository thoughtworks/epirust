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
use common::models::custom_types::Hour;
use uuid::Uuid;

use crate::citizen::Citizen;
use crate::geography::Point;
use crate::state_machine::State;

#[derive(Serialize)]
pub struct CitizenState {
    citizen_id: Uuid,
    state: String,
    location: Point,
}

impl CitizenState {
    fn state_str(state: State) -> String {
        match state {
            State::Susceptible { .. } => "s".to_string(),
            State::Exposed { .. } => "e".to_string(),
            State::Infected { .. } => "i".to_string(),
            State::Recovered { .. } => "r".to_string(),
            State::Deceased { .. } => "d".to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct CitizenStatesAtHr {
    pub hr: Hour,
    pub citizen_states: Vec<CitizenState>,
}

impl CitizenStatesAtHr {
    pub fn init(population_size: usize) -> CitizenStatesAtHr {
        CitizenStatesAtHr { hr: 1, citizen_states: Vec::with_capacity(population_size) }
    }

    pub fn update(&mut self, citizen: &Citizen, location: &Point) {
        self.citizen_states.push(CitizenState {
            citizen_id: citizen.id,
            state: CitizenState::state_str(citizen.state_machine.state),
            location: *location,
        })
    }

    pub fn next_hour(&mut self) {
        self.hr += 1;
        self.citizen_states.clear();
    }
}

#[cfg(test)]
mod tests {
    use crate::citizen::WorkStatus;
    use crate::geography::Area;
    use common::utils::RandomWrapper;

    use super::*;

    #[test]
    fn citizen_states_at_hr() {
        let mut citizen_states_at_hr = CitizenStatesAtHr::init(10);
        assert_eq!(citizen_states_at_hr.hr, 1);
        assert_eq!(citizen_states_at_hr.citizen_states.len(), 0);

        citizen_states_at_hr.update(&citizen(), &Point::new(1, 1));
        assert_eq!(citizen_states_at_hr.hr, 1);
        assert_eq!(citizen_states_at_hr.citizen_states.len(), 1);

        citizen_states_at_hr.next_hour();
        assert_eq!(citizen_states_at_hr.hr, 2);
        assert_eq!(citizen_states_at_hr.citizen_states.len(), 0);
    }

    fn citizen() -> Citizen {
        let area = Area::new(&"sim_id".to_string(), Point::new(0, 0), Point::new(1, 1));
        Citizen::new(area.clone(), area, Point::new(2, 2), true, WorkStatus::Normal, &mut RandomWrapper::new())
    }
}
