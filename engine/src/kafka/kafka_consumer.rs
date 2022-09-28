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

use futures::StreamExt;
use rdkafka::ClientConfig;
use rdkafka::consumer::{MessageStream, StreamConsumer};
use rdkafka::consumer::Consumer;
use rdkafka::error::KafkaError;
use rdkafka::message::BorrowedMessage;
use rdkafka::message::Message;

use crate::config::Config;
use crate::epidemiology_simulation::Epidemiology;
use crate::RunMode;
use crate::utils::environment;
use crate::commute::{Commute, CommutePlan};

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

    pub async fn listen_loop(&self, run_mode: &RunMode) {
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
                    self.run_sim(request, run_mode).await;
                    if let RunMode::MultiEngine { engine_id: _e } = run_mode {
                        return;
                    }
                }
            };
        }
    }

    async fn run_sim(&self, request: Request, run_mode: &RunMode) {
        match request {
            Request::SimulationRequest(req) => {
                let mut epidemiology = Epidemiology::new(&req.config, None, req.sim_id);
                epidemiology.run(&req.config, run_mode).await;
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
                        epidemiology.run(&req.config.config, run_mode).await;
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

#[derive(Debug, Deserialize)]
struct SimulationRequest {
    sim_id: String,
    #[serde(flatten)]
    config: Config,
}

#[derive(Debug, Deserialize)]
struct SimRequestByEngine {
    engine_id: String,
    config: SimulationRequest,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Migration {
    pub enabled: bool,
    matrix: Option<Vec<Vec<u32>>>,
    start_migration_hour: u32,
    end_migration_hour: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TravelPlanConfig {
    pub regions: Vec<String>,
    pub migration: Migration,
    pub commute: Commute,
}

impl TravelPlanConfig {
    pub fn get_end_migration_hour(&self) -> u32 {
        self.migration.end_migration_hour
    }

    pub fn get_start_migration_hour(&self) -> u32 {
        self.migration.start_migration_hour
    }

    pub fn get_migration_matrix(&self) -> Option<Vec<Vec<u32>>> {
        self.migration.matrix.clone()
    }

    pub fn get_regions(&self) -> Vec<String> {
        self.regions.clone()
    }

    pub fn commute_plan(&self) -> CommutePlan {
        CommutePlan { regions: self.regions.clone(), matrix: self.commute.matrix.as_ref().unwrap().clone() }
    }
}

#[derive(Debug, Deserialize)]
struct MultiSimRequest {
    engine_configs: Vec<SimRequestByEngine>,
    travel_plan: TravelPlanConfig,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Request {
    SimulationRequest(SimulationRequest),
    MultiSimRequest(MultiSimRequest),
}
