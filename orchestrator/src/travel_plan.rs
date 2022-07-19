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
    pub enabled: bool,
    matrix: Option<Vec<Vec<u32>>>,
    start_migration_hour: Option<u32>,
    end_migration_hour: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Commute {
    pub enabled: bool,
    pub matrix: Option<Vec<Vec<u32>>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TravelPlan {
    regions: Vec<String>,
    pub migration: Migration,
    pub commute: Commute,
    // matrix: Vec<Vec<i32>>,
    // #[serde(default)]
    // lockdown_travellers: i32,
}


#[derive(Clone, Debug, Deserialize)]
pub struct CommutePlan {
    pub regions: Vec<String>,
    pub matrix: Vec<Vec<u32>>,
}

impl CommutePlan {
    pub fn get_total_outgoing(&self, region_id: &String) -> u32 {
        let from_index = self.get_position(region_id);
        let row = self.matrix.get(from_index).unwrap();
        row.iter().sum()
    }

    pub fn get_total_incoming(&self, region_id: &String) -> u32 {
        let region_index = self.get_position(region_id);
        self.column(region_index).sum()
    }

    fn get_position(&self, engine_id: &String) -> usize {
        self.regions.iter().position(|i| i.eq(engine_id))
            .expect("Could not find region with specified name")
    }

    fn column(&self, index: usize) -> impl Iterator<Item=u32> + '_ {
        self.matrix.iter().map(move |row| *row.get(index).unwrap())
    }

}


#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct MigrationPlan {
    regions: Vec<String>,
    matrix: Vec<Vec<u32>>,
}

impl MigrationPlan {
    pub fn new(regions: Vec<String>, matrix: Vec<Vec<u32>>) -> MigrationPlan {
        MigrationPlan { regions, matrix }
    }

    pub fn get_total_outgoing(&self, engine_id: &str) -> u32 {
        let index = self.get_position(engine_id);
        let row = self.matrix.get(index).unwrap();
        row.iter().sum()
    }

    pub fn get_total_incoming(&self, engine_id:&str) -> u32 {
        let index = self.get_position(engine_id);
        self.column(index).sum()
    }

    fn get_position(&self, engine_id: &str) -> usize {
        self.regions.iter().position(|i| i.eq(engine_id))
            .expect("Could not find region with specified name")
    }

    fn column(&self, index: usize) -> impl Iterator<Item=u32> + '_ {
        self.matrix.iter().map(move |row| *row.get(index).unwrap())
    }
}


impl TravelPlan {
    pub fn get_regions(&self) -> &Vec<String> {
        &self.regions
    }

    pub fn validate_regions(&self, regions: &[String]) -> bool {
        regions.len() == self.regions.len() &&
            regions.iter().map(|region| self.regions.contains(region))
                .all(|x| x)
    }

    pub fn commute_plan(&self) -> CommutePlan {
        CommutePlan { regions: self.regions.clone(), matrix: self.commute.matrix.as_ref().unwrap().clone() }
    }

    pub fn migration_plan(&self) -> MigrationPlan {
        MigrationPlan::new(self.regions.clone(), self.migration.matrix.as_ref().unwrap().clone())
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
    use crate::config::Configuration;

    #[test]
    fn should_validate_regions() {
        let config = Configuration::read("config/test/travel_plan.json").unwrap();
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
