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

use crate::geography::Point;
use crate::models::constants;
use crate::travel::commute::Commuter;
use crate::models::custom_types::Hour;

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

    pub fn get_commuters_by_region(
        regions: &[String],
        commuters: &Vec<(Point, Commuter)>,
        simulation_hour: Hour,
    ) -> Vec<CommutersByRegion> {
        let mut commuters_by_region: Vec<CommutersByRegion> = Vec::new();
        for region in regions {
            let mut commuters_for_region: Vec<Commuter> = Vec::new();
            for (_point, commuter) in commuters {
                if simulation_hour % 24 == constants::ROUTINE_TRAVEL_START_TIME && commuter.work_location.location_id == *region {
                    commuters_for_region.push(commuter.clone())
                }
                if simulation_hour % 24 == constants::ROUTINE_TRAVEL_END_TIME && commuter.home_location.location_id == *region {
                    commuters_for_region.push(commuter.clone())
                }
            }
            commuters_by_region.push(CommutersByRegion { to_engine_id: region.clone(), commuters: commuters_for_region })
        }
        commuters_by_region
    }
}
