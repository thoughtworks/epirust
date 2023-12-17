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

use common::config::{Config, TravelPlanConfig};

use crate::epidemiology_simulation::Epidemiology;
use crate::kafka::kafka_consumer::KafkaConsumer;
use crate::run_mode::RunMode;
use crate::state_machine::DiseaseHandler;
use crate::transport::engine_handlers::NoOpEngineHandlers;
use crate::{KafkaTransport, MpiTransport};

pub const STANDALONE_SIM_ID: &str = "0";

pub struct EngineApp;

impl EngineApp {
    pub async fn start_with_mpi<T: DiseaseHandler + Sync>(
        engine_id: String,
        config: Config,
        run_mode: &RunMode,
        travel_plan_config: Option<TravelPlanConfig>,
        dsh: Option<T>,
        transport: Option<MpiTransport>,
        threads: u32,
        // output_dir_path: &Path,
    ) {
        // let transport: Option<MpiTransport> = MpiTransport::new(engine_id.clone(), );
        let engine_handlers = NoOpEngineHandlers::default();
        if dsh.is_none() {
            let disease = config.get_disease();
            let mut epidemiology =
                Epidemiology::new(engine_id, config, travel_plan_config, run_mode, disease, transport, engine_handlers);
            epidemiology.run(threads).await;
        } else {
            let mut epidemiology =
                Epidemiology::new(engine_id, config, travel_plan_config, run_mode, dsh.unwrap(), transport, engine_handlers);
            epidemiology.run(threads).await;
        }
        info!("Done");
    }

    pub async fn start_in_daemon<D: DiseaseHandler + Sync + Clone>(
        engine_id: &str,
        run_mode: &RunMode,
        dsh: Option<D>,
        threads: u32,
    ) {
        info!("Started in daemon mode");
        let consumer = KafkaConsumer::new(engine_id, &["simulation_requests"]);
        consumer.listen_loop(engine_id, run_mode, dsh, threads).await;
        info!("Done");
    }

    pub async fn start_standalone<D: DiseaseHandler + Sync>(config: Config, run_mode: &RunMode, dsh: Option<D>, threads: u32) {
        let transport: Option<KafkaTransport> = None;
        let engine_handlers = NoOpEngineHandlers::default();
        if dsh.is_none() {
            let disease = config.get_disease();
            let mut epidemiology =
                Epidemiology::new(STANDALONE_SIM_ID.to_string(), config, None, run_mode, disease, transport, engine_handlers);
            epidemiology.run(threads).await;
        } else {
            let mut epidemiology = Epidemiology::new(
                STANDALONE_SIM_ID.to_string(),
                config,
                None,
                run_mode,
                dsh.unwrap(),
                transport,
                engine_handlers,
            );
            epidemiology.run(threads).await;
        }
        info!("Done");
    }
}
