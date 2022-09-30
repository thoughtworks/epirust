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

use crate::geography::{Area, Point};
use common::config::StartingInfections;
use common::models::custom_types::{Count, Percentage};

pub struct CitizensData<'a> {
    pub(crate) number_of_agents: Count,
    pub(crate) home_locations: &'a [Area],
    pub(crate) work_locations: &'a [Area],
    pub(crate) public_transport_locations: &'a [Point],
    pub(crate) public_transport_percentage: Percentage,
    pub(crate) working_percentage: Percentage,
    pub(crate) starting_infections: &'a StartingInfections,
    pub(crate) region: String,
}

impl<'a> CitizensData<'a> {
    pub fn new(
        region: String,
        number_of_agents: Count,
        home_locations: &'a [Area],
        work_locations: &'a [Area],
        public_transport_locations: &'a [Point],
        public_transport_percentage: Percentage,
        working_percentage: Percentage,
        starting_infections: &'a StartingInfections,
    ) -> Self {
        CitizensData {
            region,
            number_of_agents,
            home_locations,
            work_locations,
            public_transport_locations,
            public_transport_percentage,
            working_percentage,
            starting_infections,
        }
    }
}
