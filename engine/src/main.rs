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
#[macro_use]
extern crate log;

use clap::{App, Arg};
use common::config::intervention_config::InterventionConfig::Vaccinate;
use common::config::Config;
use opentelemetry::sdk::trace::{config, Span};
use opentelemetry::sdk::Resource;
use opentelemetry::trace::{FutureExt, TraceContextExt, TraceError, Tracer};
use opentelemetry::{global, sdk, Context, KeyValue};
use opentelemetry_jaeger::Exporter;
use std::error::Error;

use crate::kafka::kafka_consumer::KafkaConsumer;

mod allocation_map;
mod citizen;
mod disease_state_machine;
mod epidemiology_simulation;
mod geography;
mod helpers;
mod interventions;
mod kafka;
mod listeners;
mod models;
mod population;
mod tick;
mod travel;
mod utils;

const STANDALONE_SIM_ID: &str = "0";

fn init_tracer() -> Result<sdk::trace::Tracer, TraceError> {
    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    opentelemetry_jaeger::new_agent_pipeline()
        .with_auto_split_batch(true)
        .with_max_packet_size(9216)
        .with_service_name("epirust-trace")
        .with_trace_config(config().with_resource(Resource::new(vec![KeyValue::new("exporter", "otlp-jaeger")])))
        .install_batch(opentelemetry::runtime::Tokio)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    env_logger::init();
    let matches = App::new("EpiRust")
        .version("0.1")
        .about("Epidemiology Simulations in Rust")
        .arg(
            Arg::with_name("config").long("config").short('c').value_name("FILE").help("Use a config file to run the simulation"),
        )
        .arg(
            Arg::with_name("daemon")
                .long("daemon")
                .short('d')
                .help(
                    "Start the engine in daemon mode. It will wait for messages from Kafka. \
            Specifying this flag will cause the config argument to be ignored",
                )
                .takes_value(false),
        )
        .arg(
            Arg::with_name("id")
                .long("id")
                .short('i')
                .help(
                    "An identifier for the engine. Needed in daemon mode when running a larger simulation \
            distributed across multiple engines.",
                )
                .takes_value(true),
        )
        .get_matches();

    let _tracer = init_tracer()?;

    let span: Span = _tracer.start("root");
    let cx: Context = Context::current_with_span(span);

    let daemon = matches.is_present("daemon");
    let has_named_engine = matches.is_present("id");
    let engine_id = matches.value_of("id").unwrap_or("default_engine");
    let run_mode = if daemon && has_named_engine {
        RunMode::MultiEngine { engine_id: engine_id.to_string() }
    } else if daemon {
        RunMode::SingleDaemon
    } else {
        RunMode::Standalone
    };

    if daemon {
        info!("Started in daemon mode");
        let consumer = KafkaConsumer::new(engine_id, &["simulation_requests"]);
        consumer.listen_loop(&run_mode).with_context(cx).await;
        info!("Done");
    } else {
        let config_file = matches.value_of("config").unwrap_or("config/default.json");

        let config = Config::read(config_file).expect("Failed to read config file");

        let mut epidemiology = epidemiology_simulation::Epidemiology::new(&config, None, STANDALONE_SIM_ID.to_string());
        epidemiology.run(&config, &run_mode).await;
        info!("Done");
    }
    global::shutdown_tracer_provider(); // sending remaining spans
    Ok(())
}

pub enum RunMode {
    //run once and exit
    Standalone,

    //daemon mode, with only one engine
    SingleDaemon,

    //daemon mode, with multiple engines and an orchestrator
    MultiEngine { engine_id: String },
}
