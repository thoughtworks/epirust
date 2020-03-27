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

pub struct KafkaConsumer {
    consumer: StreamConsumer,
}

impl KafkaConsumer {
    pub fn new() -> KafkaConsumer {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", "localhost:9092")
            .set("group.id", "engine")
            .create()
            .expect("Consumer creation failed");

        let topics = ["simulation_requests"];
        consumer.subscribe(&topics)
            .expect("Can't subscribe to specified topics");

        return KafkaConsumer { consumer };
    }

    pub async fn listen_loop(&self) {
        let mut message_stream: MessageStream<DefaultConsumerContext> = self.consumer.start();
        while let Some(message) = message_stream.next().await {
            let simulation_config = self.parse_message(message);
            match simulation_config {
                Err(e) => {
                    println!("Received a message, but could not parse it.\n\
                        Error Details: {}", e)
                }
                Ok(request) => {
                    let mut epidemiology = Epidemiology::new(&request.config, request.sim_id);
                    epidemiology.run(&request.config);
                }
            };
        }
    }

    fn parse_message(&self, message: Result<BorrowedMessage, KafkaError>) -> Result<SimulationRequest, Box<dyn Error>> {
        let borrowed_message = message?;
        let parsed_message = borrowed_message.payload_view::<str>().unwrap()?;
        serde_json::from_str(parsed_message).map_err(|e| e.into())
    }
}

#[derive(Deserialize)]
struct SimulationRequest {
    sim_id: String,
    #[serde(flatten)]
    config: Config,
}
