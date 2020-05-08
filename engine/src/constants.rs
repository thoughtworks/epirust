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

pub const HOUSE_AREA_RELATIVE_SIZE: f32 = 0.4;
pub const TRANSPORT_AREA_RELATIVE_SIZE: f32 = 0.1;
pub const WORK_AREA_RELATIVE_SIZE: f32 = 0.2;
pub const INITIAL_HOSPITAL_RELATIVE_SIZE: f32 = 0.1;

pub const NUMBER_OF_HOURS: i32 = 24;
pub const ROUTINE_START_TIME: i32 = 0;
pub const SLEEP_START_TIME: i32 = 1;
pub const SLEEP_END_TIME: i32 = 6;
pub const ROUTINE_TRAVEL_START_TIME: i32 = 7;
pub const ROUTINE_WORK_TIME: i32 = 8;
pub const ROUTINE_TRAVEL_END_TIME: i32 = 17;
pub const ROUTINE_WORK_END_TIME: i32 = 18;
pub const ROUTINE_END_TIME: i32 = 23;
pub const NON_WORKING_TRAVEL_END_TIME: i32 = 12;

pub const HOURS_IN_A_DAY: i32 = 24;
pub const QUARANTINE_DAYS: i32 = 14;
pub const IMMUNITY_RANGE: [i32; 5] = [-2, -1, 0, 1, 2];

pub const HOSPITAL_STAFF_PERCENTAGE:f64 = 0.02;

pub const HOME_SIZE: i32 = 2;
pub const OFFICE_SIZE: i32 = 10;

//TODO: Quick fix, move these field to input parameters
pub const ASYMPTOMATIC_LAST_DAY: i32 = 9;
pub const MILD_INFECTED_LAST_DAY: i32 = 12;