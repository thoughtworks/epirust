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

use std::error::Error;
use std::fs::File;

use crate::travel_plan::TravelPlan;
use serde_json::Value;


pub fn read_simulation_conf(filename: &str) -> String {
    let reader = File::open(filename).unwrap();
    let config: Value = serde_json::from_reader(reader).unwrap();
    let sim = config.get("simulation").unwrap().as_array().unwrap();
    serde_json::to_string(sim).unwrap()
}

pub fn get_hours(filename: &str) -> i64 {
    let reader = File::open(filename).unwrap();
    let config: Value = serde_json::from_reader(reader).unwrap();
    let sim = config.get("simulation").unwrap().as_array().unwrap();
    let hours = sim[0].get("config").unwrap().get("hours");
    hours.unwrap().as_i64().unwrap()
}


#[derive(Deserialize)]
pub struct Config {
    simulation: Vec<EngineId>,
    travel_plan: TravelPlan,
}

impl Config {
    pub fn get_travel_plan(&self) -> &TravelPlan {
        &self.travel_plan
    }

    pub fn get_engine_ids(&self) -> Vec<String> {
        self.simulation.iter().map(|s| s.engine_id.clone()).collect()
    }

    pub fn read(filename: &str) -> Result<Config, Box<dyn Error>> {
        let reader = File::open(filename)?;
        let config: Config = serde_json::from_reader(reader)?;
        if !config.travel_plan.validate_regions(&config.get_engine_ids()) {
            panic!("Engine names should match regions in travel plan");
        }
        Ok(config)
    }
}

// just a struct for easier parsing
#[derive(Deserialize)]
struct EngineId {
    engine_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_read_config() {
        let config = Config::read("config/test/travel_plan.json").unwrap();
        let travel_plan = config.get_travel_plan();

        assert_eq!(travel_plan.get_regions(), &vec!["engine1".to_string(), "engine2".to_string(),
                                             "engine3".to_string()]);
        let matrix = travel_plan.get_matrix().clone();
        assert_eq!(matrix, vec![
            vec![0, 156, 10],
            vec![108, 0, 290],
            vec![90, 75, 0]
        ]);

        assert_eq!(config.get_engine_ids(), vec!["engine1".to_string(), "engine2".to_string(),
                                                 "engine3".to_string()])
    }

    #[test]
    fn should_read_config_for_engines() {
        let config_for_engines = read_simulation_conf("config/test/travel_plan.json");
        assert!(!config_for_engines.is_empty())
    }

    #[test]
    fn should_read_hours() {
        let hours = get_hours("config/test/travel_plan.json");
        assert_eq!(hours, 10000);
    }

}
