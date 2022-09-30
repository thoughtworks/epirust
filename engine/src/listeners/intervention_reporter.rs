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
use serde_json::Value;
use std::any::Any;
use std::fs::File;

use crate::interventions::intervention_type::InterventionType;
use crate::listeners::listener::Listener;
use crate::utils::environment;

#[derive(Serialize)]
struct InterventionReport {
    hour: Hour,
    intervention: String,
    data: Value,
}

impl InterventionReport {
    pub fn new(hour: Hour, intervention: String, data: String) -> InterventionReport {
        InterventionReport { hour, intervention, data: serde_json::from_str(data.as_str()).unwrap() }
    }
}

pub struct InterventionReporter {
    interventions: Vec<InterventionReport>,
    output_file_name: String,
}

impl InterventionReporter {
    pub fn new(output_file_name: String) -> InterventionReporter {
        InterventionReporter { interventions: Vec::new(), output_file_name }
    }
}

impl Listener for InterventionReporter {
    fn simulation_ended(&mut self) {
        let mut output_path = environment::output_dir();
        output_path.push(&self.output_file_name);
        let file = File::create(output_path).expect("Failed to create intervention report file");
        serde_json::to_writer(file, &self.interventions).expect("Failed to serialize intervention report");
    }

    fn intervention_applied(&mut self, at_hour: Hour, intervention: &dyn InterventionType) {
        let report = InterventionReport::new(at_hour, intervention.name(), intervention.json_data());
        self.interventions.push(report);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
