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
        CommutePlan::new(self.regions.clone(), self.commute.matrix.as_ref().unwrap().clone())
    }
}

#[cfg(test)]
mod test {
    use crate::config::travel_plan_config::{Commute, Migration};
    use crate::config::TravelPlanConfig;
    use crate::models::{CommutePlan, MigrationPlan};
    use crate::utils::test_utils;

    #[test]
    fn test_getters() {
        let num_of_regions: usize = test_utils::random_number(0, 100);

        let regions: Vec<String> = (0..num_of_regions).map(|_| test_utils::random_string(num_of_regions)).collect();

        let matrix: Vec<Vec<u32>> = (0..num_of_regions).map(|_| test_utils::random_int_vec(num_of_regions)).collect();

        let is_commute_enabled = test_utils::random_bool();

        let commute = Commute { matrix: Some(matrix.clone()), enabled: is_commute_enabled };

        let start_migration_hour: u32 = test_utils::random_hour();
        let end_migration_hour: u32 = test_utils::random_hour();

        let is_migration_enabled = test_utils::random_bool();

        let migration =
            Migration { enabled: is_migration_enabled, matrix: Some(matrix.clone()), start_migration_hour, end_migration_hour };

        let travel_plan_config = TravelPlanConfig { commute, regions: regions.clone(), migration };

        assert_eq!(travel_plan_config.get_end_migration_hour(), end_migration_hour);
        assert_eq!(travel_plan_config.get_start_migration_hour(), start_migration_hour);
        assert_eq!(travel_plan_config.get_migration_matrix(), Some(matrix.clone()));
        assert_eq!(travel_plan_config.get_regions(), regions);
        assert_eq!(travel_plan_config.migration_plan(), MigrationPlan::new(regions.clone(), matrix.clone()));
        assert_eq!(travel_plan_config.commute_plan(), CommutePlan::new(regions, matrix));
        assert_eq!(travel_plan_config.commute.enabled, is_commute_enabled);
        assert_eq!(travel_plan_config.migration.enabled, is_migration_enabled);
    }

    #[test]
    fn test_validate_regions() {
        let is_commute_enabled = test_utils::random_bool();

        let commute = Commute { matrix: None, enabled: is_commute_enabled };
        let is_migration_enabled = test_utils::random_bool();

        let start_migration_hour: u32 = test_utils::random_hour();
        let end_migration_hour: u32 = test_utils::random_hour();
        let migration = Migration { enabled: is_migration_enabled, matrix: None, start_migration_hour, end_migration_hour };

        let regions: Vec<String> =
            vec!["region1".to_string(), "region2".to_string(), "region3".to_string(), "region6".to_string()];

        let travel_plan_config = TravelPlanConfig { commute, migration, regions };

        let expected_res = travel_plan_config.validate_regions(&[
            "region1".to_string(),
            "region2".to_string(),
            "region3".to_string(),
            "region6".to_string(),
        ]);

        assert!(expected_res);

        let expected_res = travel_plan_config.validate_regions(&[
            "region1".to_string(),
            "region2".to_string(),
            "region3".to_string(),
            "region6".to_string(),
            "region7".to_string(),
        ]);

        assert!(!expected_res);

        let expected_res = travel_plan_config.validate_regions(&[
            "region1".to_string(),
            "region2".to_string(),
            "region9".to_string(),
            "region6".to_string(),
        ]);

        assert!(!expected_res)
    }
}
