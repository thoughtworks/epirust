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

use rdkafka::consumer::MessageStream;
use crate::kafka::travel_consumer;
use crate::travel::commute::Commuter;
use futures::StreamExt;

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

    pub(crate) async fn receive_commuters_from_region(
        message_stream: &mut MessageStream<'_>,
        engine_id: &String,
    ) -> Option<CommutersByRegion> {
        let msg = message_stream.next().await;
        let mut maybe_commuters = travel_consumer::read_commuters(msg);
        while maybe_commuters.is_none()
            || (maybe_commuters.as_ref().unwrap().commuters.is_empty()
                && maybe_commuters.as_ref().unwrap().to_engine_id() == engine_id)
        {
            let next_msg = message_stream.next().await;
            maybe_commuters = travel_consumer::read_commuters(next_msg);
        }
        maybe_commuters
    }
}
