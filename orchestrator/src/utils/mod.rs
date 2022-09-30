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

use serde_json::Value;
use std::fs::File;

pub fn read_simulation_conf(filename: &str) -> String {
    let reader = File::open(filename).unwrap();
    let config: Value = serde_json::from_reader(reader).unwrap();
    let sim = config.as_object().unwrap();
    serde_json::to_string(sim).unwrap()
}

pub fn get_hours(filename: &str) -> i64 {
    let reader = File::open(filename).unwrap();
    let config: Value = serde_json::from_reader(reader).unwrap();
    let sim = config.get("engine_configs").unwrap().as_array().unwrap();
    let hours = sim[0].get("config").unwrap().get("hours");
    hours.unwrap().as_i64().unwrap()
}
