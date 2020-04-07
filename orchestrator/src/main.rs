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

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::ops::Range;

use clap::{App, Arg};
use rdkafka::ClientConfig;
use rdkafka::admin::{AdminClient, AdminOptions};
use rdkafka::client::DefaultClientContext;

use crate::kafka_producer::KafkaProducer;
use crate::travel_plan::TravelPlan;

mod kafka_producer;
mod kafka_consumer;
mod ticks;
mod environment;
mod travel_plan;

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
        .arg(Arg::with_name("travel")
            .long("travel")
            .short("t")
            .default_value("config/travel_plan.json")
            .help("The travel plan for agents to move between regions"))
        .get_matches();

    let config_path = matches.value_of("config").unwrap_or("config/simulation.json");
    let travel_plan_config = matches.value_of("travel").unwrap_or("config/travel_plan.json");

    let sim_conf = read(config_path)
        .expect("Unable to read configuration file");
    let engines = parse_engine_names(&sim_conf);

    let travel_plan = TravelPlan::read(travel_plan_config);
    if !travel_plan.validate_regions(&engines) {
        panic!("Engine names should match regions in travel plan");
    }

    let hours = 0..10000;

    cleanup().await;
    start(engines, hours, &sim_conf).await;
}

async fn cleanup() {
    let kafka_url = environment::kafka_url();
    let kafka_admin: AdminClient<DefaultClientContext> = ClientConfig::new()
        .set("bootstrap.servers", kafka_url.as_str())
        .create()
        .expect("Admin client creation failed");
    match kafka_admin.delete_topics(&["ticks", "ticks_ack"], &AdminOptions::new()).await {
        Ok(_) => {}
        Err(_) => { error!("Warning: Failed to cleanup ticks and ticks_ack topics") }
    }
}

async fn start(engines: Vec<String>, hours: Range<i32>, sim_conf: &String) {
    let mut producer = KafkaProducer::new();

    match producer.start_request(sim_conf).await.unwrap() {
        Ok(_) => { ticks::start_ticking(engines, hours).await; }
        Err(_) => { panic!("Failed to send simulation request to engines"); }
    }
}

fn read(filename: &str) -> Result<String, Box<dyn Error>> {
    let mut reader = File::open(filename)?;
    let mut contents: String = "".to_string();
    reader.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_engine_names(sim_conf: &String) -> Vec<String> {
    let engine_ids: Vec<EngineId> = serde_json::from_str(sim_conf).expect("Failed to parse simulation config");
    engine_ids.iter().map(|e| e.engine_id.clone()).collect()
}

// just a struct for easier parsing
#[derive(Deserialize)]
struct EngineId {
    engine_id: String,
}
