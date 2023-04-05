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

use crate::models::custom_types::{validate_percentage, Count, Hour, Percentage};
use validator::Validate;

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
#[serde(rename = "Intervention")]
pub enum InterventionConfig {
    Vaccinate(VaccinateConfig),
    Lockdown(LockdownConfig),
    BuildNewHospital(BuildNewHospitalConfig),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone, Validate)]
pub struct VaccinateConfig {
    pub at_hour: Hour,
    #[validate(custom = "validate_percentage")]
    pub percent: Percentage,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone, Validate)]
pub struct LockdownConfig {
    pub at_number_of_infections: Count,
    #[validate(custom = "validate_percentage")]
    pub essential_workers_population: Percentage,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, Copy, Clone)]
pub struct BuildNewHospitalConfig {
    pub spread_rate_threshold: u32,
}
