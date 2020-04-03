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

#[macro_use]
extern crate log;

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::ops::Range;

use futures::StreamExt;
use rdkafka::{ClientConfig, Message};
use rdkafka::admin::{AdminClient, AdminOptions};
use rdkafka::client::DefaultClientContext;
use rdkafka::error::KafkaError;
use rdkafka::message::BorrowedMessage;



use crate::kafka_consumer::KafkaConsumer;
use crate::kafka_producer::KafkaProducer;
use crate::ticks::{TickAck, TickAcks};

mod kafka_producer;
mod kafka_consumer;
mod ticks;
mod environment;

#[tokio::main]
async fn main() {
    env_logger::init();
    let sim_conf = read("config/simulation.json")
        .expect("Unable to read configuration file");
    let engines = parse_engine_names(&sim_conf);
    let hours = 0..10000;

    cleanup().await;
    start(engines, hours, &sim_conf).await;
}

async fn cleanup() {
    let kafka_url = environment::kafka_url();
    let kafka_admin: AdminClient<DefaultClientContext> = ClientConfig::new()
        .set("bootstrap.servers", kafka_url.as_str())
        .create()
        .expect("Admin client creation failed");
    match kafka_admin.delete_topics(&["ticks", "ticks_ack"], &AdminOptions::new()).await {
        Ok(_) => {}
        Err(_) => { error!("Warning: Failed to cleanup ticks and ticks_ack topics") }
    }
}

async fn start(engines: Vec<String>, hours: Range<i32>, sim_conf: &String) {
    let mut producer = KafkaProducer::new();

    match producer.start_request(sim_conf).await.unwrap() {
        Ok(_) => { start_ticking(engines, hours).await; }
        Err(_) => { panic!("Failed to send simulation request to engines"); }
    }
}

async fn start_ticking(engines: Vec<String>, hours: Range<i32>) {
    let mut acks: TickAcks = TickAcks::new(engines);
    let mut producer = KafkaProducer::new();
    let consumer = KafkaConsumer::new();
    let mut message_stream = consumer.start_message_stream();
    for h in hours {
        acks.reset(h);

        match producer.send_tick(h).await.unwrap() {
            Ok(_) => {
                while let Some(message) = message_stream.next().await {
                    let tick_ack = parse_message(message);
                    match tick_ack {
                        Err(e) => {
                            error!("Received a message, but could not parse it.\n\
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
    debug!("Received: {}", parsed_message);
    serde_json::from_str(parsed_message).map_err(|e| e.into())
}

fn parse_engine_names(sim_conf: &String) -> Vec<String> {
    let engine_ids: Vec<EngineId> = serde_json::from_str(sim_conf).expect("Failed to parse simulation config");
    engine_ids.iter().map(|e| e.engine_id.clone()).collect()
}

// just a struct for easier parsing
#[derive(Deserialize)]
struct EngineId {
    engine_id: String,
}
