use rdkafka::consumer::{Consumer, MessageStream, StreamConsumer};
use rdkafka::ClientConfig;

use crate::environment;

pub struct KafkaConsumer {
    consumer: StreamConsumer,
}

impl KafkaConsumer {
    pub fn new() -> KafkaConsumer {
        let kafka_url = environment::kafka_url();
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", kafka_url.as_str())
            .set("group.id", "orchestrator")
            .set("message.max.bytes", "104857600")
            .set("auto.offset.reset", "earliest")
            .create()
            .expect("Consumer creation failed");

        let topics = ["ticks_ack"];
        consumer.subscribe(&topics).expect("Can't subscribe to specified topics");

        KafkaConsumer { consumer }
    }

    pub fn start_message_stream(&self) -> MessageStream {
        self.consumer.stream()
    }
}
