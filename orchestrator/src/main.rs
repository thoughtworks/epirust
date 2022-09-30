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

extern crate core;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use std::ops::Range;
use std::string::String;

use clap::{App, Arg};
use rdkafka::admin::{AdminClient, AdminOptions, NewTopic, TopicReplication};
use rdkafka::client::DefaultClientContext;
use rdkafka::ClientConfig;
use common::config::TravelPlanConfig;

use crate::config::Configuration;
use crate::kafka_producer::KafkaProducer;
use crate::utils::get_hours;

mod config;
mod environment;
mod kafka_consumer;
mod kafka_producer;
mod ticks;
mod utils;

#[tokio::main]
async fn main() {
    env_logger::init();

    let matches = App::new("EpiRust Orchestrator")
        .version("0.1")
        .about("Epidemiology Simulations in Rust")
        .arg(
            Arg::with_name("config")
                .long("config")
                .short('c')
                .value_name("FILE")
                .default_value("config/simulation.json")
                .help("Use a config file to run the simulation"),
        )
        .get_matches();

    let config_path = matches.value_of("config").unwrap_or("config/simulation.json");

    let config = Configuration::read(config_path).expect("Error while reading config");
    let sim_conf = utils::read_simulation_conf(config_path);
    let travel_plan = config.get_travel_plan();

    let hours = 1..get_hours(config_path);

    config.validate();
    cleanup(&travel_plan.get_regions()).await;
    start(travel_plan, hours, &sim_conf).await;
}

async fn cleanup(regions: &Vec<String>) {
    let kafka_url = environment::kafka_url();
    let kafka_admin: AdminClient<DefaultClientContext> =
        ClientConfig::new().set("bootstrap.servers", kafka_url.as_str()).create().expect("Admin client creation failed");
    match kafka_admin.delete_topics(&["simulation_requests", "counts_updated", "ticks", "ticks_ack"], &AdminOptions::new()).await
    {
        Ok(t) => {
            debug!("Deleted topics {:?}", t)
        }
        Err(e) => {
            debug!("Error while deleting topics {:?}", e)
        }
    }
    for region in regions {
        match kafka_admin.delete_topics(&[&*format!("{}{}", "commute_", region)], &AdminOptions::new()).await {
            Ok(t) => {
                debug!("Deleted topic {:?}", t)
            }
            Err(e) => {
                debug!("Error while deleting topic {:?}", e)
            }
        }
        match kafka_admin.delete_topics(&[&*format!("{}{}", "migration_", region)], &AdminOptions::new()).await {
            Ok(t) => {
                debug!("Deleted topic {:?}", t)
            }
            Err(e) => {
                debug!("Error while deleting topic {:?}", e)
            }
        }
    }
    for region in regions {
        match kafka_admin
            .create_topics(
                &[NewTopic::new(&*format!("{}{}", "commute_", region), 1, TopicReplication::Fixed(1))],
                &AdminOptions::new(),
            )
            .await
        {
            Ok(t) => {
                debug!("Created topic {:?}", t)
            }
            Err(e) => {
                debug!("Error while creating topics {:?}", e)
            }
        }
        match kafka_admin
            .create_topics(
                &[NewTopic::new(&*format!("{}{}", "migration_", region), 1, TopicReplication::Fixed(1))],
                &AdminOptions::new(),
            )
            .await
        {
            Ok(t) => {
                debug!("Created topic {:?}", t)
            }
            Err(e) => {
                debug!("Error while creating topics {:?}", e)
            }
        }
    }
    match kafka_admin
        .create_topics(
            &[
                NewTopic::new("simulation_requests", 1, TopicReplication::Fixed(1)),
                NewTopic::new("counts_updated", 1, TopicReplication::Fixed(1)),
                NewTopic::new("ticks", 1, TopicReplication::Fixed(1)),
                NewTopic::new("ticks_ack", 1, TopicReplication::Fixed(1)),
            ],
            &AdminOptions::new(),
        )
        .await
    {
        Ok(t) => {
            debug!("Created topics {:?}", t)
        }
        Err(e) => {
            debug!("Error while creating topics {:?}", e)
        }
    }
}

async fn start(travel_plan: &TravelPlanConfig, hours: Range<i64>, sim_conf: &String) {
    let mut producer = KafkaProducer::new();

    match producer.start_request(sim_conf) {
        Ok(_) => {
            debug!("Sent Request Successfully");
            ticks::start_ticking(travel_plan, hours).await;
        }
        Err(_) => {
            panic!("Failed to send simulation request to engines");
        }
    }
}
