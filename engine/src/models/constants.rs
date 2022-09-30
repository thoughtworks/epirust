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

use common::models::custom_types::{Day, Hour, Percentage, Size};

pub const HOUSE_AREA_RELATIVE_SIZE: Percentage = 0.4;
pub const TRANSPORT_AREA_RELATIVE_SIZE: Percentage = 0.2;
pub const WORK_AREA_RELATIVE_SIZE: Percentage = 0.2;
pub const INITIAL_HOSPITAL_RELATIVE_SIZE: Percentage = 0.1;

pub const NUMBER_OF_HOURS: Hour = 24;
pub const ROUTINE_START_TIME: Hour = 0;
pub const SLEEP_START_TIME: Hour = 1;
pub const SLEEP_END_TIME: Hour = 6;
pub const ROUTINE_TRAVEL_START_TIME: Hour = 7;
pub const ROUTINE_WORK_TIME: Hour = 8;
pub const ROUTINE_TRAVEL_END_TIME: Hour = 17;
pub const ROUTINE_WORK_END_TIME: Hour = 16;
pub const ROUTINE_END_TIME: Hour = 23;
pub const NON_WORKING_TRAVEL_END_TIME: Hour = 12;

pub const HOURS_IN_A_DAY: Hour = 24;
pub const QUARANTINE_DAYS: Day = 14;
pub const IMMUNITY_RANGE: [i32; 5] = [-2, -1, 0, 1, 2];
pub const RANGE_FOR_EXPOSED: [i32; 3] = [-1, 0, 1];

pub const HOSPITAL_STAFF_PERCENTAGE: Percentage = 0.002;

pub const HOME_SIZE: Size = 2;
pub const OFFICE_SIZE: Size = 10;

//TODO: Quick fix, move these field to input parameters
pub const ASYMPTOMATIC_LAST_DAY: Day = 9;
pub const MILD_INFECTED_LAST_DAY: Day = 12;
