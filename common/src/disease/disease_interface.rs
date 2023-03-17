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

use crate::models::custom_types::{Day, Hour, Percentage};

pub trait DiseaseInterface {
    fn is_to_be_hospitalized(&self, infection_day: Day) -> bool;
    fn get_last_day(&self) -> Day;
    fn deceased(&mut self) -> bool;
    fn get_percentage_asymptomatic_population(&self) -> Percentage;
    fn get_percentage_severe_infected_population(&self) -> Percentage;
    fn get_exposed_duration(&self) -> Hour;
    fn get_pre_symptomatic_duration(&self) -> Hour;
}
