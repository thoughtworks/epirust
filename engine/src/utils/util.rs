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

use crate::models::events::Counts;
use crate::run_mode::RunMode;
use crate::config::{Config, StartingInfections};
use crate::models::custom_types::Count;
use time::OffsetDateTime;

pub fn output_file_format(config: &Config, run_mode: &RunMode, engine_id: String) -> String {
    let format = time::format_description::parse("[year]-[month]-[day]T[hour]:[minute]:[second]").unwrap();
    let now = OffsetDateTime::now_utc();
    let mut output_file_prefix = config.get_output_file().unwrap_or_else(|| "simulation".to_string());
    if let RunMode::MultiEngine = run_mode {
        output_file_prefix = format!("{}_{}", output_file_prefix, engine_id);
    }
    format!("{}_{}", output_file_prefix, now.format(&format).unwrap())
}

pub fn counts_at_start(population: Count, start_infections: &StartingInfections) -> Counts {
    let s = population - start_infections.total();
    let e = start_infections.get_exposed();
    let i = start_infections.total_infected();
    assert_eq!(s + e + i, population);
    Counts::new(s, e, i)
}
