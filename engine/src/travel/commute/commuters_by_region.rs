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

use crate::travel::commute::Commuter;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CommutersByRegion {
    pub(in crate::travel::commute) to_engine_id: String,
    pub commuters: Vec<Commuter>,
}

impl CommutersByRegion {
    pub fn to_engine_id(&self) -> &String {
        &self.to_engine_id
    }

    pub fn get_commuters(self) -> Vec<Commuter> {
        self.commuters
    }
}
