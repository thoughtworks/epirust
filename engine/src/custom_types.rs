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

use validator::ValidationError;

pub type Hour = u32;
pub type Count = u32;
pub type Day = u32;
pub type Size = u32;
pub type CoOrdinate = i32;
pub type Percentage = f64;

pub fn validate_percentage(value: &f64) -> Result<(), ValidationError> {
    if value < &0.0 && value > &1.0 {
        return Err(ValidationError::new("percentage value needs to be between 0 to 1"));
    }
    Ok(())
}
