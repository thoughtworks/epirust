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

use crate::models::custom_types::{validate_percentage, Percentage, Size};
use validator::Validate;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Validate)]
pub struct GeographyParameters {
    pub grid_size: Size,
    #[validate(custom = "validate_percentage")]
    pub hospital_beds_percentage: Percentage,
}

impl GeographyParameters {
    pub fn new(grid_size: Size, hospital_beds_percentage: f64) -> GeographyParameters {
        GeographyParameters { grid_size, hospital_beds_percentage }
    }
}
