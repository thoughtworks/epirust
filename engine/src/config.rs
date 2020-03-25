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

use crate::disease::{Disease, DiseaseOverride};
use crate::interventions::{Intervention, Vaccinate};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    population: Population,
    disease: Disease,
    #[serde(default)]
    disease_overrides: Vec<DiseaseOverride>,
    grid_size: i32,
    hours: i32,
    interventions: Vec<Intervention>,
    output_file: Option<String>,
    #[serde(default)]
    enable_citizen_state_messages: bool,
}

impl Config {
    pub fn get_disease(&self) -> Disease {
        self.disease
    }

    pub fn get_population(&self) -> Population {
        self.population.clone()
    }

    pub fn get_disease_overrides(&self) -> Vec<DiseaseOverride> {
        self.disease_overrides.clone()
    }

    pub fn get_grid_size(&self) -> i32 {
        self.grid_size
    }

    pub fn get_hours(&self) -> i32 {
        self.hours
    }

    pub fn get_interventions(&self) -> Vec<Intervention> {
        self.interventions.clone()
    }

    pub fn get_output_file(&self) -> Option<String> {
        self.output_file.clone()
    }

    pub fn enable_citizen_state_messages(&self) -> bool {
        self.enable_citizen_state_messages
    }

    pub fn new(population: Population, disease: Disease, disease_overrides: Vec<DiseaseOverride>,
               grid: i32, hours: i32, interventions: Vec<Intervention>, output_file: Option<String>)
               -> Config {
        Config {
            population,
            disease,
            disease_overrides,
            grid_size: grid,
            hours,
            interventions,
            output_file,
            enable_citizen_state_messages: false,
        }
    }
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

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub struct AutoPopulation {
    pub number_of_agents: i32,
    pub public_transport_percentage: f64,
    pub working_percentage: f64,
}

pub fn read(filename: String) -> Result<Config, Box<dyn Error>> {
    let reader = File::open(filename)?;
    let v: Config = serde_json::from_reader(reader)?;
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_read_config_with_csv_population() {
        let read_config = read(String::from("config/test/csv_pop.json")).unwrap();

        let vaccinate = Vaccinate::new(5000, 0.2);
        let disease_override = DiseaseOverride::new(
            String::from("age"),
            vec!["60-64".to_string(), "65-69".to_string(), "70-74".to_string(), "75-79".to_string(), "80+".to_string()],
            Disease::new(5, 20, 40, 0.025, 0.25, 0.2));

        let population = Population::Csv(CsvPopulation {
            file: "config/pune_population.csv".to_string(),
            cols: vec![String::from("age"), String::from("sex"), String::from("working"),
                       String::from("pub_transport")],
        });

        let expected_config = Config {
            population,
            disease: Disease::new(5, 20, 40, 0.025, 0.25, 0.035),
            disease_overrides: vec![disease_override],
            grid_size: 5660,
            hours: 10000,
            interventions: vec![Intervention::Vaccinate(vaccinate)],
            output_file: None,
            enable_citizen_state_messages: false
        };

        assert_eq!(expected_config, read_config);
    }

    #[test]
    fn should_read_config_with_auto_population() {
        let read_config = read(String::from("config/test/auto_pop.json")).unwrap();

        let vaccinate = Vaccinate::new(5000, 0.2);

        let population = Population::Auto(AutoPopulation {
            number_of_agents: 10000,
            public_transport_percentage: 0.2,
            working_percentage: 0.7,
        });

        let expected_config = Config {
            population,
            disease: Disease::new(5, 20, 40, 0.025, 0.25, 0.035),
            disease_overrides: vec![],
            grid_size: 250,
            hours: 10000,
            interventions: vec![Intervention::Vaccinate(vaccinate)],
            output_file: Some("simulation_default_config".to_string()),
            enable_citizen_state_messages: false
        };

        assert_eq!(expected_config, read_config);
    }
}
