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

use crate::config::{Config, TravelPlanConfig};
use crate::epidemiology_simulation::Epidemiology;
use crate::run_mode::RunMode;
use crate::state_machine::DiseaseHandler;
use std::path::Path;

pub struct EngineApp;

impl EngineApp {
    pub async fn start<T: DiseaseHandler + Sync>(
        engine_id: String,
        config: Config,
        run_mode: &RunMode,
        travel_plan_config: Option<TravelPlanConfig>,
        dsh: Option<T>,
        threads: u32,
        output_dir_path: &Path,
    ) {
        if dsh.is_none() {
            let disease = config.get_disease();
            let mut epidemiology = Epidemiology::new(config, travel_plan_config, engine_id, run_mode, disease, output_dir_path);
            epidemiology.run(run_mode, threads).await;
        } else {
            let mut epidemiology =
                Epidemiology::new(config, travel_plan_config, engine_id, run_mode, dsh.unwrap(), output_dir_path);
            epidemiology.run(run_mode, threads).await;
        }
        info!("Done");
    }
}
