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
use crate::models::constants;
use crate::models::custom_types::Hour;
use crate::geography::Point;
use crate::models::events::Tick;
use crate::travel::commute::Commuter;
use crate::travel::commute::CommutersByRegion;

#[derive(Clone, Debug, Deserialize)]
pub struct CommutePlan {
    pub regions: Vec<String>,
    pub matrix: Vec<Vec<u32>>,
}

impl CommutePlan {
    pub fn incoming_regions_count(&self, engine_id: &String) -> u32 {
        let index = self.get_position(engine_id);
        self.column(index).filter(|val| *val > 0).count() as u32
    }

    pub fn get_outgoing(&self, from_region: &String, to_region: &String) -> u32 {
        let from_index = self.get_position(from_region);
        let to_index = self.get_position(to_region);

        let row = self.matrix.get(from_index).unwrap();
        *row.get(to_index).unwrap()
    }

    fn get_position(&self, engine_id: &String) -> usize {
        self.regions.iter().position(|i| i.eq(engine_id)).expect("Could not find region with specified name")
    }

    fn column(&self, index: usize) -> impl Iterator<Item = u32> + '_ {
        self.matrix.iter().map(move |row| *row.get(index).unwrap())
    }

    pub fn get_total_commuters_by_region(&self, from_region: String) -> Vec<(String, u32)> {
        let mut commuters_by_region: Vec<(String, u32)> = vec![];
        for region in &self.regions {
            commuters_by_region.push((region.to_string(), self.get_outgoing(&from_region, region)))
        }
        commuters_by_region
    }

    pub fn get_commuters_by_region(&self, commuters: &Vec<(Point, Commuter)>, simulation_hour: Hour) -> Vec<CommutersByRegion> {
        let mut commuters_by_region: Vec<CommutersByRegion> = Vec::new();
        for region in &self.regions {
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

    pub(crate) async fn receive_commuters(
        &self,
        tick: Option<Tick>,
        message_stream: &mut MessageStream<'_>,
        engine_id: &String,
    ) -> Vec<Commuter> {
        if tick.is_some() {
            let mut incoming: Vec<Commuter> = Vec::new();
            let hour = tick.unwrap().hour() % 24;
            if hour == constants::ROUTINE_TRAVEL_START_TIME || hour == constants::ROUTINE_TRAVEL_END_TIME {
                let expected_incoming_regions = self.incoming_regions_count(engine_id);
                let mut received_incoming_regions = 0;
                debug!("Receiving commuters from {} regions", expected_incoming_regions);
                while expected_incoming_regions != received_incoming_regions {
                    let maybe_msg = CommutersByRegion::receive_commuters_from_region(message_stream, engine_id).await;
                    if let Some(region_incoming) = maybe_msg {
                        if hour == constants::ROUTINE_TRAVEL_START_TIME {
                            trace!(
                                "Travel_start: Received {} commuters from {:?} region",
                                region_incoming.commuters.len(),
                                region_incoming.commuters.get(0).map(|x| x.home_location.location_id.to_string())
                            );
                        }

                        if hour == constants::ROUTINE_TRAVEL_END_TIME {
                            trace!(
                                "Travel_end: Received {} commuters from {:?} region",
                                region_incoming.commuters.len(),
                                region_incoming.commuters.get(0).map(|x| x.work_location.location_id.to_string())
                            )
                        }
                        incoming.extend(region_incoming.get_commuters());
                        received_incoming_regions += 1;
                    }
                }
            }
            incoming
        } else {
            Vec::new()
        }
    }
}
