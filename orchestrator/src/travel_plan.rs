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


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Migration {
    matrix: Vec<Vec<i32>>,
    start_migration_hour: u32,
    end_migration_hour: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Commute {
    matrix: Vec<Vec<u32>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TravelPlan {
    regions: Vec<String>,
    migration: Migration,
    commute: Commute
    // matrix: Vec<Vec<i32>>,
    // #[serde(default)]
    // lockdown_travellers: i32,
}

impl TravelPlan {

    pub fn get_regions(&self) -> &Vec<String> {
        &self.regions
    }

    pub fn validate_regions(&self, regions: &Vec<String>) -> bool {
        regions.len() == self.regions.len() &&
            regions.iter().map(|region| self.regions.contains(region))
                .fold(true, |acc, x| acc && x)
    }

    // pub fn update_with_lockdowns(&self, lockdown_status: &HashMap<String, bool>) -> TravelPlan {
    //     let locked_regions: Vec<&String> = lockdown_status.iter().filter(|(_k, v)| **v).map(|(k, _v)| k)
    //         .collect();
    //     let mut new_travel_plan = self.clone();
    //     for region in locked_regions {
    //         new_travel_plan.apply_lockdown(region);
    //     }
    //     new_travel_plan
    // }

    // fn apply_lockdown(&mut self, region: &String) {
    //     let index = self.regions.iter().position(|i| i.eq(region)).unwrap();
    //     for i in 0..self.regions.len() {
    //         for j in 0..self.regions.len() {
    //             if i == index || j == index {
    //                 let val = *self.matrix.get_mut(i).unwrap().get_mut(j).unwrap();
    //                 if val > 0 {
    //                     *self.matrix.get_mut(i).unwrap().get_mut(j).unwrap() = self.lockdown_travellers;
    //                 }
    //             }
    //         }
    //     }
    // }

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
