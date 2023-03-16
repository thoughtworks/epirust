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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct CommutePlan {
    regions: Vec<String>,
    matrix: Vec<Vec<u32>>,
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
    pub fn new(regions: Vec<String>, matrix: Vec<Vec<u32>>) -> Self {
        CommutePlan { regions, matrix }
    }

    pub fn get_total_commuters_by_region(&self, from_region: &str) -> Vec<(String, u32)> {
        let mut commuters_by_region: Vec<(String, u32)> = vec![];
        for region in &self.regions {
            commuters_by_region.push((region.to_string(), self.get_outgoing(from_region, region)))
        }
        commuters_by_region
    }
}

#[cfg(test)]
mod test {
    use crate::models::travel_plan::TravelPlan;
    use crate::models::CommutePlan;
    use crate::utils::test_utils;

    #[test]
    fn test_getters() {
        let num_of_regions: usize = test_utils::random_number(0, 100);

        let regions: Vec<String> = (0..num_of_regions).map(|_| test_utils::random_string(num_of_regions)).collect();
        let matrix: Vec<Vec<u32>> = (0..num_of_regions).map(|_| test_utils::random_int_vec(num_of_regions)).collect();

        let commute_plan = CommutePlan::new(regions.clone(), matrix.clone());
        assert_eq!(commute_plan.get_regions(), &regions);
        assert_eq!(commute_plan.get_matrix(), &matrix);
    }

    #[test]
    fn should_get_total_commuters_by_region() {
        let regions = vec!["region1".to_string(), "region2".to_string(), "region3".to_string(), "region4".to_string()];
        let matrix: Vec<Vec<u32>> = vec![vec![0, 5, 8, 9], vec![9, 0, 8, 9], vec![7, 5, 0, 9], vec![2, 5, 6, 0]];
        let commute_plan = CommutePlan::new(regions, matrix);

        let actual_res = commute_plan.get_total_commuters_by_region("region1");
        let expected_res: Vec<(String, u32)> =
            vec![("region1".to_string(), 0), ("region2".to_string(), 5), ("region3".to_string(), 8), ("region4".to_string(), 9)];
        assert_eq!(actual_res, expected_res);

        let actual_res = commute_plan.get_total_commuters_by_region("region2");
        let expected_res: Vec<(String, u32)> =
            vec![("region1".to_string(), 9), ("region2".to_string(), 0), ("region3".to_string(), 8), ("region4".to_string(), 9)];
        assert_eq!(actual_res, expected_res);

        let actual_res = commute_plan.get_total_commuters_by_region("region3");
        let expected_res: Vec<(String, u32)> =
            vec![("region1".to_string(), 7), ("region2".to_string(), 5), ("region3".to_string(), 0), ("region4".to_string(), 9)];
        assert_eq!(actual_res, expected_res);

        let actual_res = commute_plan.get_total_commuters_by_region("region4");
        let expected_res: Vec<(String, u32)> =
            vec![("region1".to_string(), 2), ("region2".to_string(), 5), ("region3".to_string(), 6), ("region4".to_string(), 0)];
        assert_eq!(actual_res, expected_res);
    }

    #[test]
    fn should_get_position_of_the_region_from_regions_list() {
        let regions = vec!["reg1".to_string(), "reg2".to_string(), "reg3".to_string(), "reg4".to_string()];
        let commute_plan = CommutePlan::new(regions, Vec::new());

        assert_eq!(commute_plan.get_position("reg1"), 0);
        assert_eq!(commute_plan.get_position("reg2"), 1);
        assert_eq!(commute_plan.get_position("reg3"), 2);
        assert_eq!(commute_plan.get_position("reg4"), 3);

        let result = test_utils::panic_into_result(|| commute_plan.get_position("reg5"));

        assert_eq!(test_utils::panic_into_result(|| panic!("this")), Err("this".to_string()));
        assert_eq!(result, Err("Could not find region with name reg5".to_string()))
    }

    #[test]
    fn should_get_incoming_regions_count() {
        let regions = vec!["reg1".to_string(), "reg2".to_string(), "reg3".to_string(), "reg4".to_string()];
        let matrix: Vec<Vec<u32>> = vec![vec![0, 50, 60, 90], vec![54, 0, 0, 79], vec![90, 52, 0, 48], vec![0, 0, 64, 0]];
        let commute_plan = CommutePlan::new(regions, matrix);

        assert_eq!(commute_plan.incoming_regions_count("reg1"), 2);
        assert_eq!(commute_plan.incoming_regions_count("reg2"), 2);
        assert_eq!(commute_plan.incoming_regions_count("reg3"), 2);
        assert_eq!(commute_plan.incoming_regions_count("reg4"), 3);
    }

    #[test]
    fn should_get_total_incoming_commuters_to_the_particular_region() {
        let regions = vec!["reg1".to_string(), "reg2".to_string(), "reg3".to_string(), "reg4".to_string()];
        let matrix: Vec<Vec<u32>> = vec![vec![0, 50, 60, 90], vec![14, 0, 0, 79], vec![46, 52, 0, 48], vec![10, 0, 64, 0]];
        let commute_plan = CommutePlan::new(regions, matrix);

        assert_eq!(commute_plan.get_total_incoming("reg1"), 70);
        assert_eq!(commute_plan.get_total_incoming("reg2"), 102);
        assert_eq!(commute_plan.get_total_incoming("reg3"), 124);
        assert_eq!(commute_plan.get_total_incoming("reg4"), 217);
    }

    #[test]
    fn should_get_total_outgoing_commuters_from_the_particular_region() {
        let regions = vec!["reg1".to_string(), "reg2".to_string(), "reg3".to_string(), "reg4".to_string()];
        let matrix: Vec<Vec<u32>> = vec![vec![0, 12, 60, 90], vec![14, 0, 30, 12], vec![16, 22, 0, 48], vec![10, 50, 34, 0]];
        let commute_plan = CommutePlan::new(regions, matrix);

        assert_eq!(commute_plan.get_total_outgoing("reg1"), 162);
        assert_eq!(commute_plan.get_total_outgoing("reg2"), 56);
        assert_eq!(commute_plan.get_total_outgoing("reg3"), 86);
        assert_eq!(commute_plan.get_total_outgoing("reg4"), 94);
    }

    #[test]
    fn should_get_outgoing_commuters_from_one_region_to_another() {
        let regions = vec!["reg1".to_string(), "reg2".to_string(), "reg3".to_string(), "reg4".to_string()];
        let matrix: Vec<Vec<u32>> = vec![vec![0, 19, 65, 80], vec![24, 0, 40, 42], vec![36, 52, 0, 38], vec![50, 30, 64, 0]];
        let commute_plan = CommutePlan::new(regions, matrix);

        assert_eq!(commute_plan.get_outgoing("reg1", "reg2"), 19);
        assert_eq!(commute_plan.get_outgoing("reg1", "reg3"), 65);
        assert_eq!(commute_plan.get_outgoing("reg1", "reg4"), 80);
        assert_eq!(commute_plan.get_outgoing("reg2", "reg1"), 24);
        assert_eq!(commute_plan.get_outgoing("reg2", "reg3"), 40);
        assert_eq!(commute_plan.get_outgoing("reg2", "reg4"), 42);
        assert_eq!(commute_plan.get_outgoing("reg3", "reg1"), 36);
        assert_eq!(commute_plan.get_outgoing("reg3", "reg2"), 52);
        assert_eq!(commute_plan.get_outgoing("reg3", "reg4"), 38);
        assert_eq!(commute_plan.get_outgoing("reg4", "reg1"), 50);
        assert_eq!(commute_plan.get_outgoing("reg4", "reg2"), 30);
        assert_eq!(commute_plan.get_outgoing("reg4", "reg3"), 64);
    }
}
