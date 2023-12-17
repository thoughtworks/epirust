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
use futures::StreamExt;
use opentelemetry::global::ObjectSafeSpan;
use opentelemetry::trace::{FutureExt, TraceContextExt, Tracer};
use opentelemetry::{global, Context, KeyValue};
use rdkafka::consumer::MessageStream;

use common::models::custom_types::Hour;
use common::models::travel_plan::TravelPlan;
use common::models::CommutePlan;

use crate::kafka::kafka_producer::KafkaProducer;
use crate::kafka::{ticks_consumer, travel_consumer};
use crate::models::constants;
use crate::models::events::Tick;
use crate::transport::Transport;
use crate::travel::commute::{trace_commuters, Commuter, CommutersByRegion};
use crate::travel::migration::{EngineMigrationPlan, Migrator, MigratorsByRegion};

pub struct KafkaTransport<'a> {
    engine_id: String,
    producer: KafkaProducer,
    tick_stream: MessageStream<'a>,
    commuter_stream: MessageStream<'a>,
    migration_stream: MessageStream<'a>,
}

impl<'a> KafkaTransport<'a> {
    pub fn new(
        engine_id: String,
        producer: KafkaProducer,
        tick_stream: MessageStream<'a>,
        commuter_stream: MessageStream<'a>,
        migration_stream: MessageStream<'a>,
    ) -> Self {
        KafkaTransport { engine_id, producer, tick_stream, commuter_stream, migration_stream }
    }
}

impl<'a> KafkaTransport<'a> {
    async fn get_tick(&mut self, simulation_hour: Hour) -> Tick {
        let mut tick = self.extract_tick().await;
        let mut tick_hour = tick.hour();
        while tick_hour < simulation_hour {
            tick = self.extract_tick().await;
            tick_hour = tick.hour();
        }
        tick
    }

    async fn extract_tick(&mut self) -> Tick {
        debug!("Start receiving tick");
        let msg = self.tick_stream.next().await;
        let mut maybe_tick = ticks_consumer::read(msg);
        while maybe_tick.is_none() {
            debug!("Retry for Tick");
            let next_msg = self.tick_stream.next().await;
            maybe_tick = ticks_consumer::read(next_msg);
        }
        debug!("Received Tick Successfully");
        maybe_tick.unwrap()
    }
}

unsafe impl Sync for KafkaTransport<'_> {}
unsafe impl Send for KafkaTransport<'_> {}

#[async_trait]
impl<'a> Transport for KafkaTransport<'a> {
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
            let tracer = global::tracer("epirust-trace");
            let mut span = tracer.start("tick_wait_time");
            span.set_attribute(KeyValue::new("hour", simulation_hour.to_string()));
            let cx = Context::current_with_span(span);
            let t = self.get_tick(simulation_hour).with_context(cx).await;
            if t.hour() != simulation_hour {
                panic!("Local hour is {}, but received tick for {}", simulation_hour, t.hour());
            }
            return Some(t);
        }
        None
    }

    async fn send_commuters(&self, simulation_hour: Hour, commuters: Vec<CommutersByRegion>) {
        let hour = simulation_hour % 24;
        if hour == constants::ROUTINE_TRAVEL_START_TIME || hour == constants::ROUTINE_TRAVEL_END_TIME {
            self.producer.send_commuters(commuters);
        }
    }

    async fn send_migrators(&self, simulation_hour: Hour, outgoing: Vec<MigratorsByRegion>) {
        if simulation_hour % 24 == 0 {
            self.producer.send_migrators(outgoing);
        }
    }

    async fn receive_commuters(&mut self, simulation_hour: Hour, commute_plan: &CommutePlan) -> Vec<Commuter> {
        let mut incoming: Vec<Commuter> = Vec::new();
        if simulation_hour == constants::ROUTINE_TRAVEL_START_TIME || simulation_hour == constants::ROUTINE_TRAVEL_END_TIME {
            let expected_incoming_regions = commute_plan.incoming_regions_count(&self.engine_id);
            let mut received_incoming_regions = 0;
            debug!("Receiving commuters from {} regions", expected_incoming_regions);
            while expected_incoming_regions != received_incoming_regions {
                let maybe_msg =
                    CommutersByRegion::receive_commuters_from_region(&mut self.commuter_stream, &self.engine_id).await;
                if let Some(region_incoming) = maybe_msg {
                    trace_commuters(&region_incoming, simulation_hour);
                    incoming.extend(region_incoming.get_commuters());
                    received_incoming_regions += 1;
                }
            }
        }
        incoming
    }

    async fn receive_migrators(&mut self, simulation_hour: Hour, migration_plan: &EngineMigrationPlan) -> Vec<Migrator> {
        if simulation_hour % 24 == 0 {
            let expected_incoming_regions = migration_plan.incoming_regions_count();
            let mut received_incoming_regions = 0;
            debug!("Receiving migrators from {} regions", expected_incoming_regions);
            let mut incoming: Vec<Migrator> = Vec::new();
            while expected_incoming_regions != received_incoming_regions {
                let maybe_msg = travel_consumer::read_migrators(self.migration_stream.next().await);
                if let Some(region_incoming) = maybe_msg {
                    incoming.extend(region_incoming.get_migrators());
                    received_incoming_regions += 1;
                }
            }
            incoming
        } else {
            Vec::new()
        }
    }
}
