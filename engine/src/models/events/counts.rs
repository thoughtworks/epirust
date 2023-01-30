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

use crate::citizen::Citizen;
use crate::state_machine::State;
use common::models::custom_types::{Count, Hour};
use log::info;

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
pub struct Counts {
    hour: Hour,
    susceptible: Count,
    exposed: Count,
    infected: Count,
    hospitalized: Count,
    recovered: Count,
    deceased: Count,
}

impl Counts {
    #[cfg(test)]
    pub fn new_test(
        hour: Hour,
        susceptible: Count,
        exposed: Count,
        infected: Count,
        hospitalized: Count,
        recovered: Count,
        deceased: Count,
    ) -> Counts {
        Counts { hour, susceptible, exposed, infected, hospitalized, recovered, deceased }
    }

    pub fn new(susceptible: Count, exposed: Count, infected: Count) -> Counts {
        Counts { hour: 0, susceptible, exposed, infected, hospitalized: 0, recovered: 0, deceased: 0 }
    }

    pub fn get_susceptible(&self) -> Count {
        self.susceptible
    }

    pub fn get_exposed(&self) -> Count {
        self.exposed
    }

    pub fn get_infected(&self) -> Count {
        self.infected
    }

    pub fn get_hospitalized(&self) -> Count {
        self.hospitalized
    }

    pub fn get_recovered(&self) -> Count {
        self.recovered
    }

    pub fn get_deceased(&self) -> Count {
        self.deceased
    }

    pub fn get_hour(&self) -> Hour {
        self.hour
    }

    pub fn update_susceptible(&mut self, count: Count) {
        self.susceptible += count;
    }

    pub fn update_exposed(&mut self, count: Count) {
        self.exposed += count;
    }

    pub fn update_infected(&mut self, count: Count) {
        self.infected += count;
    }

    pub fn update_recovered(&mut self, count: Count) {
        self.recovered += count;
    }

    pub fn update_deceased(&mut self, count: Count) {
        self.deceased += count;
    }

    pub fn update_hospitalized(&mut self, count: Count) {
        self.hospitalized += count;
    }

    pub fn remove_susceptible(&mut self, count: Count) {
        self.susceptible -= count;
    }

    pub fn remove_exposed(&mut self, count: Count) {
        self.exposed -= count;
    }

    pub fn remove_infected(&mut self, count: Count) {
        self.infected -= count;
    }

    pub fn remove_recovered(&mut self, count: Count) {
        self.recovered -= count;
    }

    pub fn increment_hour(&mut self) {
        self.hour += 1;
    }

    pub fn update_counts(&mut self, citizen: &Citizen) {
        match citizen.state_machine.state {
            State::Susceptible { .. } => self.update_susceptible(1),
            State::Exposed { .. } => self.update_exposed(1),
            State::Infected { .. } => {
                if citizen.is_hospitalized() {
                    self.update_hospitalized(1);
                } else {
                    self.update_infected(1)
                }
            }
            State::Recovered { .. } => self.update_recovered(1),
            State::Deceased { .. } => self.update_deceased(1),
        }
    }

    pub fn clear(&mut self) {
        self.susceptible = 0;
        self.exposed = 0;
        self.infected = 0;
        self.hospitalized = 0;
        self.recovered = 0;
        self.deceased = 0;
    }

    pub fn total(&self) -> Count {
        self.susceptible + self.exposed + self.infected + self.hospitalized + self.recovered + self.deceased
    }

    pub fn log(&self) {
        info!(
            "S: {}, E:{}, I: {}, H: {}, R: {}, D: {}",
            self.get_susceptible(),
            self.get_exposed(),
            self.get_infected(),
            self.get_hospitalized(),
            self.get_recovered(),
            self.get_deceased()
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::models::events::counts::Counts;

    #[test]
    fn should_create_counts() {
        let counts = Counts::new(100, 1, 2);
        assert_eq!(counts.susceptible, 100);
        assert_eq!(counts.exposed, 1);
        assert_eq!(counts.infected, 2);
        assert_eq!(counts.hospitalized, 0);
        assert_eq!(counts.recovered, 0);
        assert_eq!(counts.deceased, 0);
        assert_eq!(counts.hour, 0);
    }

    #[test]
    fn should_update_susceptible() {
        let mut counts = Counts::new(100, 1, 2);
        counts.update_susceptible(5);
        assert_eq!(counts.susceptible, 105);
        assert_eq!(counts.exposed, 1);
        assert_eq!(counts.infected, 2);
        assert_eq!(counts.hospitalized, 0);
        assert_eq!(counts.recovered, 0);
        assert_eq!(counts.deceased, 0);
        assert_eq!(counts.hour, 0);
    }

    #[test]
    fn should_update_exposed() {
        let mut counts = Counts::new(100, 1, 0);
        counts.update_exposed(5);
        assert_eq!(counts.susceptible, 100);
        assert_eq!(counts.exposed, 6);
        assert_eq!(counts.infected, 0);
        assert_eq!(counts.hospitalized, 0);
        assert_eq!(counts.recovered, 0);
        assert_eq!(counts.deceased, 0);
        assert_eq!(counts.hour, 0);
    }

    #[test]
    fn should_update_infected() {
        let mut counts = Counts::new(100, 1, 0);
        counts.update_infected(5);
        assert_eq!(counts.susceptible, 100);
        assert_eq!(counts.exposed, 1);
        assert_eq!(counts.infected, 5);
        assert_eq!(counts.hospitalized, 0);
        assert_eq!(counts.recovered, 0);
        assert_eq!(counts.deceased, 0);
        assert_eq!(counts.hour, 0);
    }

    #[test]
    fn should_update_recovered() {
        let mut counts = Counts::new(100, 1, 0);
        counts.update_recovered(5);
        assert_eq!(counts.susceptible, 100);
        assert_eq!(counts.exposed, 1);
        assert_eq!(counts.infected, 0);
        assert_eq!(counts.hospitalized, 0);
        assert_eq!(counts.recovered, 5);
        assert_eq!(counts.deceased, 0);
        assert_eq!(counts.hour, 0);
    }

    #[test]
    fn should_update_deceased() {
        let mut counts = Counts::new(100, 1, 0);
        counts.update_deceased(5);
        assert_eq!(counts.susceptible, 100);
        assert_eq!(counts.exposed, 1);
        assert_eq!(counts.infected, 0);
        assert_eq!(counts.hospitalized, 0);
        assert_eq!(counts.recovered, 0);
        assert_eq!(counts.deceased, 5);
        assert_eq!(counts.hour, 0);
    }

    #[test]
    fn should_update_quarantined() {
        let mut counts = Counts::new(100, 1, 0);
        counts.update_hospitalized(5);
        assert_eq!(counts.susceptible, 100);
        assert_eq!(counts.exposed, 1);
        assert_eq!(counts.infected, 0);
        assert_eq!(counts.hospitalized, 5);
        assert_eq!(counts.recovered, 0);
        assert_eq!(counts.deceased, 0);
        assert_eq!(counts.hour, 0);
    }

    #[test]
    fn should_increment_hour() {
        let mut counts = Counts::new(100, 1, 0);
        counts.increment_hour();
        assert_eq!(counts.susceptible, 100);
        assert_eq!(counts.exposed, 1);
        assert_eq!(counts.infected, 0);
        assert_eq!(counts.hospitalized, 0);
        assert_eq!(counts.recovered, 0);
        assert_eq!(counts.deceased, 0);
        assert_eq!(counts.hour, 1);
    }
}
