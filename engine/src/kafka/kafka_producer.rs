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

use rdkafka::producer::{BaseRecord, DefaultProducerContext, ThreadedProducer};
use rdkafka::ClientConfig;
use std::time::Instant;

use crate::travel::commute::CommutersByRegion;
use crate::travel::migration::MigratorsByRegion;
use crate::utils::{environment, SendRecord, SendResult};

const TICK_ACKS_TOPIC: &str = "ticks_ack";
pub const MIGRATION_TOPIC: &str = "migration_";
pub const COMMUTE_TOPIC: &str = "commute_";

pub struct KafkaProducer {
    producer: ThreadedProducer<DefaultProducerContext>,
}

impl KafkaProducer {
    pub fn new() -> KafkaProducer {
        let kafka_url = environment::kafka_url();
        KafkaProducer {
            producer: ClientConfig::new()
                .set("bootstrap.servers", kafka_url.as_str())
                .set("message.max.bytes", "104857600")
                .set("compression.type", "snappy")
                .create()
                .expect("Could not create Kafka Producer"),
        }
    }

    pub fn send_ack<'a>(&mut self, tick_string: &'a String) -> SendResult<'a> {
        let record: BaseRecord<String, String> = BaseRecord::to(TICK_ACKS_TOPIC).payload(tick_string);
        self.producer.send(record)
    }

    pub fn send_migrators(&mut self, outgoing: Vec<MigratorsByRegion>) {
        for out_region in outgoing.iter() {
            let payload = serde_json::to_string(out_region).unwrap();
            trace!("Sending migrators: {} to region: {}", payload, out_region.to_engine_id());
            let topic = &*format!("{}{}", MIGRATION_TOPIC, out_region.to_engine_id());
            let record: BaseRecord<String, String> = BaseRecord::to(topic).payload(&payload);
            debug!("sending migrators");
            let start_time = Instant::now();
            self.producer.send_record(record);
            debug!("sent migrators: {}", start_time.elapsed().as_millis());
        }
    }

    pub fn send_commuters(&mut self, outgoing: Vec<CommutersByRegion>) {
        for out_region in outgoing.iter() {
            let payload = serde_json::to_string(out_region).unwrap();
            trace!("Sending commuters: {} to region: {}", payload, out_region.to_engine_id());
            debug!("Sending commuters: {} to region: {}", out_region.commuters.len(), out_region.to_engine_id());
            let topic = &*format!("{}{}", COMMUTE_TOPIC, out_region.to_engine_id());
            let record: BaseRecord<String, String> = BaseRecord::to(topic).payload(&payload);
            debug!("sending commuters");
            let start_time = Instant::now();
            self.producer.send_record(record);
            debug!("sent commuters: {}", start_time.elapsed().as_millis());
        }
    }
}
