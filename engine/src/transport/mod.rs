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

use async_trait::async_trait;

use common::models::custom_types::Hour;
use common::models::CommutePlan;

use crate::models::events::Tick;
use crate::travel::commute::{Commuter, CommutersByRegion};
use crate::travel::migration::{EngineMigrationPlan, Migrator, MigratorsByRegion};

pub mod engine_handlers;
pub mod kafka_transport;
mod mpi_tag;
pub mod mpi_transport;

#[async_trait]
pub trait Transport {
    async fn receive_tick(&mut self, simulation_hour: Hour, is_commute_enabled: bool, is_migration_enabled: bool)
        -> Option<Tick>;
    async fn send_commuters(&self, simulation_hour: Hour, commuters: Vec<CommutersByRegion>);
    async fn send_migrators(&self, simulation_hour: Hour, outgoing: Vec<MigratorsByRegion>);
    async fn receive_commuters(&mut self, simulation_hour: Hour, commute_plan: &CommutePlan) -> Vec<Commuter>;
    async fn receive_migrators(&mut self, simulation_hour: Hour, migration_plan: &EngineMigrationPlan) -> Vec<Migrator>;
}
