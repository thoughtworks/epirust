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

use rdkafka::consumer::{StreamConsumer, Consumer};
use rdkafka::{ClientConfig, Message};

use rdkafka::error::KafkaResult;
use rdkafka::message::BorrowedMessage;
use crate::environment;
use serde_json::Value;

const TICKS_TOPIC: &str = "ticks";

pub fn start(engine_id: &str) -> StreamConsumer {
    let kafka_url = environment::kafka_url();
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", kafka_url.as_str())
        .set("group.id", engine_id)
        .set("auto.offset.reset", "earliest")
        .create()
        .expect("Consumer creation failed");

    consumer.subscribe(&[TICKS_TOPIC])
        .expect("Couldn't subscribe to specified topics");

    consumer
}

pub fn read(msg: Option<KafkaResult<BorrowedMessage>>) -> Option<i32> {
    match msg {
        None => {
            debug!("End of tick stream");
            None
        }
        Some(m) => {
            let borrowed_message = m.unwrap();
            let parsed_message = borrowed_message.payload_view::<str>().unwrap().unwrap();
            debug!("Tick Data: {}", parsed_message);
            let tick = parse_tick_hour(parsed_message);
            Some(tick)
        }
    }
}

fn parse_tick_hour(message: &str) -> i32 {
    let tick_json: Value = serde_json::from_str(message).unwrap();
    tick_json.get("hour").unwrap().as_i64().expect("Could not get tick as an i64") as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_tick_hour() {
        let json = r#"
        {
            "hour": 1
        }"#;
        assert_eq!(1, parse_tick_hour(json));

        let json = r#"
        {"hour":0,"travel_plan":{"regions":["engine1","engine2"],"matrix":[[0,156],[108,0]]}}
        "#;
        assert_eq!(0, parse_tick_hour(json));
    }
}
