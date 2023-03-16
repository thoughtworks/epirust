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
use common::models::custom_types::{Day, Hour};
use mockall::*;

pub struct B;

#[automock(type Key=B; type Value=i32;)]
pub trait A {
    type Key;
    type Value;
    fn foo(&self, k: Self::Key) -> Self::Value;
}

pub trait DiseaseHandler {
    fn is_to_be_hospitalize(&self, current_state: &State, immunity: i32) -> bool;

    fn on_infected(&mut self, sim_hr: Hour, infection_day: Day, severity: Severity) -> Option<State>;

    fn on_exposed(&mut self, at_hour: Hour, sim_hr: Hour) -> Option<State>;

    fn on_susceptible(&mut self, sim_hr: Hour, cell: Point, citizen: &Citizen, map: &CitizenLocationMap) -> Option<State>;

    fn on_routine_end(&mut self, current_state: &State) -> Option<State>;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn bb() {
        let mut mock_a = MockA::new();

        mock_a.expect_foo().returning(|x: B| 2);
        assert_eq!(4, mock_a.foo(B));
    }
}
