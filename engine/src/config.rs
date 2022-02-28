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
use crate::interventions::{InterventionConfig};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    population: Population,
    disease: Disease,
    #[serde(default)]
    disease_overrides: Vec<DiseaseOverride>,
    geography_parameters: GeographyParameters,
    hours: i32,
    end_of_migration: i32,
    reduced_travel_percentage: f32,
    interventions: Vec<InterventionConfig>,
    output_file: Option<String>,
    #[serde(default)]
    enable_citizen_state_messages: bool,
    #[serde(default)]
    starting_infections: StartingInfections,
}

impl Config {
    pub fn get_disease(&self) -> Disease {
        self.disease
    }

    pub fn get_starting_infections(&self) -> StartingInfections {
        self.starting_infections
    }

    pub fn get_population(&self) -> Population {
        self.population.clone()
    }

    // pub fn get_disease_overrides(&self) -> Vec<DiseaseOverride> {
    //     self.disease_overrides.clone()
    // }

    pub fn get_grid_size(&self) -> i32 {
        self.geography_parameters.grid_size
    }

    pub fn get_hours(&self) -> i32 {
        self.hours
    }

    pub fn get_end_of_migration_hour(&self) -> i32 {
        self.end_of_migration
    }

    pub fn get_reduced_travel_percentage(&self) -> f32 {
        self.reduced_travel_percentage
    }

    pub fn get_interventions(&self) -> Vec<InterventionConfig> {
        self.interventions.clone()
    }

    pub fn get_output_file(&self) -> Option<String> {
        self.output_file.clone()
    }

    pub fn enable_citizen_state_messages(&self) -> bool {
        self.enable_citizen_state_messages
    }

    pub fn get_geography_parameters(&self) -> GeographyParameters {
        self.geography_parameters.clone()
    }

    #[cfg(test)]
    pub fn new(population: Population, disease: Disease, geography_parameters: GeographyParameters, disease_overrides: Vec<DiseaseOverride>,
               end_of_migration:i32, reduced_travel_percentage:f32, hours: i32, interventions: Vec<InterventionConfig>, output_file: Option<String>)
               -> Config {
        Config {
            population,
            disease,
            disease_overrides,
            geography_parameters,
            end_of_migration,
            reduced_travel_percentage,
            hours,
            interventions,
            output_file,
            enable_citizen_state_messages: true,
            starting_infections: StartingInfections::default(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GeographyParameters {
    pub grid_size: i32,
    pub hospital_beds_percentage: f64,
}

impl GeographyParameters{
    #[cfg(test)]
    pub fn new(grid_size: i32, hospital_beds_percentage: f64) -> GeographyParameters {
        GeographyParameters{
            grid_size, hospital_beds_percentage
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

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub struct StartingInfections {
    infected_mild_asymptomatic: i32,
    infected_mild_symptomatic: i32,
    infected_severe: i32,
    exposed: i32,
}

impl StartingInfections {
    #[cfg(test)]
    pub fn new(mild_asymp: i32, mild_symp: i32, severe: i32, exposed: i32) -> StartingInfections {
        StartingInfections {
            infected_mild_asymptomatic: mild_asymp,
            infected_mild_symptomatic: mild_symp,
            infected_severe: severe,
            exposed,
        }
    }

    pub fn total(&self) -> i32 {
        self.total_infected() + self.exposed
    }

    pub fn total_infected(&self) -> i32 {
        self.infected_mild_asymptomatic + self.infected_mild_symptomatic + self.infected_severe
    }

    pub fn get_infected_mild_asymptomatic(&self) -> i32 {
        self.infected_mild_asymptomatic
    }

    pub fn get_infected_mild_symptomatic(&self) -> i32 {
        self.infected_mild_symptomatic
    }

    pub fn get_infected_severe(&self) -> i32 {
        self.infected_severe
    }

    pub fn get_exposed(&self) -> i32 {
        self.exposed
    }
}

impl Default for StartingInfections {
    fn default() -> Self {
        StartingInfections {
            infected_mild_asymptomatic: 0,
            infected_mild_symptomatic: 0,
            infected_severe: 0,
            exposed: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interventions::vaccination::VaccinateConfig;

    #[test]
    fn should_read_config_with_csv_population() {
        let read_config = read(String::from("config/test/csv_pop.json")).unwrap();

        let vaccinate = VaccinateConfig::new(5000, 0.2);
        let disease_override = DiseaseOverride::new(
            String::from("age"),
            vec!["60-64".to_string(), "65-69".to_string(), "70-74".to_string(), "75-79".to_string(), "80+".to_string()],
            Disease::new(5, 20, 40, 9, 12, 0.025, 0.25, 0.2, 0.3, 0.3, 48, 48));

        let population = Population::Csv(CsvPopulation {
            file: "config/pune_population.csv".to_string(),
            cols: vec![String::from("age"), String::from("sex"), String::from("working"),
                       String::from("pub_transport")],
        });

        let expected_config = Config {
            population,
            disease: Disease::new(5, 20, 40, 9, 12, 0.025, 0.25, 0.035, 0.3, 0.3, 48, 48),
            disease_overrides: vec![disease_override],
            geography_parameters: GeographyParameters::new(5660, 0.003),
            end_of_migration: 336,
            reduced_travel_percentage: 0.0005,
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
        let read_config = read(String::from("config/test/auto_pop.json")).unwrap();

        let vaccinate = VaccinateConfig::new(5000, 0.2);

        let population = Population::Auto(AutoPopulation {
            number_of_agents: 10000,
            public_transport_percentage: 0.2,
            working_percentage: 0.7,
        });

        let expected_config = Config {
            population,
            disease: Disease::new(5, 20, 40, 9, 12, 0.025, 0.25, 0.035, 0.3, 0.3, 48, 48),
            disease_overrides: vec![],
            geography_parameters: GeographyParameters::new(250, 0.003),
            end_of_migration: 336,
            reduced_travel_percentage: 0.0005,
            hours: 10000,
            interventions: vec![InterventionConfig::Vaccinate(vaccinate)],
            output_file: Some("simulation_default_config".to_string()),
            enable_citizen_state_messages: false,
            starting_infections: StartingInfections::new(2, 3, 4, 5),
        };

        assert_eq!(expected_config, read_config);
    }
}
