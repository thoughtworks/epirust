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

extern crate csv;

use std::error::Error;

use csv::Writer;
use serde::Serialize;
use std::any::Any;
use std::path::PathBuf;

use crate::listeners::listener::Listener;
use crate::models::events::Counts;
use crate::utils::environment;

pub struct CsvListener {
    output_file_name: String,
    counts: Vec<Counts>,
}

impl CsvListener {
    pub fn new(output_file_name: String) -> CsvListener {
        CsvListener { output_file_name, counts: Vec::new() }
    }
}

pub fn write(file_path: &PathBuf, data: &Vec<impl Serialize>) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(file_path)?;

    for row in data {
        let result = wtr.serialize(row);
        match result {
            Ok(_) => {}
            Err(e) => {
                error!("Error occurred while serializing! {:?}", e);
            }
        }
    }

    wtr.flush()?;
    Ok(())
}

impl Listener for CsvListener {
    fn counts_updated(&mut self, counts: Counts) {
        self.counts.push(counts);
    }

    fn simulation_ended(&mut self) {
        let mut output_path = environment::output_dir();
        output_path.push(&self.output_file_name);

        write(&output_path, &self.counts).expect("Failed to write to file");
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
