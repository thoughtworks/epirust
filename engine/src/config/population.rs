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

use validator::Validate;

use crate::models::custom_types::{validate_percentage, Count, Percentage};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum Population {
    Csv(CsvPopulation),
    Auto(AutoPopulation),
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Clone)]
pub struct CsvPopulation {
    pub file: String,
    pub cols: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone, Validate)]
pub struct AutoPopulation {
    pub number_of_agents: Count,
    #[validate(custom = "validate_percentage")]
    pub public_transport_percentage: Percentage,
    #[validate(custom = "validate_percentage")]
    pub working_percentage: Percentage,
}
