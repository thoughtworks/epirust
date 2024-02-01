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

use crate::interventions::lockdown::LockdownIntervention;
use crate::kafka::kafka_producer::KafkaProducer;
use crate::models::events::Counts;
use crate::tick::send_ack;
use common::models::custom_types::Hour;

pub trait EngineHandlers {
    fn on_tick_start();
    fn on_tick_end(
        &mut self,
        engine_id: &str,
        counts: Counts,
        simulation_hour: Hour,
        lockdown: &LockdownIntervention,
        is_commute_enabled: bool,
        is_migration_enabled: bool,
    );
}

pub struct NoOpEngineHandlers;

impl EngineHandlers for NoOpEngineHandlers {
    fn on_tick_start() {}

    fn on_tick_end(
        &mut self,
        _engine_id: &str,
        _counts: Counts,
        _simulation_hour: Hour,
        _lockdown: &LockdownIntervention,
        _is_commute_enabled: bool,
        _is_migration_enabled: bool,
    ) {
    }
}

pub struct KafkaImplEngineHandler {
    producer: KafkaProducer,
}

impl KafkaImplEngineHandler {
    pub fn new(producer: KafkaProducer) -> Self {
        KafkaImplEngineHandler { producer }
    }
}

impl EngineHandlers for KafkaImplEngineHandler {
    fn on_tick_start() {}

    fn on_tick_end(
        &mut self,
        engine_id: &str,
        counts: Counts,
        simulation_hour: Hour,
        lockdown: &LockdownIntervention,
        is_commute_enabled: bool,
        is_migration_enabled: bool,
    ) {
        send_ack(engine_id, &mut self.producer, counts, simulation_hour, lockdown, is_commute_enabled, is_migration_enabled);
    }
}
