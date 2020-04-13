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


#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use std::ops::Range;

use clap::{App, Arg};
use rdkafka::ClientConfig;
use rdkafka::admin::{AdminClient, AdminOptions};
use rdkafka::client::DefaultClientContext;

use crate::kafka_producer::KafkaProducer;
use crate::travel_plan::TravelPlan;
use crate::config::Config;

mod kafka_producer;
mod kafka_consumer;
mod ticks;
mod environment;
mod travel_plan;
mod config;

#[tokio::main]
async fn main() {
    env_logger::init();

    let matches = App::new("EpiRust Orchestrator")
        .version("0.1")
        .about("Epidemiology Simulations in Rust")
        .arg(Arg::with_name("config")
            .long("config")
            .short("c")
            .value_name("FILE")
            .default_value("config/simulation.json")
            .help("Use a config file to run the simulation"))
        .get_matches();

    let config_path = matches.value_of("config").unwrap_or("config/simulation.json");

    let config = Config::read(config_path).expect("Error while reading config");
    let sim_conf = config::read_simulation_conf(config_path);
    let travel_plan = config.get_travel_plan();

    let hours = 1..10000;

    cleanup().await;
    start(&travel_plan, hours, &sim_conf).await;
}

async fn cleanup() {
    let kafka_url = environment::kafka_url();
    let kafka_admin: AdminClient<DefaultClientContext> = ClientConfig::new()
        .set("bootstrap.servers", kafka_url.as_str())
        .create()
        .expect("Admin client creation failed");
    match kafka_admin.delete_topics(&["ticks", "ticks_ack", "travels"], &AdminOptions::new()).await {
        Ok(_) => {}
        Err(_) => { error!("Warning: Failed to cleanup ticks and ticks_ack topics") }
    }
}

async fn start(travel_plan: &TravelPlan, hours: Range<i32>, sim_conf: &String) {
    let mut producer = KafkaProducer::new();

    match producer.start_request(sim_conf).await.unwrap() {
        Ok(_) => { ticks::start_ticking(travel_plan, hours).await; }
        Err(_) => { panic!("Failed to send simulation request to engines"); }
    }
}
