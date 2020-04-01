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


#[macro_use]
extern crate serde_derive;

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::ops::Range;

use futures::StreamExt;
use rdkafka::error::KafkaError;
use rdkafka::Message;
use rdkafka::message::BorrowedMessage;

use crate::kafka_consumer::KafkaConsumer;
use crate::kafka_producer::KafkaProducer;
use crate::ticks::{TickAck, TickAcks};

mod kafka_producer;
mod kafka_consumer;
mod ticks;

#[tokio::main]
async fn main() {
    let engines = vec!["engine1", "engine2"];
    let hours = 0..10000;

    start(engines, hours).await;
}

async fn start(engines: Vec<&str>, hours: Range<i32>) {
    let mut producer = KafkaProducer::new();

    let sim_conf = read("config/simulation.json").expect("Unable to read configuration file");
    match producer.start_request(sim_conf).await.unwrap() {
        Ok(_) => { start_ticking(engines, hours).await; }
        Err(_) => { panic!("Failed to send simulation request to engines"); }
    }
}

async fn start_ticking(engines: Vec<&str>, hours: Range<i32>) {
    let mut acks: TickAcks = TickAcks::new(engines);
    let mut producer = KafkaProducer::new();
    for h in hours {
        acks.reset(h);

        match producer.send_tick(h).await.unwrap() {
            Ok(_) => {
                let consumer = KafkaConsumer::new();
                let mut message_stream = consumer.start_message_stream();
                while let Some(message) = message_stream.next().await {
                    let tick_ack = parse_message(message);
                    match tick_ack {
                        Err(e) => {
                            println!("Received a message, but could not parse it.\n\
                                Error Details: {}", e)
                        }
                        Ok(ack) => {
                            acks.push(ack);
                            if acks.all_received() {
                                break;
                            }
                        }
                    };
                }
            }
            Err(_) => { panic!("Failed to send simulation request to engines"); }
        }

    }
}

fn read(filename: &str) -> Result<String, Box<dyn Error>> {
    let mut reader = File::open(filename)?;
    let mut contents: String = "".to_string();
    reader.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_message(message: Result<BorrowedMessage, KafkaError>) -> Result<TickAck, Box<dyn Error>> {
    let borrowed_message = message?;
    let parsed_message = borrowed_message.payload_view::<str>().unwrap()?;
    println!("Received: {}", parsed_message);
    serde_json::from_str(parsed_message).map_err(|e| e.into())
}
