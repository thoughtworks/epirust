use rdkafka::error::KafkaError;
use rdkafka::producer::{BaseRecord, DefaultProducerContext, ThreadedProducer};
use rdkafka::ClientConfig;

use crate::environment;

pub struct KafkaProducer {
    producer: ThreadedProducer<DefaultProducerContext>,
}

type SendResult<'a> = Result<(), (KafkaError, BaseRecord<'a, String, String>)>;

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

    pub fn start_request<'a>(&mut self, request: &'a String) -> SendResult<'a> {
        let record: BaseRecord<String, String> = BaseRecord::to("simulation_requests").payload(request);
        info!("Sent simulation request");
        self.producer.send(record)
    }

    pub fn send_tick<'a>(&mut self, tick_ack_str: &'a String) -> SendResult<'a> {
        let record: BaseRecord<String, String> = BaseRecord::to("ticks").payload(tick_ack_str);
        debug!("Send tick: {}", record.payload.unwrap());
        self.producer.send(record)
    }
}
