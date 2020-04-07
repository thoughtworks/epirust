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

use std::fs::File;

#[derive(Debug, Serialize, Deserialize)]
pub struct TravelPlan {
    regions: Vec<String>,
    matrix: Vec<Vec<i32>>,
}

impl TravelPlan {
    pub fn read(file_path: &str) -> TravelPlan {
        let file = File::open(file_path).unwrap();
        serde_json::from_reader(file).unwrap()
    }

    pub fn validate_regions(&self, regions: &Vec<String>) -> bool {
        regions.len() == self.regions.len() &&
            regions.iter().map(|region| self.regions.contains(region))
                .fold(true, |acc, x| acc && x)
    }

    pub fn get_regions(&self) -> &Vec<String> {
        &self.regions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_read() {
        let travel_plan: TravelPlan = TravelPlan::read("config/test/travel_plan.json");

        assert_eq!(travel_plan.regions, vec!["engine1".to_string(), "engine2".to_string(),
                                             "engine3".to_string()]);
        assert_eq!(travel_plan.matrix, vec![
            vec![0, 156, 10],
            vec![108, 0, 290],
            vec![90, 75, 0]
        ]);
    }

    #[test]
    fn should_validate_regions() {
        let travel_plan: TravelPlan = TravelPlan::read("config/test/travel_plan.json");
        assert!(travel_plan.validate_regions(&vec!["engine1".to_string(), "engine2".to_string(),
                                                  "engine3".to_string()]));
        assert!(travel_plan.validate_regions(&vec!["engine3".to_string(), "engine2".to_string(),
                                                  "engine1".to_string()]));
        assert!(!travel_plan.validate_regions(&vec!["engine3".to_string()]));
        assert!(!travel_plan.validate_regions(&vec!["engine1".to_string(), "engine2".to_string(),
                                                  "engine3".to_string(), "engine4".to_string()]));
    }
}
