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

use rdkafka::producer::{FutureProducer, FutureRecord, DeliveryFuture};
use rdkafka::ClientConfig;
use crate::environment;
use crate::travel_plan::TravellersByRegion;

const TICK_ACKS_TOPIC: &str = "ticks_ack";
pub const TRAVELS_TOPIC: &str = "travels";

pub struct KafkaProducer {
    producer: FutureProducer,
}

impl KafkaProducer {
    pub fn new() -> KafkaProducer {
        let kafka_url = environment::kafka_url();
        KafkaProducer {
            producer: ClientConfig::new()
                .set("bootstrap.servers", kafka_url.as_str())
                .create()
                .expect("Could not create Kafka Producer")
        }
    }

    pub fn send_ack(&mut self, tick: &TickAck) -> DeliveryFuture {
        let tick_string = serde_json::to_string(&tick).unwrap();
        let record: FutureRecord<String, String> = FutureRecord::to(TICK_ACKS_TOPIC)
            .payload(&tick_string);
        self.producer.send(record, 0)
    }

    pub fn send_travellers(&mut self, outgoing: Vec<TravellersByRegion>) -> DeliveryFuture {
        let payload = serde_json::to_string(&outgoing).unwrap();
        debug!("Sending travellers: {}", payload);
        let record: FutureRecord<String, String> = FutureRecord::to(TRAVELS_TOPIC)
            .payload(&payload);
        self.producer.send(record, 0)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TickAck {
    pub engine_id: String,
    pub hour: i32,
    pub terminate: bool
}
