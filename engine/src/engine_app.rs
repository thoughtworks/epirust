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
use crate::run_mode::RunMode;
use crate::state_machine::DiseaseHandler;
use common::config::Config;

pub const STANDALONE_SIM_ID: &str = "0";

pub struct EngineApp;

impl EngineApp {
    pub async fn start_in_daemon<T: DiseaseHandler + Sync + Clone>(
        engine_id: &str,
        run_mode: &RunMode,
        dsh: Option<T>,
        threads: u32,
    ) {
        info!("Started in daemon mode");
        let consumer = KafkaConsumer::new(engine_id, &["simulation_requests"]);
        consumer.listen_loop(run_mode, dsh, threads).await;
        info!("Done");
    }

    pub async fn start_standalone<T: DiseaseHandler + Sync>(config: Config, run_mode: &RunMode, dsh: Option<T>, threads: u32) {
        if dsh.is_none() {
            let disease = config.get_disease();
            let mut epidemiology = Epidemiology::new(config, None, STANDALONE_SIM_ID.to_string(), run_mode, disease);
            epidemiology.run(run_mode, threads).await;
        } else {
            let mut epidemiology = Epidemiology::new(config, None, STANDALONE_SIM_ID.to_string(), run_mode, dsh.unwrap());
            epidemiology.run(run_mode, threads).await;
        }
        info!("Done");
    }
}
