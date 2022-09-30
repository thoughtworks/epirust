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

use serde::de::Unexpected;
use serde::{de, Deserialize, Deserializer};

#[derive(Deserialize)]
pub struct PopulationRecord {
    pub ind: u32,
    pub age: String,
    #[serde(deserialize_with = "bool_from_string")]
    pub working: bool,
    #[serde(deserialize_with = "bool_from_string")]
    pub pub_transport: bool,
}

/// Deserialize bool from String with custom value mapping
fn bool_from_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    match String::deserialize(deserializer)?.as_ref() {
        "True" => Ok(true),
        "False" => Ok(false),
        other => Err(de::Error::invalid_value(Unexpected::Str(other), &"True or False")),
    }
}
