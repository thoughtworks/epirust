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
use std::any::Any;

use crate::listeners::listener::Listener;
use crate::state_machine::State;
use crate::travel::migration::MigratorsByRegion;
use crate::utils::environment;

#[derive(Serialize, Debug, PartialEq)]
struct CountsByRegion {
    hr: Hour,
    destination: String,
    susceptible: i32,
    exposed: i32,
    infected: i32,
    recovered: i32,
}

impl CountsByRegion {
    fn create_from(hr: Hour, travellers_by_region: &MigratorsByRegion) -> CountsByRegion {
        let mut susceptible = 0;
        let mut exposed = 0;
        let mut infected = 0;
        let mut recovered = 0;
        travellers_by_region.get_migrators_slice().iter().for_each(|traveller| match traveller.state_machine.state {
            State::Susceptible { .. } => susceptible += 1,
            State::Exposed { .. } => exposed += 1,
            State::Infected { .. } => infected += 1,
            State::Recovered { .. } => recovered += 1,
            State::Deceased { .. } => {
                panic!("Deceased citizen should never be travelling!")
            }
        });
        CountsByRegion { hr, destination: travellers_by_region.to_engine_id().clone(), susceptible, exposed, infected, recovered }
    }

    #[cfg(test)]
    fn new(hr: Hour, destination: String, s: i32, e: i32, i: i32, r: i32) -> CountsByRegion {
        CountsByRegion { hr, destination, susceptible: s, exposed: e, infected: i, recovered: r }
    }
}

pub struct TravelCounter {
    counts: Vec<CountsByRegion>,
    output_file_name: String,
}

impl TravelCounter {
    pub fn new(output_file_name: String) -> TravelCounter {
        TravelCounter { counts: Vec::new(), output_file_name }
    }
}

impl Listener for TravelCounter {
    fn simulation_ended(&mut self) {
        let mut output_path = environment::output_dir();
        output_path.push(&self.output_file_name);
        match crate::listeners::csv_service::write(&output_path, &self.counts) {
            Ok(_) => {}
            Err(e) => {
                error!("Failed to serialize outgoing travels: {}", e)
            }
        }
    }

    fn outgoing_migrators_added(&mut self, hr: Hour, travellers: &[MigratorsByRegion]) {
        let counts_by_region: Vec<CountsByRegion> = travellers.iter().map(|t| CountsByRegion::create_from(hr, t)).collect();
        self.counts.extend(counts_by_region);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state_machine::State;
    use crate::travel::migration::Migrator;
    use std::env;

    #[test]
    fn should_add_outgoing_travellers() {
        //dump the output file to a temp location
        env::set_var(environment::EPI_OUTPUT_DIR, env::temp_dir().to_str().unwrap());
        let mut counter = TravelCounter::new("test_travel.csv".to_string());

        let travellers_by_region = vec![create_travellers("engine2"), create_travellers("engine3")];
        counter.outgoing_migrators_added(24, &travellers_by_region);
        counter.outgoing_migrators_added(48, &[create_travellers("engine2")]);
        counter.simulation_ended();

        assert_eq!(3, counter.counts.len());

        let row1 = CountsByRegion::new(24, "engine2".to_string(), 2, 2, 2, 2);
        let row2 = CountsByRegion::new(24, "engine3".to_string(), 2, 2, 2, 2);
        let row3 = CountsByRegion::new(48, "engine2".to_string(), 2, 2, 2, 2);

        assert_eq!(row1, *counter.counts.get(0).unwrap());
        assert_eq!(row2, *counter.counts.get(1).unwrap());
        assert_eq!(row3, *counter.counts.get(2).unwrap());

        //clear the env variable
        env::remove_var(environment::EPI_OUTPUT_DIR);
    }

    fn create_travellers(region: &str) -> MigratorsByRegion {
        let mut travellers = MigratorsByRegion::create(&region.to_string());
        for _i in 0..2 {
            let mut s = Migrator::new();
            s.state_machine.state = State::Susceptible;
            travellers.alloc_citizen(s);

            let mut e = Migrator::new();
            e.state_machine.state = State::expose(10);
            travellers.alloc_citizen(e);

            let mut i = Migrator::new();
            i.state_machine.state = State::mild_infected(0);
            travellers.alloc_citizen(i);

            let mut r = Migrator::new();
            r.state_machine.state = State::Recovered;
            travellers.alloc_citizen(r);
        }

        travellers
    }
}
