use rdkafka::producer::{FutureProducer, FutureRecord, DeliveryFuture};
use rdkafka::ClientConfig;
use crate::environment;

pub struct KafkaProducer {
    producer: FutureProducer,
}

impl KafkaProducer {
    pub fn new() -> KafkaProducer {
        let kafka_url = environment::kafka_url();
        KafkaProducer {
            producer: ClientConfig::new()
                .set("bootstrap.servers", kafka_url.as_str())
                .create()
                .expect("Could not create Kafka Producer")
        }
    }

    pub fn start_request(&mut self, request: &String) -> DeliveryFuture {
        let record: FutureRecord<String, String> = FutureRecord::to("simulation_requests")
            .payload(request);
        println!("Sent simulation request");
        self.producer.send(record, 0)
    }

    pub fn send_tick(&mut self, hour: i32) -> DeliveryFuture {
        let h = &hour.to_string();
        let record: FutureRecord<String, String> = FutureRecord::to("ticks")
            .payload(h);
        println!("Send tick: {}", h);
        self.producer.send(record, 0)
    }
}
