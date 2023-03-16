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
    use crate::utils::test_utils;

    #[test]
    fn test_getters() {
        let num_of_regions: usize = test_utils::random_number(0, 100);

        let regions: Vec<String> = (0..num_of_regions).map(|_| test_utils::random_string(num_of_regions)).collect();
        let matrix: Vec<Vec<u32>> = (0..num_of_regions).map(|_| test_utils::random_int_vec(num_of_regions)).collect();

        let migration_plan = MigrationPlan::new(regions.clone(), matrix.clone());
        assert_eq!(migration_plan.get_regions(), &regions);
        assert_eq!(migration_plan.get_matrix(), &matrix);
    }

    #[test]
    fn should_get_position_of_the_region_from_regions_list() {
        let regions = vec!["reg11".to_string(), "reg12".to_string(), "reg13".to_string(), "reg14".to_string()];
        let migration_plan = MigrationPlan::new(regions, Vec::new());

        assert_eq!(migration_plan.get_position("reg11"), 0);
        assert_eq!(migration_plan.get_position("reg12"), 1);
        assert_eq!(migration_plan.get_position("reg13"), 2);
        assert_eq!(migration_plan.get_position("reg14"), 3);

        let result = test_utils::panic_into_result(|| migration_plan.get_position("reg25"));

        assert_eq!(test_utils::panic_into_result(|| panic!("this")), Err("this".to_string()));
        assert_eq!(result, Err("Could not find region with name reg25".to_string()))
    }

    #[test]
    fn should_get_incoming_regions_count() {
        let regions = vec!["reg11".to_string(), "reg12".to_string(), "reg13".to_string(), "reg14".to_string()];
        let matrix: Vec<Vec<u32>> = vec![vec![0, 52, 60, 90], vec![54, 0, 40, 79], vec![90, 56, 0, 48], vec![0, 0, 64, 0]];
        let migration_plan = MigrationPlan::new(regions, matrix);

        assert_eq!(migration_plan.incoming_regions_count("reg11"), 2);
        assert_eq!(migration_plan.incoming_regions_count("reg12"), 2);
        assert_eq!(migration_plan.incoming_regions_count("reg13"), 3);
        assert_eq!(migration_plan.incoming_regions_count("reg14"), 3);
    }

    #[test]
    fn should_get_total_incoming_commuters_to_the_particular_region() {
        let regions = vec!["reg1".to_string(), "reg2".to_string(), "reg3".to_string(), "reg4".to_string()];
        let matrix: Vec<Vec<u32>> = vec![vec![0, 50, 60, 90], vec![14, 0, 2, 79], vec![46, 52, 0, 48], vec![10, 0, 64, 0]];
        let migration_plan = MigrationPlan::new(regions, matrix);

        assert_eq!(migration_plan.get_total_incoming("reg1"), 70);
        assert_eq!(migration_plan.get_total_incoming("reg2"), 102);
        assert_eq!(migration_plan.get_total_incoming("reg3"), 126);
        assert_eq!(migration_plan.get_total_incoming("reg4"), 217);
    }

    #[test]
    fn should_get_total_outgoing_commuters_from_the_particular_region() {
        let regions = vec!["reg1".to_string(), "reg2".to_string(), "reg3".to_string(), "reg4".to_string()];
        let matrix: Vec<Vec<u32>> = vec![vec![0, 14, 60, 90], vec![14, 0, 30, 12], vec![16, 22, 0, 48], vec![10, 50, 34, 0]];
        let migration_plan = MigrationPlan::new(regions, matrix);

        assert_eq!(migration_plan.get_total_outgoing("reg1"), 164);
        assert_eq!(migration_plan.get_total_outgoing("reg2"), 56);
        assert_eq!(migration_plan.get_total_outgoing("reg3"), 86);
        assert_eq!(migration_plan.get_total_outgoing("reg4"), 94);
    }

    #[test]
    fn should_get_outgoing_commuters_from_one_region_to_another() {
        let regions = vec!["reg21".to_string(), "reg22".to_string(), "reg23".to_string(), "reg24".to_string()];
        let matrix: Vec<Vec<u32>> = vec![vec![0, 39, 65, 80], vec![24, 0, 30, 42], vec![36, 22, 0, 38], vec![50, 30, 64, 0]];
        let migration_plan = MigrationPlan::new(regions, matrix);

        assert_eq!(migration_plan.get_outgoing("reg21", "reg22"), 39);
        assert_eq!(migration_plan.get_outgoing("reg21", "reg23"), 65);
        assert_eq!(migration_plan.get_outgoing("reg21", "reg24"), 80);
        assert_eq!(migration_plan.get_outgoing("reg22", "reg21"), 24);
        assert_eq!(migration_plan.get_outgoing("reg22", "reg23"), 30);
        assert_eq!(migration_plan.get_outgoing("reg22", "reg24"), 42);
        assert_eq!(migration_plan.get_outgoing("reg23", "reg21"), 36);
        assert_eq!(migration_plan.get_outgoing("reg23", "reg22"), 22);
        assert_eq!(migration_plan.get_outgoing("reg23", "reg24"), 38);
        assert_eq!(migration_plan.get_outgoing("reg24", "reg21"), 50);
        assert_eq!(migration_plan.get_outgoing("reg24", "reg22"), 30);
        assert_eq!(migration_plan.get_outgoing("reg24", "reg23"), 64);
    }
}
