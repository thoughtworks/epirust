/*
 * EpiRust
 * Copyright (c) 2022  ThoughtWorks, Inc.
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
use crate::models::constants;
use crate::models::events::{Counts, Tick, TickAck};
use crate::run_mode::RunMode;
use crate::models::custom_types::Hour;
use futures::StreamExt;
use opentelemetry::trace::{FutureExt, Span, TraceContextExt, Tracer};
use opentelemetry::{global, Context, KeyValue};

pub async fn extract_tick() -> Tick {
    debug!("Start receiving tick");
    // let msg = message_stream.next().await;
    // let mut maybe_tick = ticks_consumer::read(msg);
    // while maybe_tick.is_none() {
    //     debug!("Retry for Tick");
    //     let next_msg = message_stream.next().await;
    //     maybe_tick = ticks_consumer::read(next_msg);
    // }
    // debug!("Received Tick Successfully");
    // maybe_tick.unwrap()

    Tick { hour: 0, terminate: false }
}

pub async fn get_tick(simulation_hour: Hour) {
    //TODO: need to implement
}

pub async fn receive_tick(
    simulation_hour: Hour,
    is_commute_enabled: bool,
    is_migration_enabled: bool,
) -> Option<Tick> {
    // let day_hour = simulation_hour % 24;
    // let is_commute_hour = day_hour == constants::ROUTINE_TRAVEL_END_TIME || day_hour == constants::ROUTINE_TRAVEL_START_TIME;
    // let is_migration_hour = day_hour == 0;
    // let receive_tick_for_commute: bool = is_commute_enabled && is_commute_hour;
    // let receive_tick_for_migration: bool = is_migration_enabled && is_migration_hour;
    // if receive_tick_for_commute || receive_tick_for_migration {
    //     let tracer = global::tracer("epirust-trace");
    //     let mut span = tracer.start("tick_wait_time");
    //     span.set_attribute(KeyValue::new("hour", simulation_hour.to_string()));
    //     let cx = Context::current_with_span(span);
    //     let t = get_tick(simulation_hour).with_context(cx).await;
    //     if t.hour() != simulation_hour {
    //         panic!("Local hour is {}, but received tick for {}", simulation_hour, t.hour());
    //     }
    //     return Some(t);
    // }
    None
}

pub fn send_ack(
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
        // if let RunMode::MultiEngine = run_mode {
        //     let ack = TickAck {
        //         engine_id: engine_id.to_string(),
        //         hour: simulation_hour,
        //         counts,
        //         locked_down: lockdown.is_locked_down(),
        //     };
        //     let tick_string = serde_json::to_string(&ack).unwrap();
            //TODO: need to implement

            // let record: BaseRecord<String, String> = BaseRecord::to(TICK_ACKS_TOPIC).payload(tick_string);
            // self.producer.send(record)
        }
    }
// }
