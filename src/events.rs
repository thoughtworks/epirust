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

use crate::geography::Point;
use std::any::Any;
use crate::agent::Citizen;

#[derive(Serialize, Copy, Clone)]
pub struct Counts {
    hour: i32,
    susceptible: i32,
    infected: i32,
    quarantined: i32,
    recovered: i32,
    deceased: i32,
}

impl Counts {
    pub fn new(population: i32, infected: i32) -> Counts {
        Counts { hour: 0, susceptible: population, infected, quarantined: 0, recovered: 0, deceased: 0 }
    }

    pub fn get_infected(&self) -> i32 {
        self.infected
    }

    pub fn get_quarantined(&self) -> i32 {
        self.quarantined
    }

    pub fn get_hour(&self) -> i32 {
        self.hour
    }

    pub fn update_susceptible(&mut self, count: i32) {
        self.susceptible += count;
    }

    pub fn update_infected(&mut self, count: i32) {
        self.infected += count;
    }

    pub fn update_recovered(&mut self, count: i32) {
        self.recovered += count;
    }

    pub fn update_deceased(&mut self, count: i32) {
        self.deceased += count;
    }

    pub fn update_quarantined(&mut self, count: i32) {
        self.quarantined += count;
    }

    pub fn increment_hour(&mut self) {
        self.hour += 1;
    }
}

pub trait Listener {
    fn counts_updated(&mut self, counts: Counts);
    fn simulation_ended(&mut self);
    fn citizen_got_infected(&mut self, cell: &Point) {}
    fn citizen_state_updated(&mut self, hr: i32, citizen: &Citizen, location: &Point) {}
    fn as_any(&self) -> &dyn Any;
}

#[cfg(test)]
mod tests {
    use crate::events::Counts;

    #[test]
    fn should_create_counts() {
        let counts = Counts::new(100, 1);
        assert_eq!(counts.susceptible, 100);
        assert_eq!(counts.infected, 1);
        assert_eq!(counts.quarantined, 0);
        assert_eq!(counts.recovered, 0);
        assert_eq!(counts.deceased, 0);
        assert_eq!(counts.hour, 0);
    }

    #[test]
    fn should_update_susceptible() {
        let mut counts = Counts::new(100, 1);
        counts.update_susceptible(5);
        assert_eq!(counts.susceptible, 105);
        assert_eq!(counts.infected, 1);
        assert_eq!(counts.quarantined, 0);
        assert_eq!(counts.recovered, 0);
        assert_eq!(counts.deceased, 0);
        assert_eq!(counts.hour, 0);
    }

    #[test]
    fn should_update_infected() {
        let mut counts = Counts::new(100, 1);
        counts.update_infected(5);
        assert_eq!(counts.susceptible, 100);
        assert_eq!(counts.infected, 6);
        assert_eq!(counts.quarantined, 0);
        assert_eq!(counts.recovered, 0);
        assert_eq!(counts.deceased, 0);
        assert_eq!(counts.hour, 0);
    }

    #[test]
    fn should_update_recovered() {
        let mut counts = Counts::new(100, 1);
        counts.update_recovered(5);
        assert_eq!(counts.susceptible, 100);
        assert_eq!(counts.infected, 1);
        assert_eq!(counts.quarantined, 0);
        assert_eq!(counts.recovered, 5);
        assert_eq!(counts.deceased, 0);
        assert_eq!(counts.hour, 0);
    }

    #[test]
    fn should_update_deceased() {
        let mut counts = Counts::new(100, 1);
        counts.update_deceased(5);
        assert_eq!(counts.susceptible, 100);
        assert_eq!(counts.infected, 1);
        assert_eq!(counts.quarantined, 0);
        assert_eq!(counts.recovered, 0);
        assert_eq!(counts.deceased, 5);
        assert_eq!(counts.hour, 0);
    }

    #[test]
    fn should_update_quarantined() {
        let mut counts = Counts::new(100, 1);
        counts.update_quarantined(5);
        assert_eq!(counts.susceptible, 100);
        assert_eq!(counts.infected, 1);
        assert_eq!(counts.quarantined, 5);
        assert_eq!(counts.recovered, 0);
        assert_eq!(counts.deceased, 0);
        assert_eq!(counts.hour, 0);
    }

    #[test]
    fn should_increment_hour() {
        let mut counts = Counts::new(100, 1);
        counts.increment_hour();
        assert_eq!(counts.susceptible, 100);
        assert_eq!(counts.infected, 1);
        assert_eq!(counts.quarantined, 0);
        assert_eq!(counts.recovered, 0);
        assert_eq!(counts.deceased, 0);
        assert_eq!(counts.hour, 1);
    }
}
