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

mod geography_parameters;
mod population;
mod starting_infections;
mod travel_plan_config;

pub mod intervention_config;
pub mod request;

pub use travel_plan_config::TravelPlanConfig;

use std::error::Error;
use std::fs::File;
use validator::Validate;

pub use crate::config::geography_parameters::GeographyParameters;
pub use crate::config::population::*;
pub use crate::config::starting_infections::StartingInfections;

use crate::disease::{Disease, DiseaseOverride};
use crate::models::custom_types::{Hour, Size};
use intervention_config::InterventionConfig;

#[derive(Debug, PartialEq, Serialize, Deserialize, Validate, Clone)]
pub struct Config {
    population: Population,
    disease: Option<Disease>,
    #[serde(default)]
    disease_overrides: Vec<DiseaseOverride>,
    geography_parameters: GeographyParameters,
    hours: Hour,
    interventions: Vec<InterventionConfig>,
    output_file: Option<String>,
    #[serde(default)]
    enable_citizen_state_messages: bool,
    #[serde(default)]
    starting_infections: StartingInfections,
}

impl Config {
    pub fn new(
        population: Population,
        disease: Option<Disease>,
        geography_parameters: GeographyParameters,
        disease_overrides: Vec<DiseaseOverride>,
        hours: Hour,
        interventions: Vec<InterventionConfig>,
        output_file: Option<String>,
    ) -> Config {
        Config {
            population,
            disease,
            disease_overrides,
            geography_parameters,
            hours,
            interventions,
            output_file,
            enable_citizen_state_messages: true,
            starting_infections: StartingInfections::default(),
        }
    }

    pub fn get_disease(&self) -> Disease {
        self.disease.unwrap()
    }

    pub fn get_starting_infections(&self) -> &StartingInfections {
        &self.starting_infections
    }

    pub fn get_population(&self) -> &Population {
        &self.population
    }

    pub fn get_grid_size(&self) -> Size {
        self.geography_parameters.grid_size
    }

    pub fn get_hours(&self) -> Hour {
        self.hours
    }

    pub fn get_interventions(&self) -> &Vec<InterventionConfig> {
        &self.interventions
    }

    pub fn get_output_file(&self) -> Option<String> {
        self.output_file.clone()
    }

    pub fn enable_citizen_state_messages(&self) -> bool {
        self.enable_citizen_state_messages
    }

    pub fn get_geography_parameters(&self) -> &GeographyParameters {
        &self.geography_parameters
    }

    pub fn read(filename: &str) -> Result<Config, Box<dyn Error>> {
        let reader = File::open(filename)?;
        let v: Config = serde_json::from_reader(reader)?;
        Ok(v)
    }
}

#[cfg(test)]
mod tests {
    use crate::config::intervention_config::VaccinateConfig;
    use crate::config::population::{AutoPopulation, CsvPopulation};

    use super::*;

    #[test]
    fn should_read_config_with_csv_population() {
        let read_config = Config::read("config/test/csv_pop.json").unwrap();

        let vaccinate = VaccinateConfig { at_hour: 5000, percent: 0.2 };
        let disease_override = DiseaseOverride::new(
            String::from("age"),
            vec!["60-64".to_string(), "65-69".to_string(), "70-74".to_string(), "75-79".to_string(), "80+".to_string()],
            Disease::new(5, 20, 40, 9, 12, 0.025, 0.25, 0.2, 0.3, 0.3, 48, 48),
        );

        let population = Population::Csv(CsvPopulation {
            file: "config/pune_population.csv".to_string(),
            cols: vec![String::from("age"), String::from("sex"), String::from("working"), String::from("pub_transport")],
        });

        let expected_config = Config {
            population,
            disease: Some(Disease::new(5, 20, 40, 9, 12, 0.025, 0.25, 0.035, 0.3, 0.3, 48, 48)),
            disease_overrides: vec![disease_override],
            geography_parameters: GeographyParameters::new(5660, 0.003),
            hours: 10000,
            interventions: vec![InterventionConfig::Vaccinate(vaccinate)],
            output_file: None,
            enable_citizen_state_messages: false,
            starting_infections: StartingInfections::default(),
        };

        assert_eq!(expected_config, read_config);
    }

    #[test]
    fn should_read_config_with_auto_population() {
        let read_config = Config::read("config/test/auto_pop.json").unwrap();

        let vaccinate = VaccinateConfig { at_hour: 5000, percent: 0.2 };

        let population = Population::Auto(AutoPopulation {
            number_of_agents: 10000,
            public_transport_percentage: 0.2,
            working_percentage: 0.7,
        });

        let expected_config = Config {
            population,
            disease: Some(Disease::new(5, 20, 40, 9, 12, 0.025, 0.25, 0.035, 0.3, 0.3, 48, 48)),
            disease_overrides: vec![],
            geography_parameters: GeographyParameters::new(250, 0.003),
            hours: 10000,
            interventions: vec![InterventionConfig::Vaccinate(vaccinate)],
            output_file: Some("simulation_default_config".to_string()),
            enable_citizen_state_messages: false,
            starting_infections: StartingInfections::new(2, 3, 4, 5),
        };

        assert_eq!(expected_config, read_config);
    }
}
