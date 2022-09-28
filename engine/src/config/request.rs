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

use crate::Config;
use crate::config::TravelPlanConfig;

#[derive(Debug, Deserialize)]
pub struct SimulationRequest {
    pub(crate) sim_id: String,
    #[serde(flatten)]
    pub(crate) config: Config,
}

#[derive(Debug, Deserialize)]
pub struct SimRequestByEngine {
    pub(crate) engine_id: String,
    pub(crate) config: SimulationRequest,
}

#[derive(Debug, Deserialize)]
pub struct MultiSimRequest {
    pub(crate) engine_configs: Vec<SimRequestByEngine>,
    pub(crate) travel_plan: TravelPlanConfig,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Request {
    SimulationRequest(SimulationRequest),
    MultiSimRequest(MultiSimRequest),
}
