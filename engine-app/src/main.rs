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
use mpi::topology::Communicator;
use opentelemetry::sdk::trace::{config, Span};
use opentelemetry::sdk::Resource;
use opentelemetry::trace::{FutureExt, TraceContextExt, TraceError, Tracer};
use opentelemetry::{global, sdk, Context, KeyValue};
use std::fmt::{Display, Formatter};

use common::config::{Config, Configuration, EngineConfig, TravelPlanConfig};
use common::disease::Disease;
use engine::{EngineApp, MpiTransport, MultiEngineMode, RunMode};

const BUFFER_SIZE: usize = 100 * 1024 * 1024;

#[derive(clap::ValueEnum, Clone, Debug)]
enum Mode {
    Kafka,
    MPI,
    Standalone,
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Mode::Kafka => "Kafka",
            Mode::MPI => "MPI",
            Mode::Standalone => "Standalone",
        };
        write!(f, "{}", str)
    }
}

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long, value_name = "FILE", help = "Use a config file to run the simulation")]
    config: Option<String>,

    // #[arg(short, long, default_value_t = false)]
    // #[arg(help = "Start the engine in daemon mode. It will wait for messages from Kafka. \
    //         Specifying this flag will cause the config argument to be ignored")]
    // daemon: bool,
    #[clap(value_enum)]
    #[arg(short, long, default_value_t = Mode::Kafka)]
    #[arg(help = "start the engine with a particular implementation- Kafka or MPI")]
    mode: Mode,

    #[arg(short, long)]
    #[arg(help = "An identifier for the engine. Needed in daemon mode when running a larger simulation \
            distributed across multiple engines.")]
    id: Option<String>,

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

    println!("{:?}", args.mode);
    let mode = args.mode;
    let has_named_engine = args.id.is_some();
    let default_engine_id = "default_engine".to_string();
    let engine_id = args.id.unwrap_or(default_engine_id);
    let number_of_threads = args.threads;
    let run_mode = match mode {
        Mode::Kafka => RunMode::MultiEngine { mode: MultiEngineMode::Kafka },
        Mode::MPI => RunMode::MultiEngine { mode: MultiEngineMode::MPI },
        Mode::Standalone => RunMode::Standalone,
    };
    // let run_mode = if daemon && has_named_engine {
    //     RunMode::MultiEngine { mode: MultiEngineMode::Kafka}
    // } else if daemon {
    //     RunMode::SingleDaemon
    // } else {
    //     RunMode::Standalone
    // };

    let disease_handler: Option<Disease> = None;

    let _tracer = init_tracer().unwrap();

    let span: Span = _tracer.start("root");
    let cx: Context = Context::current_with_span(span);

    match mode {
        Mode::Kafka => {
            EngineApp::start_in_daemon(&engine_id, &run_mode, disease_handler, number_of_threads).with_context(cx).await
        }
        Mode::MPI => {
            println!("in multi-engine mode");
            let mut universe = mpi::initialize().unwrap();

            // Try to attach a buffer.
            universe.set_buffer_size(BUFFER_SIZE);
            assert_eq!(universe.buffer_size(), BUFFER_SIZE);

            let world = universe.world();
            let rank = world.rank();
            let default_config_path = "engine/config/simulation.json".to_string();
            let config_path = args.config.unwrap_or(default_config_path);
            let config = Configuration::read(&config_path).expect("Error while reading config");
            config.validate();
            let config_per_engine = config.get_engine_configs();
            let index: usize = (rank) as usize;
            let self_config: &EngineConfig = config_per_engine.get(index).unwrap();
            let travel_plan: &TravelPlanConfig = config.get_travel_plan();
            let engine_config = &self_config.config;
            let engine_id = String::from(&self_config.engine_id);
            // FileLogger::init(engine_id.to_string()).unwrap();

            let mpi_transport = MpiTransport::new(engine_id.to_string(), &travel_plan.get_regions());
            EngineApp::start_with_mpi(
                engine_id.clone(),
                engine_config.clone(),
                &run_mode,
                Some(travel_plan.clone()),
                disease_handler,
                Some(mpi_transport),
                number_of_threads,
                // output_dir,
            )
            .with_context(cx)
            .await;
        }
        Mode::Standalone => {
            let default_config_path = "config/default.json".to_string();
            let config_file = args.config.unwrap_or(default_config_path);
            let config = Config::read(&config_file).expect("Failed to read config file");
            EngineApp::start_standalone(config, &run_mode, disease_handler, number_of_threads).await;
        }
    }

    // if daemon {
    //     EngineApp::start_in_daemon(&engine_id, &run_mode, disease_handler, number_of_threads).with_context(cx).await;
    // } else {
    //     let default_config_path = "config/default.json".to_string();
    //     let config_file = args.config.unwrap_or(default_config_path);
    //     let config = Config::read(&config_file).expect("Failed to read config file");
    //     EngineApp::start_standalone(config, &run_mode, disease_handler, number_of_threads).await;
    // }
}
