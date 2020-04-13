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


#[derive(Debug, Serialize, Deserialize)]
pub struct TravelPlan {
    regions: Vec<String>,
    matrix: Vec<Vec<i32>>,
}

impl TravelPlan {

    pub fn validate_regions(&self, regions: &Vec<String>) -> bool {
        regions.len() == self.regions.len() &&
            regions.iter().map(|region| self.regions.contains(region))
                .fold(true, |acc, x| acc && x)
    }

    pub fn get_regions(&self) -> &Vec<String> {
        &self.regions
    }

    #[cfg(test)]
    pub fn get_matrix(&self) -> &Vec<Vec<i32>> {
        &self.matrix
    }
}

#[cfg(test)]
mod tests {
    use crate::config::Config;

    #[test]
    fn should_validate_regions() {
        let config = Config::read("config/test/travel_plan.json").unwrap();
        let travel_plan = config.get_travel_plan();
        assert!(travel_plan.validate_regions(&vec!["engine1".to_string(), "engine2".to_string(),
                                                  "engine3".to_string()]));
        assert!(travel_plan.validate_regions(&vec!["engine3".to_string(), "engine2".to_string(),
                                                  "engine1".to_string()]));
        assert!(!travel_plan.validate_regions(&vec!["engine3".to_string()]));
        assert!(!travel_plan.validate_regions(&vec!["engine1".to_string(), "engine2".to_string(),
                                                  "engine3".to_string(), "engine4".to_string()]));
    }
}
