use rdkafka::ClientConfig;
use rdkafka::consumer::{DefaultConsumerContext, MessageStream, StreamConsumer, Consumer};
use std::time::Duration;
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
            .set("auto.offset.reset", "earliest")
            .create()
            .expect("Consumer creation failed");

        let topics = ["ticks_ack"];
        consumer.subscribe(&topics)
            .expect("Can't subscribe to specified topics");

        return KafkaConsumer { consumer };
    }

    pub fn start_message_stream(&self) -> MessageStream<DefaultConsumerContext> {
        self.consumer.start_with(Duration::from_millis(1), false)
    }
}
