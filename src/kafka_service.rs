use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use crate::events::{Listener, Counts};

pub struct KafkaService {
    producer: FutureProducer
}

impl KafkaService {
    pub fn new() -> KafkaService {
        KafkaService {
            producer: ClientConfig::new()
                .set("bootstrap.servers", "localhost:9092")
                .create()
                .expect("Could not crate Kafka Producer")
        }
    }
}

impl Listener for KafkaService {
    fn counts_updated(&mut self, counts: Counts) {
        let message = serde_json::to_string(&counts).expect("Failed to serialize counts");
        let record: FutureRecord<String, String> = FutureRecord::to("counts_updated").payload(&message);
        self.producer.send(record, 0);
    }

    fn simulation_ended(&self) {
    }
}
