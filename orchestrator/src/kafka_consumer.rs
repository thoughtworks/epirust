use rdkafka::ClientConfig;
use rdkafka::consumer::{DefaultConsumerContext, MessageStream, StreamConsumer, Consumer};
use std::time::Duration;

pub struct KafkaConsumer {
    consumer: StreamConsumer,
}

impl KafkaConsumer {
    pub fn new() -> KafkaConsumer {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", "localhost:9092")
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
        self.consumer.start_with(Duration::from_millis(10), false)
    }
}
