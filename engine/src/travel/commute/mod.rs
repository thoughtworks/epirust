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
pub use commuter::Commuter;
pub use commuters_by_region::CommutersByRegion;

use crate::models::constants;

mod commuter;
mod commuters_by_region;

pub fn trace_commuters(commuters_by_region: &CommutersByRegion, hour: Hour) {
    if hour == constants::ROUTINE_TRAVEL_START_TIME {
        trace!(
            "Travel_start: Received {} commuters from {:?} region",
            commuters_by_region.commuters.len(),
            commuters_by_region.commuters.get(0).map(|x| x.home_location.location_id.to_string())
        );
    }
    if hour == constants::ROUTINE_TRAVEL_END_TIME {
        trace!(
            "Travel_end: Received {} commuters from {:?} region",
            commuters_by_region.commuters.len(),
            commuters_by_region.commuters.get(0).map(|x| x.work_location.location_id.to_string())
        )
    }
}
