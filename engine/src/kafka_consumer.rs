use std::error::Error;

use futures::StreamExt;
use rdkafka::ClientConfig;
use rdkafka::consumer::{DefaultConsumerContext, MessageStream, StreamConsumer};
use rdkafka::consumer::Consumer;
use rdkafka::error::KafkaError;
use rdkafka::message::BorrowedMessage;
use rdkafka::message::Message;

use crate::config::Config;
use crate::epidemiology_simulation::Epidemiology;
use crate::{RunMode, environment};

pub struct KafkaConsumer<'a> {
    engine_id: &'a str,
    consumer: StreamConsumer,
}

impl KafkaConsumer<'_> {
    pub fn new<'a>(engine_id: &'a str, topics: &[&str]) -> KafkaConsumer<'a> {
        let kafka_url = environment::kafka_url();
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", kafka_url.as_str())
            .set("group.id", engine_id)
            .set("auto.offset.reset", "earliest")
            .create()
            .expect("Consumer creation failed");

        consumer.subscribe(topics)
            .expect("Can't subscribe to specified topics");

        return KafkaConsumer { engine_id, consumer };
    }

    pub async fn listen_loop(&self, run_mode: &RunMode) {
        let mut message_stream: MessageStream<DefaultConsumerContext> = self.consumer.start();
        while let Some(message) = message_stream.next().await {
            let simulation_config = self.parse_message(message);
            match simulation_config {
                Err(e) => {
                    error!("Received a message, but could not parse it.\n\
                        Error Details: {}", e);
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
                let mut epidemiology = Epidemiology::new(&req.config, req.sim_id);
                epidemiology.run(&req.config, None, run_mode).await;
            }
            Request::MultiSimRequest(req) => {
                let travel_plan_config = req.travel_plan;
                let sim_req = req.engine_configs.iter().find(|c| c.engine_id == self.engine_id);
                match sim_req {
                    None => { error!("Couldn't find any work for engine_id: {}", self.engine_id) }
                    Some(req) => {
                        let sim_id = req.config.sim_id.clone();
                        let mut epidemiology = Epidemiology::new(&req.config.config, sim_id);
                        epidemiology.run(&req.config.config, Some(travel_plan_config), run_mode).await;
                    }
                }
            }
        }
    }

    fn parse_message(&self, message: Result<BorrowedMessage, KafkaError>) -> Result<Request, Box<dyn Error>> {
        let borrowed_message = message?;
        let parsed_message = borrowed_message.payload_view::<str>().unwrap()?;
        debug!("Received: {}", parsed_message);
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

#[derive(Debug, Deserialize)]
struct Migration {
    matrix: Vec<Vec<u32>>,
    start_migration_hour: u32,
    end_migration_hour: u32,
}

#[derive(Debug, Deserialize)]
pub struct TravelPlanConfig {
    regions: Vec<String>,
    migration: Migration
}

impl TravelPlanConfig {
    pub fn get_end_migration_hour(&self) -> u32 {
        self.migration.end_migration_hour
    }

    pub fn get_start_migration_hour(&self) -> u32 {
        self.migration.start_migration_hour
    }

    pub fn get_migration_matrix(&self) -> Vec<Vec<u32>> {
        self.migration.matrix.clone()
    }

    pub fn get_regions(&self) -> Vec<String> {
        self.regions.clone()
    }
}

#[derive(Debug, Deserialize)]
struct MultiSimRequest {
    engine_configs: Vec<SimRequestByEngine>,
    travel_plan: TravelPlanConfig
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Request {
    SimulationRequest(SimulationRequest),
    MultiSimRequest(MultiSimRequest),
}
