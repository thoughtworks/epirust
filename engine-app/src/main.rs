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

use clap::{App, Arg};
use common::config::Config;
use common::disease::Disease;
use engine::{EngineApp, RunMode};

#[tokio::main]
async fn main() {
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

    let disease_handler: Option<Disease> = None;
    if daemon {
        EngineApp::start_in_daemon(engine_id, &run_mode, disease_handler).await;
    } else {
        let config_file = matches.value_of("config").unwrap_or("config/default.json");
        let config = Config::read(config_file).expect("Failed to read config file");
        EngineApp::start_standalone(config, &run_mode, disease_handler).await;
    }
}
