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

use clap::Parser;
use engine::config::configuration::{Configuration, EngineConfig};
use engine::config::{Config, TravelPlanConfig};
use engine::disease::Disease;
use engine::{EngineApp, RunMode};
use mpi::traits::Communicator;
use opentelemetry::sdk::trace::{config, Span};
use opentelemetry::sdk::Resource;
use opentelemetry::trace::{FutureExt, TraceContextExt, TraceError, Tracer};
use opentelemetry::{global, sdk, Context, KeyValue};

const STANDALONE_ENGINE_ID: &str = "standalone";

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long, value_name = "FILE", help = "Use a config file to run the simulation")]
    config: Option<String>,

    #[arg(short, long, default_value_t = false)]
    #[arg(help = "Start the engine in daemon mode. It will wait for messages from Kafka. \
            Specifying this flag will cause the config argument to be ignored")]
    standalone: bool,

    #[arg(short, long, default_value_t = 4)]
    #[arg(help = "Number of parallel threads for data parallelization")]
    threads: u32,
}

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
async fn main() {
    env_logger::init();
    let args = Args::parse();
    let number_of_threads = args.threads;
    let standalone = args.standalone;

    let disease_handler: Option<Disease> = None;

    let _tracer = init_tracer().unwrap();

    let span: Span = _tracer.start("root");
    let cx: Context = Context::current_with_span(span);

    println!("println logging is working");

    if standalone {
        println!("its here in standalone");
        let default_config_path = "engine/config/default.json".to_string();
        let config_path = args.config.unwrap_or(default_config_path);
        let engine_config = Config::read(&config_path).expect("Failed to read config file");
        let run_mode = RunMode::Standalone;
        EngineApp::start(STANDALONE_ENGINE_ID.to_string(), engine_config, &run_mode, None, disease_handler, number_of_threads)
            .with_context(cx)
            .await;
    } else {
        println!("in multi-engine mode");
        let universe = mpi::initialize().unwrap();
        let world = universe.world();
        let rank = world.rank();
        let default_config_path = "engine/config/simulation.json".to_string();
        let config_path = args.config.unwrap_or(default_config_path);
        println!("config - {}", config_path);
        let config = Configuration::read(&config_path).expect("Error while reading config");
        config.validate();
        let config_per_engine = config.get_engine_configs();
        let index: usize = (rank) as usize;
        let self_config: &EngineConfig = config_per_engine.get(index).unwrap();
        let travel_plan: &TravelPlanConfig = config.get_travel_plan();
        let engine_config = &self_config.config;
        let engine_id = String::from(&self_config.engine_id);
        println!("engine_id - {}, rank - {} , config - {:?}", engine_id, rank, &self_config);
        let run_mode = RunMode::MultiEngine;
        EngineApp::start(
            engine_id.clone(),
            engine_config.clone(),
            &run_mode,
            Some(travel_plan.clone()),
            disease_handler,
            number_of_threads,
        )
        .with_context(cx)
        .await;
    }
}
