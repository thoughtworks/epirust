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

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

mod allocation_map;
mod citizen;
mod disease_state_machine;
mod engine_app;
mod epidemiology_simulation;
mod helpers;
mod interventions;
mod kafka;
mod listeners;
mod models;
mod population;
mod run_mode;
mod state_machine;
mod tick;
mod travel;
mod utils;

pub mod geography;

pub use engine_app::EngineApp;
pub use run_mode::RunMode;
pub use state_machine::*;
