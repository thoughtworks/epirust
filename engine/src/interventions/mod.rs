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

use crate::interventions::vaccination::{VaccinateConfig, VaccinateIntervention};
use crate::interventions::lockdown::{LockdownConfig, LockdownIntervention};
use crate::interventions::hospital::{BuildNewHospitalConfig, BuildNewHospital};

pub mod hospital;
pub mod lockdown;
pub mod vaccination;
pub mod intervention_type;

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
#[serde(rename = "Intervention")]
pub enum InterventionConfig {
    Vaccinate(VaccinateConfig),
    Lockdown(LockdownConfig),
    BuildNewHospital(BuildNewHospitalConfig),
}

pub struct Interventions {
    pub vaccinate: VaccinateIntervention,
    pub lockdown: LockdownIntervention,
    pub build_new_hospital: BuildNewHospital,
}
