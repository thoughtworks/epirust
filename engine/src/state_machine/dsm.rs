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

use common::disease::Disease;
use common::models::custom_types::Hour;
use common::utils::RandomWrapper;

use crate::allocation_map::CitizenLocationMap;
use crate::citizen::Citizen;
use crate::geography::Point;
use crate::state_machine::State;

trait DSM {
    fn next_state(
        &self,
        sim_hr: Hour,
        cell: Point,
        citizen: &Citizen,
        disease: &Disease,
        map: &CitizenLocationMap,
        rng: &mut RandomWrapper,
    ) -> State;
}
