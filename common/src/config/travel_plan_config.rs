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

use crate::models::{CommutePlan, MigrationPlan};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Migration {
    pub enabled: bool,
    matrix: Option<Vec<Vec<u32>>>,
    start_migration_hour: u32,
    end_migration_hour: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Commute {
    pub enabled: bool,
    pub matrix: Option<Vec<Vec<u32>>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TravelPlanConfig {
    pub regions: Vec<String>,
    pub migration: Migration,
    pub commute: Commute,
}

impl TravelPlanConfig {
    pub fn get_end_migration_hour(&self) -> u32 {
        self.migration.end_migration_hour
    }

    pub fn get_start_migration_hour(&self) -> u32 {
        self.migration.start_migration_hour
    }

    pub fn get_migration_matrix(&self) -> Option<Vec<Vec<u32>>> {
        self.migration.matrix.clone()
    }

    pub fn get_regions(&self) -> Vec<String> {
        self.regions.clone()
    }

    pub fn validate_regions(&self, regions: &[String]) -> bool {
        regions.len() == self.regions.len() && regions.iter().all(|region| self.regions.contains(region))
    }

    pub fn migration_plan(&self) -> MigrationPlan {
        MigrationPlan::new(self.get_regions(), self.get_migration_matrix().unwrap())
    }

    pub fn commute_plan(&self) -> CommutePlan {
        CommutePlan { regions: self.regions.clone(), matrix: self.commute.matrix.as_ref().unwrap().clone() }
    }
}
