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

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct MigrationPlan {
    pub regions: Vec<String>,
    pub matrix: Vec<Vec<u32>>,
}

impl TravelPlan for MigrationPlan {
    fn get_regions(&self) -> &Vec<String> {
        &self.regions
    }

    fn get_matrix(&self) -> &Vec<Vec<u32>> {
        &self.matrix
    }
}

impl MigrationPlan {
    pub fn new(regions: Vec<String>, matrix: Vec<Vec<u32>>) -> MigrationPlan {
        MigrationPlan { regions, matrix }
    }
}

#[cfg(test)]
mod test {
    use crate::models::migration_plan::MigrationPlan;
    use crate::models::travel_plan::TravelPlan;

    #[test]
    fn should_get_total_outgoing() {
        let travel_plan = create_travel_plan();
        assert_eq!(156 + 24, travel_plan.get_total_outgoing(&"engine1".into()));
        assert_eq!(108 + 221, travel_plan.get_total_outgoing(&"engine2".into()));
        assert_eq!(97 + 12, travel_plan.get_total_outgoing(&"engine3".into()));
    }

    // #[test]
    // fn should_get_total_incoming() {
    //     let travel_plan = create_travel_plan();
    //     assert_eq!(108 + 97, travel_plan.get_total_incoming("engine1".to_string()));
    //     assert_eq!(156 + 12, travel_plan.get_total_incoming("engine2".to_string()));
    //     assert_eq!(24 + 221, travel_plan.get_total_incoming("engine3".to_string()));
    // }

    #[test]
    fn should_get_incoming_regions_count() {
        let migration_plan = MigrationPlan {
            regions: vec!["engine1".to_string(), "engine2".to_string(), "engine3".to_string()],
            matrix: vec![vec![0, 0, 0], vec![108, 0, 0], vec![97, 12, 0]],
        };

        assert_eq!(2, migration_plan.incoming_regions_count(&"engine1".into()));
        assert_eq!(1, migration_plan.incoming_regions_count(&"engine2".into()));
        assert_eq!(0, migration_plan.incoming_regions_count(&"engine3".into()));
    }

    fn create_travel_plan() -> MigrationPlan {
        MigrationPlan {
            regions: vec!["engine1".to_string(), "engine2".to_string(), "engine3".to_string()],
            matrix: vec![vec![0, 156, 24], vec![108, 0, 221], vec![97, 12, 0]],
        }
    }
}
