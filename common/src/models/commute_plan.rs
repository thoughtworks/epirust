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

use crate::models::travel_plan::TravelPlan;

#[derive(Clone, Debug, Deserialize)]
pub struct CommutePlan {
    pub regions: Vec<String>,
    pub matrix: Vec<Vec<u32>>,
}

impl TravelPlan for CommutePlan {
    fn get_regions(&self) -> &Vec<String> {
        &self.regions
    }

    fn get_matrix(&self) -> &Vec<Vec<u32>> {
        &self.matrix
    }
}

impl CommutePlan {
    pub fn get_total_commuters_by_region(&self, from_region: String) -> Vec<(String, u32)> {
        let mut commuters_by_region: Vec<(String, u32)> = vec![];
        for region in &self.regions {
            commuters_by_region.push((region.to_string(), self.get_outgoing(&from_region, region)))
        }
        commuters_by_region
    }
}
