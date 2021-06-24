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

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
pub struct Counts {
    hour: i32,
    susceptible: i32,
    exposed: i32,
    infected: i32,
    hospitalized: i32,
    recovered: i32,
    deceased: i32,
}

impl Counts {
    #[cfg(test)]
    pub fn new_test(hour: i32, susceptible: i32, exposed: i32, infected: i32, hospitalized: i32, recovered: i32, deceased: i32) -> Counts {
        Counts { hour, susceptible, exposed, infected, hospitalized, recovered, deceased }
    }

    pub fn new(susceptible: i32, exposed: i32, infected: i32) -> Counts {
        Counts { hour: 0, susceptible, exposed, infected, hospitalized: 0, recovered: 0, deceased: 0 }
    }

    pub fn get_susceptible(&self) -> i32 {
        self.susceptible
    }

    pub fn get_exposed(&self) -> i32 {
        self.exposed
    }

    pub fn get_infected(&self) -> i32 {
        self.infected
    }

    pub fn get_hospitalized(&self) -> i32 {
        self.hospitalized
    }

    pub fn get_recovered(&self) -> i32 {
        self.recovered
    }

    pub fn get_deceased(&self) -> i32 {
        self.deceased
    }

    pub fn get_hour(&self) -> i32 {
        self.hour
    }

    pub fn update_susceptible(&mut self, count: i32) {
        self.susceptible += count;
    }

    pub fn update_exposed(&mut self, count: i32) {
        self.exposed += count;
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

    pub fn update_hospitalized(&mut self, count: i32) {
        self.hospitalized += count;
    }

    pub fn increment_hour(&mut self) {
        self.hour += 1;
    }

    pub fn clear(&mut self) {
        self.susceptible = 0;
        self.exposed = 0;
        self.infected = 0;
        self.hospitalized = 0;
        self.recovered = 0;
        self.deceased = 0;
    }

    pub fn total(&self) -> i32 {
        self.susceptible +
        self.exposed +
        self.infected +
        self.hospitalized +
        self.recovered +
        self.deceased
    }

    pub fn log(&self) {
        info!("S: {}, E:{}, I: {}, H: {}, R: {}, D: {}", self.get_susceptible(), self.get_exposed(),
              self.get_infected(), self.get_hospitalized(), self.get_recovered(),
              self.get_deceased())
    }
}

pub struct CumulativeAverage {
    // cumulative sum of daily new cases.
    infected: Vec<i32>,
    prev_day_counts: Counts,
}

impl CumulativeAverage {
    pub fn new(total_hours: i32, init_counts: &Counts) -> Self {
        let mut infected = Vec::with_capacity((total_hours+1) as usize);
        infected.push(init_counts.infected);
        debug!("initialized cumulative counts {:?}", infected);
        Self { infected, prev_day_counts: *init_counts}
    }

    pub fn update_values(&mut self, counts: &Counts) -> i32{
        let new_cases = counts.infected - self.prev_day_counts.infected + counts.deceased - self.prev_day_counts.deceased + counts.recovered - self.prev_day_counts.recovered + counts.hospitalized - self.prev_day_counts.hospitalized;
        self.infected.push(self.infected.last().unwrap() + new_cases);
        debug!("daily new cases = {}", new_cases);
        self.prev_day_counts = *counts;
        new_cases
    }

    fn get_cumulative_sum(&self, hour:i32) -> i32 {
        *self.infected.get(hour as usize).unwrap_or(&0)
    }

    // start and end hours are inclusive
    pub fn get_infected_moving_average(&self, start_hour: i32, end_hour: i32) -> f64 {
        if start_hour==0 {
            return (self.get_cumulative_sum(end_hour) as f64) / (end_hour as f64)
        }
        ((self.get_cumulative_sum(end_hour) - self.get_cumulative_sum(start_hour)) as f64)/((end_hour-start_hour) as f64)
    }
}
#[cfg(test)]
mod tests {
    use crate::listeners::events::counts::Counts;

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
