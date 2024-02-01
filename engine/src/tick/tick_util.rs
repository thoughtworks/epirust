/*
 * EpiRust
 * Copyright (c) 2024  ThoughtWorks, Inc.
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

use crate::interventions::lockdown::LockdownIntervention;
use crate::kafka::kafka_producer::KafkaProducer;
use crate::models::constants;
use crate::models::events::{Counts, TickAck};

pub fn send_ack(
    engine_id: &str,
    producer: &mut KafkaProducer,
    counts: Counts,
    simulation_hour: Hour,
    lockdown: &LockdownIntervention,
    is_commute_enabled: bool,
    is_migration_enabled: bool,
) {
    let day_hour = simulation_hour % 24;
    let is_commute_hour = day_hour == constants::ROUTINE_TRAVEL_END_TIME || day_hour == constants::ROUTINE_TRAVEL_START_TIME;
    let is_migration_hour = day_hour == 0;
    let received_tick_for_commute: bool = is_commute_enabled && is_commute_hour;
    let received_tick_for_migration: bool = is_migration_enabled && is_migration_hour;

    if simulation_hour == 1 || received_tick_for_commute || received_tick_for_migration {
        // if let RunMode::MultiEngine { engine_id } = run_mode {
        let ack =
            TickAck { engine_id: engine_id.to_string(), hour: simulation_hour, counts, locked_down: lockdown.is_locked_down() };
        let tick_string = serde_json::to_string(&ack).unwrap();
        match producer.send_ack(&tick_string) {
            Ok(_) => {}
            Err(e) => panic!("Failed while sending acknowledgement: {:?}", e.0),
        }
        // }
    }
}
