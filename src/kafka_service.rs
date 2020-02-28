use std::error::Error;

use futures::StreamExt;
use rdkafka::{ClientConfig, Message};
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::error::KafkaError;
use rdkafka::message::BorrowedMessage;
use rdkafka::producer::{FutureProducer, FutureRecord};

use crate::epidemiology_simulation::{Epidemiology, SimulationParams};
use crate::events::{Counts, Listener};
use crate::geography::Point;
use std::any::Any;

pub struct KafkaProducer {
    producer: FutureProducer
}

impl KafkaProducer {
    pub fn new() -> KafkaProducer {
        KafkaProducer {
            producer: ClientConfig::new()
                .set("bootstrap.servers", "localhost:9092")
                .create()
                .expect("Could not create Kafka Producer")
        }
    }
}

impl Listener for KafkaProducer {
    fn counts_updated(&mut self, counts: Counts) {
        let message = serde_json::to_string(&counts).expect("Failed to serialize counts");
        let record: FutureRecord<String, String> = FutureRecord::to("counts_updated").payload(&message);
        self.producer.send(record, 0);
    }

    fn simulation_ended(&mut self) {}

    fn citizen_got_infected(&mut self, _cell: &Point) {}

    fn as_any(&self) -> &dyn Any {
        self
    }
}

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
        let mut message_stream = self.consumer.start();
        while let Some(message) = message_stream.next().await {
            let simulation_params = self.parse_message(message);
            match simulation_params {
                Err(e) => {
                    println!("Received a message, but could not parse it.\n\
                        Error Details: {}",  e)
                }
                Ok(params) => {
                    let mut epidemiology = Epidemiology::new(&params);
                    epidemiology.run(&params);
                }
            };
        }
    }

    fn parse_message(&self, message: Result<BorrowedMessage, KafkaError>) -> Result<SimulationParams, Box<dyn Error>> {
        let borrowed_message = message?;
        let parsed_message = borrowed_message.payload_view::<str>().unwrap()?;
        serde_json::from_str(parsed_message).map_err(|e| e.into())
    }
}
