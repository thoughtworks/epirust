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
use common::config::Config;
use common::disease::Disease;
use engine::{EngineApp, RunMode};
use opentelemetry::sdk::trace::{config, Span};
use opentelemetry::sdk::Resource;
use opentelemetry::trace::{FutureExt, TraceContextExt, TraceError, Tracer};
use opentelemetry::{global, sdk, Context, KeyValue};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long, value_name = "FILE", help = "Use a config file to run the simulation")]
    config: Option<String>,

    #[arg(short, long, default_value_t = false)]
    #[arg(help = "Start the engine in daemon mode. It will wait for messages from Kafka. \
            Specifying this flag will cause the config argument to be ignored")]
    daemon: bool,

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

    let daemon = args.daemon;
    let has_named_engine = args.id.is_some();
    let default_engine_id = "default_engine".to_string();
    let engine_id = args.id.unwrap_or(default_engine_id);
    let number_of_threads = args.threads;
    let run_mode = if daemon && has_named_engine {
        RunMode::MultiEngine { engine_id: engine_id.to_string() }
    } else if daemon {
        RunMode::SingleDaemon
    } else {
        RunMode::Standalone
    };

    let disease_handler: Option<Disease> = None;

    let _tracer = init_tracer().unwrap();

    let span: Span = _tracer.start("root");
    let cx: Context = Context::current_with_span(span);

    if daemon {
        EngineApp::start_in_daemon(&engine_id, &run_mode, disease_handler, number_of_threads).with_context(cx).await;
    } else {
        let default_config_path = "config/default.json".to_string();
        let config_file = args.config.unwrap_or(default_config_path);
        let config = Config::read(&config_file).expect("Failed to read config file");
        EngineApp::start_standalone(config, &run_mode, disease_handler, number_of_threads).await;
    }
}
