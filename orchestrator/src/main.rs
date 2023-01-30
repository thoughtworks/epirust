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

use clap::Parser;
use common::config::TravelPlanConfig;
use rdkafka::admin::{AdminClient, AdminOptions, NewTopic, TopicReplication};
use rdkafka::client::DefaultClientContext;
use rdkafka::ClientConfig;

use crate::config::Configuration;
use crate::kafka_producer::KafkaProducer;
use crate::utils::get_hours;

mod config;
mod environment;
mod kafka_consumer;
mod kafka_producer;
mod ticks;
mod utils;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long, value_name = "FILE", help = "Use a config file to run the simulation")]
    config: Option<String>,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = Args::parse();

    let default_config_path = "config/simulation.json".to_string();
    let config_path = args.config.unwrap_or(default_config_path);

    let config = Configuration::read(&config_path).expect("Error while reading config");
    let sim_conf = utils::read_simulation_conf(&config_path);
    let travel_plan = config.get_travel_plan();

    //TODO: use already read config instead of passing config path and reading file again
    let hours = 1..get_hours(&config_path);

    config.validate();
    cleanup(&travel_plan.get_regions()).await;
    start(travel_plan, hours, &sim_conf).await;
}

async fn cleanup(regions: &Vec<String>) {
    let kafka_url = environment::kafka_url();
    let kafka_admin: AdminClient<DefaultClientContext> =
        ClientConfig::new().set("bootstrap.servers", kafka_url.as_str()).create().expect("Admin client creation failed");
    let mut fixed_topics_names: Vec<&str> = vec!["simulation_requests", "counts_updated", "ticks", "ticks_ack"];
    let dynamic_topic_names: Vec<String> = generate_topic_names(regions);
    let mut all_topic_names: Vec<&str> = dynamic_topic_names.iter().map(|s| &**s).collect();
    all_topic_names.append(&mut fixed_topics_names);
    match kafka_admin.delete_topics(all_topic_names.as_slice(), &AdminOptions::new()).await {
        Ok(t) => {
            debug!("Deleted topics {:?}", t)
        }
        Err(e) => {
            debug!("Error while deleting topics {:?}", e)
        }
    }
    create_topics(all_topic_names, kafka_admin).await;
}

fn generate_topic_names(regions: &Vec<String>) -> Vec<String> {
    let mut topics = vec![];
    for region in regions {
        topics.push(format!("{}{}", "commute_", region));
        topics.push(format!("{}{}", "migration_", region));
    }
    topics
}

async fn create_topics(topic_names: Vec<&str>, kafka_admin: AdminClient<DefaultClientContext>) {
    let topics: Vec<NewTopic> = topic_names.iter().map(|name| NewTopic::new(name, 1, TopicReplication::Fixed(1))).collect();
    match kafka_admin.create_topics(topics.iter(), &AdminOptions::new()).await {
        Ok(t) => {
            debug!("Created topic {:?}", t)
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
