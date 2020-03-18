extern crate clap;
#[macro_use]
extern crate serde_derive;

use clap::{App, Arg};

use crate::kafka_service::KafkaConsumer;

mod constants;

mod agent;
mod epidemiology_simulation;
mod allocation_map;
mod geography;
mod disease;
mod csv_service;
mod random_wrapper;
mod disease_tracker;
mod events;
mod kafka_service;
mod population;
mod geojson_service;
mod models;
mod config;

#[tokio::main]
async fn main() {
    let matches = App::new("EpiRust")
        .version("0.1")
        .about("Epidemiology Simulations in Rust")
        .arg(Arg::with_name("config")
            .long("config")
            .short("c")
            .value_name("FILE")
            .help("Use a config file to run the simulation"))
        .arg(Arg::with_name("daemon")
            .long("daemon")
            .short("d")
            .help("Start the engine in daemon mode. It will wait for messages from Kafka. \
            Specifying this flag will cause other arguments to be ignored")
            .takes_value(false))
        .get_matches();

    let daemon = matches.is_present("daemon");

    if daemon {
        println!("Started in daemon mode");
        let consumer = KafkaConsumer::new();
        consumer.listen_loop().await;
    } else {
        let config_file = matches.value_of("config").unwrap_or("config/default.json");

        // let (count, grid_default) = match input_count {
        //     0..=100 => (100, 25),
        //     101..=1000 => (1000, 80),
        //     1001..=10000 => (10000, 250),
        //     10001..=100_000 => (100_000, 800),
        //     100_001..=1_000_000 => (1_000_000, 2500),
        //     1_000_001..=2_000_000 => (2_000_000, 3550),
        //     2_000_001..=5_000_000 => (5_000_000, 5660),
        //     _ => panic!("Cannot run for {} agents", input_count)
        // };

        let config = config::read(config_file.to_string()).expect("Failed to read config file");

        let mut epidemiology = epidemiology_simulation::Epidemiology::new(&config);
        epidemiology.run(&config);
        println!("Done");
    }
}
