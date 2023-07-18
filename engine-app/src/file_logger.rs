/*
 * EpiRust
 * Copyright (c) 2023  ThoughtWorks, Inc.
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

use log::{LevelFilter, SetLoggerError};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Handle;

pub const EPIRUST_LOG_HOME_ENV_VAR: &str = "EPIRUST_LOG_HOME";

pub struct FileLogger;

impl FileLogger {
    pub fn init(engine_id: String) -> Result<Handle, SetLoggerError> {
        let result = std::env::var(EPIRUST_LOG_HOME_ENV_VAR);

        let default_log_home_dir: String = "/tmp".to_string();

        let log_home_dir = match result {
            Ok(path) => path,
            Err(_) => {
                println!("Environment variable EPIRUST_LOG_HOME is not set, taking default value {}", default_log_home_dir);
                default_log_home_dir
            }
        };

        let logfile = FileAppender::builder()
            // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
            .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
            .build(format!("{}/{}", log_home_dir, engine_id))
            .unwrap();

        // Log Trace level output to file where trace is the default level
        // and the programmatically specified level to stderr.
        let config = log4rs::config::Config::builder()
            .appender(Appender::builder().build("logfile", Box::new(logfile)))
            .build(Root::builder().appender("logfile").build(LevelFilter::Trace))
            .unwrap();

        log4rs::init_config(config)
    }
}
