/*
 * EpiRust
 * Copyright (c) 2020  ThoughtWorks, Inc.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

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
mod interventions;

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
