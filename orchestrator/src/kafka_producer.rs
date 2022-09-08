use std::time::Duration;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use rdkafka::producer::future_producer::OwnedDeliveryResult;
use crate::environment;
use crate::ticks::Tick;

pub struct KafkaProducer {
    producer: FutureProducer,
}

impl KafkaProducer {
    pub fn new() -> KafkaProducer {
        let kafka_url = environment::kafka_url();
        KafkaProducer {
            producer: ClientConfig::new()
                .set("bootstrap.servers", kafka_url.as_str())
                .set("message.max.bytes", "104857600")
                .create()
                .expect("Could not create Kafka Producer"),
        }
    }

    pub async fn start_request(&mut self, request: &String) -> OwnedDeliveryResult {
        let record: FutureRecord<String, String> = FutureRecord::to("simulation_requests").payload(request);
        info!("Sent simulation request");
        self.producer.send(record, Duration::from_secs(0)).await
    }

    pub async fn send_tick(&mut self, tick: &Tick) -> OwnedDeliveryResult {
        let payload = serde_json::to_string(tick).unwrap();
        let record: FutureRecord<String, String> = FutureRecord::to("ticks").payload(&payload);
        debug!("Send tick: {}", payload);
        self.producer.send(record, Duration::from_secs(0)).await
    }
}
