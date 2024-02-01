/*
 * EpiRust
 * Copyright (c) 2023  ThoughtWorks, Inc.
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

use std::collections::HashMap;

use async_trait::async_trait;
use mpi::point_to_point::{Destination, Source};
use mpi::request::RequestCollection;
use mpi::topology::{Communicator, SimpleCommunicator};
use mpi::Rank;
use snap::raw::{Decoder, Encoder};

use common::models::custom_types::Hour;
use common::models::CommutePlan;

use crate::models::constants;
use crate::models::events::Tick;
use crate::transport::mpi_tag::MpiTag;
use crate::transport::Transport;
use crate::travel::commute::{Commuter, CommutersByRegion};
use crate::travel::migration::{EngineMigrationPlan, Migrator, MigratorsByRegion};

pub struct MpiTransport {
    world: SimpleCommunicator,
    engine_ranks: HashMap<String, Rank>,
}

impl MpiTransport {
    pub fn new(regions: &[String]) -> Self {
        let mut engine_ranks: HashMap<String, Rank> = HashMap::new();
        for (i, engine) in regions.iter().enumerate() {
            engine_ranks.insert(engine.clone(), Rank::from(i as u8));
        }
        MpiTransport { world: SimpleCommunicator::world(), engine_ranks }
    }
}

unsafe impl Sync for MpiTransport {}

unsafe impl Send for MpiTransport {}

#[async_trait]
impl Transport for MpiTransport {
    async fn receive_tick(
        &mut self,
        simulation_hour: Hour,
        is_commute_enabled: bool,
        is_migration_enabled: bool,
    ) -> Option<Tick> {
        let day_hour = simulation_hour % 24;
        let is_commute_hour = day_hour == constants::ROUTINE_TRAVEL_END_TIME || day_hour == constants::ROUTINE_TRAVEL_START_TIME;
        let is_migration_hour = day_hour == 0;
        let receive_tick_for_commute: bool = is_commute_enabled && is_commute_hour;
        let receive_tick_for_migration: bool = is_migration_enabled && is_migration_hour;
        if receive_tick_for_commute || receive_tick_for_migration {
            Some(Tick::new(simulation_hour, false))
        } else {
            None
        }
    }

    async fn send_commuters(&self, hour: Hour, commuters: Vec<CommutersByRegion>) {
        let h = hour % 24;
        if h == constants::ROUTINE_TRAVEL_START_TIME || h == constants::ROUTINE_TRAVEL_END_TIME {
            let total_count = self.engine_ranks.iter().len();
            let self_rank = self.world.rank();
            assert_eq!(commuters.len(), total_count);

            let serialized_commuters: Vec<(&Rank, Vec<u8>)> = commuters
                .iter()
                .map(|s| {
                    let rank: &Rank = self.engine_ranks.iter().find(|(x, _)| *x == s.to_engine_id()).unwrap().1;
                    let serialized: Vec<u8> = bincode::serialize(&s).unwrap();
                    let compressed: Vec<u8> = Encoder::new().compress_vec(&serialized[..]).unwrap();
                    let length_of_buffer = compressed.len();
                    let mut compressed_data_with_length = bincode::serialize(&length_of_buffer).unwrap();
                    compressed_data_with_length.extend(compressed);
                    debug!("Rank {self_rank}: commute to {} this much {}", rank, length_of_buffer);
                    (rank, compressed_data_with_length)
                })
                .collect();

            for (&rank, data) in serialized_commuters.iter() {
                let p = self.world.process_at_rank(rank);
                p.buffered_send_with_tag(&data[..], MpiTag::CommuterTag.into());
            }
        }
    }

    async fn send_migrators(&self, hour: Hour, outgoing: Vec<MigratorsByRegion>) {
        debug!("Hi there");
        if hour % 24 == 0 {
            let total_count = self.engine_ranks.iter().len() - 1;
            let self_rank = self.world.rank();
            assert_eq!(outgoing.len(), total_count);

            let serialized_migrators: Vec<(&Rank, Vec<u8>)> = outgoing
                .iter()
                .map(|s| {
                    let rank: &Rank = self.engine_ranks.iter().find(|(x, _)| *x == s.to_engine_id()).unwrap().1;
                    let serialized: Vec<u8> = bincode::serialize(&s).unwrap();
                    let compressed: Vec<u8> = Encoder::new().compress_vec(&serialized[..]).unwrap();
                    let length_of_buffer = compressed.len();
                    let mut compressed_data_with_length = bincode::serialize(&length_of_buffer).unwrap();
                    compressed_data_with_length.extend(compressed);
                    debug!("Rank {self_rank}: migrate to {} this much {}", rank, length_of_buffer);
                    (rank, compressed_data_with_length)
                })
                .collect();

            for (&rank, data) in serialized_migrators.iter() {
                let p = self.world.process_at_rank(rank);
                p.buffered_send_with_tag(&data[..], MpiTag::MigratorTag.into());
            }
        }
    }

    async fn receive_commuters(&mut self, simulation_hour: Hour, _commute_plan: &CommutePlan) -> Vec<Commuter> {
        let h = simulation_hour % 24;
        let mut incoming: Vec<Commuter> = Vec::new();

        if h == constants::ROUTINE_TRAVEL_START_TIME || h == constants::ROUTINE_TRAVEL_END_TIME {
            let total_count = self.engine_ranks.iter().len();
            let self_rank = self.world.rank();

            let buffer = vec![0u8; 1200000];
            let mut result = vec![buffer; total_count];

            mpi::request::multiple_scope(total_count, |scope, coll: &mut RequestCollection<[u8]>| {
                for (index, value) in result.iter_mut().enumerate() {
                    let rank = Rank::from(index as i32);
                    let p = self.world.process_at_rank(rank);
                    let rreq = p.immediate_receive_into_with_tag(scope, &mut value[..], MpiTag::CommuterTag.into());
                    coll.add(rreq);
                }
                let mut recv_count = 0;
                while coll.incomplete() > 0 {
                    let (_u, s, r) = coll.wait_any().unwrap();
                    let length_of_msg: usize = bincode::deserialize::<u32>(&r[0..7]).unwrap() as usize;
                    let decompressed = Decoder::new().decompress_vec(&r[8..length_of_msg + 8]).unwrap();
                    let received: CommutersByRegion = bincode::deserialize(&decompressed[..]).unwrap();
                    trace!(
                        "engine rank: {}, hour: {}, from_rank: {}, received_commuters - {:?}",
                        self_rank,
                        simulation_hour,
                        s.source_rank(),
                        received
                    );
                    let vec1 = received.get_commuters();
                    info!("current rank : {}, source: {}, commuters received: {}", self_rank, s.source_rank(), vec1.len());
                    incoming.extend(vec1);
                    recv_count += 1;
                }
                assert_eq!(recv_count, total_count);
            });
        }
        incoming
    }

    async fn receive_migrators(&mut self, hour: Hour, _migration_plan: &EngineMigrationPlan) -> Vec<Migrator> {
        let mut incoming: Vec<Migrator> = Vec::new();

        if hour % 24 == 0 {
            let total_count = self.engine_ranks.iter().len();
            let self_rank = self.world.rank();

            let buffer = vec![0u8; 1200000];
            let mut result = vec![buffer; total_count];

            mpi::request::multiple_scope(total_count - 1, |scope, coll: &mut RequestCollection<[u8]>| {
                for (index, value) in result.iter_mut().enumerate().filter(|r| r.0 != (self_rank as usize)) {
                    let rank = Rank::from(index as i32);
                    let p = self.world.process_at_rank(rank);
                    let rreq = p.immediate_receive_into_with_tag(scope, &mut value[..], MpiTag::MigratorTag.into());
                    coll.add(rreq);
                }
                let mut recv_count = 0;
                while coll.incomplete() > 0 {
                    let (_u, s, r) = coll.wait_any().unwrap();
                    let length_of_msg: usize = bincode::deserialize::<u32>(&r[0..7]).unwrap() as usize;
                    let decompressed = Decoder::new().decompress_vec(&r[8..length_of_msg + 8]).unwrap();
                    let received: MigratorsByRegion = bincode::deserialize(&decompressed[..]).unwrap();
                    trace!(
                        "engine rank: {}, hour: {}, from_rank: {}, received_commuters - {:?}",
                        self_rank,
                        hour,
                        s.source_rank(),
                        received
                    );
                    let vec1 = received.get_migrators();
                    info!("current rank : {}, source: {}, migrators received: {}", self_rank, s.source_rank(), vec1.len());
                    incoming.extend(vec1);
                    recv_count += 1;
                }
                assert_eq!(recv_count, total_count - 1);
            });
        }
        incoming
    }
}
