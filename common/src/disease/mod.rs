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

use rand::Rng;
use validator::Validate;

use crate::models::custom_types::{validate_percentage, Day, Hour, Percentage};
use crate::utils::RandomWrapper;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy, Validate)]
pub struct Disease {
    regular_transmission_start_day: Day,
    high_transmission_start_day: Day,
    last_day: Day,
    asymptomatic_last_day: Day,
    mild_infected_last_day: Day,
    #[validate(custom = "validate_percentage")]
    regular_transmission_rate: Percentage,
    #[validate(custom = "validate_percentage")]
    high_transmission_rate: Percentage,
    #[validate(custom = "validate_percentage")]
    death_rate: Percentage,
    #[validate(custom = "validate_percentage")]
    percentage_asymptomatic_population: Percentage,
    #[validate(custom = "validate_percentage")]
    percentage_severe_infected_population: Percentage,
    exposed_duration: Hour,
    pre_symptomatic_duration: Hour,
}

impl Disease {
    pub fn init(config_file_path: &str, disease_name: &String) -> Disease {
        use std::collections::HashMap;
        use std::fs::File;

        let reader = File::open(config_file_path).expect("Failed to open disease config file");
        let yaml: HashMap<String, Disease> = serde_yaml::from_reader(reader).expect("Failed to parse disease config file");
        let disease = yaml.get(disease_name).expect("Failed to find disease");
        *disease
    }

    pub fn new(
        regular_transmission_start_day: Day,
        high_transmission_start_day: Day,
        last_day: Day,
        asymptomatic_last_day: Day,
        mild_infected_last_day: Day,
        regular_transmission_rate: Percentage,
        high_transmission_rate: Percentage,
        death_rate: Percentage,
        percentage_asymptomatic_population: Percentage,
        percentage_severe_infected_population: Percentage,
        exposed_duration: Hour,
        pre_symptomatic_duration: Hour,
    ) -> Disease {
        Disease {
            regular_transmission_start_day,
            high_transmission_start_day,
            last_day,
            asymptomatic_last_day,
            mild_infected_last_day,
            regular_transmission_rate,
            high_transmission_rate,
            death_rate,
            percentage_asymptomatic_population,
            percentage_severe_infected_population,
            exposed_duration,
            pre_symptomatic_duration,
        }
    }

    pub fn get_current_transmission_rate(&self, infection_day: Day) -> Percentage {
        if self.regular_transmission_start_day < infection_day && infection_day <= self.high_transmission_start_day {
            return self.regular_transmission_rate;
        } else if self.high_transmission_start_day < infection_day && infection_day <= self.last_day {
            return self.high_transmission_rate;
        }
        0.0
    }

    pub fn is_to_be_hospitalized(&self, infection_day: Day) -> bool {
        self.get_current_transmission_rate(infection_day) >= self.high_transmission_rate
    }

    pub fn get_last_day(&self) -> Day {
        self.last_day
    }

    pub fn is_to_be_deceased(&self, rng: &mut RandomWrapper) -> bool {
        rng.get().gen_bool(self.death_rate)
    }

    pub fn get_percentage_asymptomatic_population(&self) -> Percentage {
        self.percentage_asymptomatic_population
    }

    pub fn get_percentage_severe_infected_population(&self) -> Percentage {
        self.percentage_severe_infected_population
    }

    pub fn get_exposed_duration(&self) -> Hour {
        self.exposed_duration
    }

    pub fn get_pre_symptomatic_duration(&self) -> Hour {
        self.pre_symptomatic_duration
    }
}

/// Override disease parameters for a specific population trait
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct DiseaseOverride {
    population_param: String,
    values: Vec<String>,
    disease: Disease,
}

impl DiseaseOverride {
    #[cfg(test)]
    pub fn new(population_param: String, values: Vec<String>, disease: Disease) -> DiseaseOverride {
        DiseaseOverride { population_param, values, disease }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_current_transmission_rate() {
        let disease = Disease::init("config/diseases.yaml", &String::from("small_pox"));
        let infection_rate = disease.get_current_transmission_rate(12);
        assert_eq!(infection_rate, 0.05);

        let infection_rate = disease.get_current_transmission_rate(22);
        assert_eq!(infection_rate, 0.5);
    }

    #[test]
    fn to_be_quarantined() {
        let disease = Disease::init("config/diseases.yaml", &String::from("small_pox"));
        let actual = disease.is_to_be_hospitalized(12);
        assert!(!actual);

        let actual = disease.is_to_be_hospitalized(22);
        assert!(actual);
    }

    #[test]
    fn read_smallpox_config() {
        let disease = Disease::init("config/diseases.yaml", &String::from("small_pox"));
        let expected = Disease {
            regular_transmission_start_day: 10,
            high_transmission_start_day: 16,
            last_day: 22,
            asymptomatic_last_day: 22,
            mild_infected_last_day: 22,
            regular_transmission_rate: 0.05,
            high_transmission_rate: 0.5,
            death_rate: 0.2,
            percentage_asymptomatic_population: 0.0,
            percentage_severe_infected_population: 1.0,
            exposed_duration: 0,
            pre_symptomatic_duration: 0,
        };
        assert_eq!(expected, disease)
    }
}
