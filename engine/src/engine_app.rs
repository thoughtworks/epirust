/*
 * EpiRust
 * Copyright (c) 2022  ThoughtWorks, Inc.
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

use crate::epidemiology_simulation::Epidemiology;
use crate::kafka::kafka_consumer::KafkaConsumer;
use crate::RunMode;
use common::config::Config;

pub const STANDALONE_SIM_ID: &str = "0";

pub struct EngineApp;

impl EngineApp {
    pub async fn start_in_daemon(engine_id: &str, run_mode: &RunMode) {
        info!("Started in daemon mode");
        let consumer = KafkaConsumer::new(engine_id, &["simulation_requests"]);
        consumer.listen_loop(run_mode).await;
        info!("Done");
    }

    pub async fn start_standalone(config: Config, run_mode: &RunMode) {
        let mut epidemiology = Epidemiology::new(config, None, STANDALONE_SIM_ID.to_string(), run_mode);
        epidemiology.run(run_mode).await;
        info!("Done");
    }
}
