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

use rdkafka::{ClientConfig, Message};
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::error::KafkaResult;
use rdkafka::message::BorrowedMessage;

use crate::environment;
use crate::kafka_producer::MIGRATION_TOPIC;
use crate::travel_plan::MigratorsByRegion;

pub fn start(engine_id: &str) -> StreamConsumer {
    let kafka_url = environment::kafka_url();
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", kafka_url.as_str())
        .set("group.id", engine_id)
        .set("auto.offset.reset", "earliest")
        .set("max.poll.interval.ms", "86400000") //max allowed
        .create()
        .expect("Consumer creation failed");

    consumer.subscribe(&[MIGRATION_TOPIC])
        .expect("Couldn't subscribe to specified topics");

    consumer
}

pub fn read(message: Option<KafkaResult<BorrowedMessage>>) -> Option<MigratorsByRegion> {
    match message {
        None => { None }
        Some(msg) => {
            match msg {
                Err(e) => {
                    debug!("error occured: {}", e);
                    None
                }
                Ok(borrowed_message) => {
                    let str_message = borrowed_message.payload_view::<str>().unwrap().unwrap();
                    debug!("Reading Migration Data: {}", str_message);
                    Some(parse_migrators(str_message))
                }
            }
        }
    }
}

fn parse_migrators(message: &str) -> MigratorsByRegion {
    serde_json::from_str(message).expect("Could not parse migrators")
}
