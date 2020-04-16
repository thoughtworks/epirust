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

use crate::listeners::events::counts::Counts;
use crate::listeners::listener::Listener;
use std::any::Any;
use crate::disease_state_machine::State;
use crate::environment;
use crate::travel_plan::Traveller;

pub struct TravelCounter {
    hr: i32,
    outgoing_counts: Vec<Counts>,
    current_hr_counts: Counts,
    output_file_name: String,
}

impl TravelCounter {
    pub fn new(output_file_name: String) -> TravelCounter {
        TravelCounter {
            hr: 0,
            outgoing_counts: Vec::new(),
            current_hr_counts: Counts::new(0, 0),
            output_file_name,
        }
    }

    fn prepare_next_hour(&mut self, hr: i32) {
        if self.hr > 0 {
            self.outgoing_counts.push(self.current_hr_counts);
            self.current_hr_counts = Counts::new(0, 0);
        }
        self.current_hr_counts.set_hour(hr);
        self.hr = hr;
    }
}

impl Listener for TravelCounter {
    fn simulation_ended(&mut self) {
        self.prepare_next_hour(-1);
        let mut output_path = environment::output_dir();
        output_path.push(&self.output_file_name);
        match crate::listeners::csv_service::write(&output_path, &self.outgoing_counts) {
            Ok(_) => {},
            Err(e) => { error!("Failed to serialize outgoing travels: {}", e)},
        }
    }

    fn outgoing_traveller_added(&mut self, hr: i32, traveller: &Traveller) {
        if hr > self.hr {
            self.prepare_next_hour(hr);
        }
        match traveller.state_machine.state {
            State::Susceptible { .. } => { self.current_hr_counts.update_susceptible(1) }
            State::Exposed { .. } => { self.current_hr_counts.update_exposed(1) }
            State::Infected { .. } => { self.current_hr_counts.update_infected(1) }
            State::Recovered { .. } => { self.current_hr_counts.update_recovered(1) }
            State::Deceased { .. } => { panic!("Deceased citizen should never be travelling!") }
        }
    }


    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::disease_state_machine::InfectionSeverity;
    use std::env;

    #[test]
    fn should_add_outgoing_travellers() {
        //dump the output file to a temp location
        env::set_var(environment::EPI_OUTPUT_DIR, env::temp_dir().to_str().unwrap());

        let mut counter = TravelCounter::new("test_travel.csv".to_string());
        let travellers = create_travellers();
        travellers.iter().for_each(|t| counter.outgoing_traveller_added(24, t));
        let row1 = Counts::new_test(24, 1, 1, 1, 0, 1, 0);
        assert_eq!(row1, counter.current_hr_counts);

        counter.outgoing_traveller_added(48, &Traveller::new());
        let row2 = Counts::new_test(48, 1, 0, 0, 0, 0, 0);
        assert_eq!(row2, counter.current_hr_counts);

        counter.simulation_ended();
        assert_eq!(2, counter.outgoing_counts.len());
        assert_eq!(row1, *counter.outgoing_counts.get(0).unwrap());
        assert_eq!(row2, *counter.outgoing_counts.get(1).unwrap());

        //clear the env variable
        env::remove_var(environment::EPI_OUTPUT_DIR);
    }

    fn create_travellers() -> Vec<Traveller> {
        let mut s = Traveller::new();
        s.state_machine.state = State::Susceptible {};
        let mut e = Traveller::new();
        e.state_machine.state = State::Exposed { at_hour: 10 };
        let mut i = Traveller::new();
        i.state_machine.state = State::Infected { symptoms: true, severity: InfectionSeverity::Mild {} };
        let mut r = Traveller::new();
        r.state_machine.state = State::Recovered {};

        vec![s, e, i, r]
    }
}
