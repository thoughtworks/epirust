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

use std::error::Error;

use common::config::request::Request;
use futures::StreamExt;
use rdkafka::consumer::Consumer;
use rdkafka::consumer::{MessageStream, StreamConsumer};
use rdkafka::error::KafkaError;
use rdkafka::message::BorrowedMessage;
use rdkafka::message::Message;
use rdkafka::ClientConfig;

use crate::epidemiology_simulation::Epidemiology;
use crate::utils::environment;
use crate::RunMode;

pub struct KafkaConsumer<'a> {
    engine_id: &'a str,
    consumer: StreamConsumer,
}

impl KafkaConsumer<'_> {
    pub fn new<'a>(engine_id: &'a str, topics: &[&str]) -> KafkaConsumer<'a> {
        let kafka_url = environment::kafka_url();
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", kafka_url.as_str())
            .set("group.id", &*format!("request_{}", engine_id))
            .set("auto.offset.reset", "earliest")
            .set("max.poll.interval.ms", "86400000") //max allowed
            .set("message.max.bytes", "104857600") //in order to allow message greater than 1MB
            .create()
            .expect("Consumer creation failed");

        consumer.subscribe(topics).expect("Can't subscribe to specified topics");

        KafkaConsumer { engine_id, consumer }
    }

    pub async fn listen_loop(&self, run_mode: &RunMode, threads: u32) {
        let mut message_stream: MessageStream = self.consumer.stream();
        debug!("Started the stream. Waiting for simulation request");
        while let Some(message) = message_stream.next().await {
            let simulation_config = self.parse_message(message);
            match simulation_config {
                Err(e) => {
                    error!(
                        "Received a message, but could not parse it.\n\
                        Error Details: {}",
                        e
                    );
                }
                Ok(request) => {
                    self.run_sim(request, run_mode, threads).await;
                    if let RunMode::MultiEngine { engine_id: _e } = run_mode {
                        return;
                    }
                }
            };
        }
    }

    async fn run_sim(&self, request: Request, run_mode: &RunMode, threads: u32) {
        match request {
            Request::SimulationRequest(req) => {
                let mut epidemiology = Epidemiology::new(&req.config, None, req.sim_id);
                epidemiology.run(&req.config, run_mode, threads).await;
            }
            Request::MultiSimRequest(req) => {
                let travel_plan_config = Some(req.travel_plan);
                let sim_req = req.engine_configs.iter().find(|c| c.engine_id == self.engine_id);
                match sim_req {
                    None => {
                        error!("Couldn't find any work for engine_id: {}", self.engine_id)
                    }
                    Some(req) => {
                        let mut epidemiology =
                            Epidemiology::new(&req.config.config, travel_plan_config, req.engine_id.to_string());
                        epidemiology.run(&req.config.config, run_mode, threads).await;
                    }
                }
            }
        }
    }

    fn parse_message(&self, message: Result<BorrowedMessage, KafkaError>) -> Result<Request, Box<dyn Error>> {
        let borrowed_message = message?;
        let parsed_message = borrowed_message.payload_view::<str>().unwrap()?;
        trace!("Received: {}", parsed_message);
        serde_json::from_str(parsed_message).map_err(|e| e.into())
    }
}
