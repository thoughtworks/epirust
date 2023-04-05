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
mod commuter;
mod commuters_by_region;

use bincode::deserialize;
use mpi::topology::{SystemCommunicator};
use mpi::traits::{Communicator, Source};
use crate::models::travel_plan::TravelPlan;
use crate::models::custom_types::Hour;
use crate::models::constants;

pub use commuter::Commuter;
pub use commuters_by_region::CommutersByRegion;
use crate::models::commute_plan::CommutePlan;

pub(crate) async fn receive_commuters(
    commute_plan: &CommutePlan,
    simulation_hour: Hour,
    engine_id: &String,
    world: SystemCommunicator,
) -> Vec<Commuter> {
    let mut incoming: Vec<Commuter> = Vec::new();
    let hour = simulation_hour % 24;
    if hour == constants::ROUTINE_TRAVEL_START_TIME || hour == constants::ROUTINE_TRAVEL_END_TIME {
        let expected_incoming_regions = commute_plan.incoming_regions_count(engine_id);
        let mut received_incoming_regions = 0;
        debug!("Receiving commuters from {} regions", expected_incoming_regions);
        while expected_incoming_regions != received_incoming_regions {
            let my_rank = world.rank();
            let mut buffer = vec![0u8; 1024];
            let status = world.any_process().receive_into(&mut buffer[..]);
            let received: CommutersByRegion = deserialize(&buffer[..]).unwrap();
            info!("rank - {:?}, simulation_hour - {}, {:?}, {:?}", my_rank, simulation_hour, received, status);
            // info!("received commuters: {:?}", received);
            trace_commuters(&received, hour);
            incoming.extend(received.get_commuters());
            received_incoming_regions += 1;
        }
    }
    incoming
}

fn trace_commuters(commuters_by_region: &CommutersByRegion, hour: Hour) {
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
