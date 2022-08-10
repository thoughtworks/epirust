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
use crate::custom_types::Hour;

use crate::environment;

const TICKS_TOPIC: &str = "ticks";

pub fn start(engine_id: &str) -> StreamConsumer {
    let kafka_url = environment::kafka_url();
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", kafka_url.as_str())
        .set("group.id", engine_id)
        .set("auto.offset.reset", "earliest")
        .set("enable.auto.commit", "true") //to avoid consuming duplicate message
        .set("max.poll.interval.ms", "86400000") //max allowed
        .create()
        .expect("Consumer creation failed");

    consumer.subscribe(&[TICKS_TOPIC]).expect("Couldn't subscribe to specified topics");

    consumer
}

pub fn read(msg: Option<KafkaResult<BorrowedMessage>>) -> Option<Tick> {
    match msg {
        None => {
            debug!("End of tick stream");
            None
        }
        Some(m) => match m {
            Err(e) => {
                debug!("error occured: {}", e);
                None
            }
            Ok(borrowed_message) => {
                let str_message = borrowed_message.payload_view::<str>().unwrap().unwrap();
                debug!("Tick Data: {}", str_message);
                Some(parse_tick(str_message))
            }
        },
    }
}

fn parse_tick(message: &str) -> Tick {
    serde_json::from_str(message).expect("Could not parse tick")
}

#[derive(Debug, Copy, Deserialize, PartialEq, Clone)]
pub struct Tick {
    hour: Hour,
    terminate: bool,
}

impl Tick {
    pub fn hour(&self) -> Hour {
        self.hour
    }

    pub fn terminate(&self) -> bool {
        self.terminate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_tick() {
        let json = r#"
        {
            "hour": 1,
            "terminate": false
        }"#;
        let expected = Tick { hour: 1, terminate: false };
        assert_eq!(expected, parse_tick(json));
    }
}
