use std::fs::File;
use std::io::Read;

use crate::kafka_producer::KafkaProducer;
use std::error::Error;

mod kafka_producer;

#[tokio::main]
async fn main() {
    let n_engines = 2;
    let hours = 10000;

    let mut kafka_producer = KafkaProducer::new();

    let sim_conf = read("config/simulation.json");
    kafka_producer.start_request(sim_conf).await.expect("Failed to send simulation request");

    // for h in hours {
    //     //send tick to engines
    //     //wait for acknowledgement
    // }
}

fn read(filename: &str) -> String {
    let mut reader = File::open(filename).unwrap();
    let mut contents: String = "".to_string();
    reader.read_to_string(&mut contents).unwrap();
    contents
}
