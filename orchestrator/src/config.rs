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

use serde_json::Value;
use validator::Validate;
use crate::config::Population::Auto;

use crate::custom_types::{Count, Percentage, Size, validate_percentage};
use crate::travel_plan::TravelPlan;

pub const TRANSPORT_AREA_RELATIVE_SIZE: Percentage = 0.1;

pub fn read_simulation_conf(filename: &str) -> String {
    let reader = File::open(filename).unwrap();
    let config: Value = serde_json::from_reader(reader).unwrap();
    let sim = config.as_object().unwrap();
    serde_json::to_string(sim).unwrap()
}

pub fn get_hours(filename: &str) -> i64 {
    let reader = File::open(filename).unwrap();
    let config: Value = serde_json::from_reader(reader).unwrap();
    let sim = config.get("engine_configs").unwrap().as_array().unwrap();
    let hours = sim[0].get("config").unwrap().get("hours");
    hours.unwrap().as_i64().unwrap()
}


#[derive(Deserialize, Serialize)]
pub struct Configuration {
    engine_configs: Vec<EngineConfig>,
    travel_plan: TravelPlan,
}

impl Configuration {
    pub fn get_travel_plan(&self) -> &TravelPlan {
        &self.travel_plan
    }

    pub fn get_engine_ids(&self) -> Vec<String> {
        self.engine_configs.iter().map(|s| s.engine_id.clone()).collect()
    }

    pub fn read(filename: &str) -> Result<Configuration, Box<dyn Error>> {
        let reader = File::open(filename)?;
        let config: Configuration = serde_json::from_reader(reader)?;
        if !config.travel_plan.validate_regions(&config.get_engine_ids()) {
            panic!("Engine names should match regions in travel plan");
        }
        Ok(config)
    }

    pub fn validate(&self) {
        self.engine_configs.iter().for_each(|eng_conf: &EngineConfig| {
            let population = &eng_conf.config.population;
            let grid_size = &eng_conf.config.geography_parameters.grid_size;
            let min_accepted_ratio = 3;
            let travel_plan = self.get_travel_plan();

            let mut total_population = 0;

            let transport_area_row_size = (*grid_size as f64 * TRANSPORT_AREA_RELATIVE_SIZE).ceil() as u32 - 1;
            let total_number_of_transport_cells =  transport_area_row_size * grid_size;


            let mut  total_number_of_agents = 0;
            let mut public_transport_percentage : f64 = 0.0;
            //TODO: Handled only for Auto population right now, Add CSV support
            if let Auto(x) = population {
                total_population += &x.number_of_agents;
                total_number_of_agents += &x.number_of_agents;
                public_transport_percentage = x.public_transport_percentage;
            }

            if travel_plan.commute.enabled {
                let commute_plan = travel_plan.commute_plan();
                let incoming_commuters = commute_plan.get_total_incoming(&eng_conf.engine_id);
                let outgoing_commuters = commute_plan.get_total_outgoing(&eng_conf.engine_id);

                debug!("For engine id - {}, Total incoming commuters: {}, Total outgoing commuters: {}", eng_conf.engine_id ,incoming_commuters, outgoing_commuters);

                let total_commuters_with_public_transport = (total_number_of_agents as f64 * public_transport_percentage).ceil() as u32 - outgoing_commuters +  incoming_commuters;

                if total_commuters_with_public_transport > total_number_of_transport_cells {
                    panic!("For engine id - {}, Incoming commuters are more than engine transport capacity", eng_conf.engine_id);
                }

                total_population += incoming_commuters - outgoing_commuters;
            }

            if travel_plan.migration.enabled {
                let migration_plan = travel_plan.migration_plan();
                let incoming_migrators = migration_plan.get_total_incoming(&eng_conf.engine_id);
                let outgoing_migrators = migration_plan.get_total_outgoing(&eng_conf.engine_id);

                debug!("Total incoming migrators: {}, Total outgoing migrators: {}", incoming_migrators, outgoing_migrators);

                total_population += incoming_migrators - outgoing_migrators;
            }

            let actual_ratio = (grid_size * grid_size) / total_population;
            if actual_ratio < min_accepted_ratio {
                debug!("grid size: {}, total population: {}", actual_ratio, total_population);
                panic!("{}: Not enough space to accumulate the migrators/commuters", eng_conf.engine_id);
            }
        });
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Validate)]
struct Config {
    population: Population,
    geography_parameters: GeographyParameters,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Validate)]
pub struct GeographyParameters {
    pub grid_size: Size,
    #[validate(custom = "validate_percentage")]
    pub hospital_beds_percentage: Percentage,
}


#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum Population {
    Csv(CsvPopulation),
    Auto(AutoPopulation),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CsvPopulation {
    pub file: String,
    pub cols: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone, Validate)]
pub struct AutoPopulation {
    pub number_of_agents: Count,
    #[validate(custom = "validate_percentage")]
    pub public_transport_percentage: Percentage,
    #[validate(custom = "validate_percentage")]
    pub working_percentage: Percentage,
}

// just a struct for easier parsing
#[derive(Deserialize, Serialize)]
struct EngineConfig {
    engine_id: String,
    config: Config,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_read_config() {
        let config = Configuration::read("config/test/travel_plan.json").unwrap();
        let travel_plan = config.get_travel_plan();

        assert_eq!(travel_plan.get_regions(), &vec!["engine1".to_string(), "engine2".to_string(),
                                                    "engine3".to_string()]);
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
