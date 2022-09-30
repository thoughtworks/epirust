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

use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::error::KafkaResult;
use rdkafka::message::BorrowedMessage;
use rdkafka::{ClientConfig, Message};

use crate::travel::commute::CommutersByRegion;
use crate::travel::migration::MigratorsByRegion;
use crate::utils::environment;

pub fn start(engine_id: &str, topics: &[&str], prefix: &str) -> StreamConsumer {
    let kafka_url = environment::kafka_url();
    let group_id = format!("{}_{}", prefix, engine_id);
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", kafka_url.as_str())
        .set("group.id", group_id)
        .set("auto.offset.reset", "earliest")
        .set("auto.commit.interval.ms", "1000")
        .set("session.timeout.ms", "120000")
        .set("max.poll.interval.ms", "86400000") //max allowed
        .set("fetch.message.max.bytes", "104857600")
        .create()
        .expect("Consumer creation failed");

    consumer.subscribe(topics).expect("Couldn't subscribe to specified topics");

    consumer
}

pub fn read_commuters(message: Option<KafkaResult<BorrowedMessage>>) -> Option<CommutersByRegion> {
    match message {
        None => None,
        Some(msg) => match msg {
            Err(e) => {
                error!("error occurred: {}", e);
                None
            }
            Ok(borrowed_message) => {
                let str_message = borrowed_message.payload_view::<str>().unwrap().unwrap();
                trace!("Reading Commute Data: {}", str_message);
                Some(parse_commuters(str_message))
            }
        },
    }
}

fn parse_commuters(message: &str) -> CommutersByRegion {
    serde_json::from_str(message).expect("Could not parse commuters")
}

pub fn read_migrators(message: Option<KafkaResult<BorrowedMessage>>) -> Option<MigratorsByRegion> {
    match message {
        None => None,
        Some(msg) => match msg {
            Err(e) => {
                debug!("error occured: {}", e);
                None
            }
            Ok(borrowed_message) => {
                let str_message = borrowed_message.payload_view::<str>().unwrap().unwrap();
                trace!("Reading Migration Data: {}", str_message);
                Some(parse_migrators(str_message))
            }
        },
    }
}

fn parse_migrators(message: &str) -> MigratorsByRegion {
    serde_json::from_str(message).expect("Could not parse migrators")
}
