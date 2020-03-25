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

use std::collections::HashMap;
use std::fs::File;

use rand::Rng;
use serde::Deserialize;

use crate::random_wrapper::RandomWrapper;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy)]
pub struct Disease {
    regular_transmission_start_day: i32,
    high_transmission_start_day: i32,
    last_day: i32,
    regular_transmission_rate: f64,
    high_transmission_rate: f64,
    death_rate: f64,
}

impl Disease {
    pub fn init(config_file_path: &str, disease_name: &String) -> Disease {
        let reader = File::open(config_file_path).expect("Failed to open disease config file");
        let yaml: HashMap<String, Disease> = serde_yaml::from_reader(reader).expect("Failed to parse disease config file");
        let disease = yaml.get(disease_name).expect("Failed to find disease");
        *disease
    }

    pub fn new(regular_transmission_start_day: i32, high_transmission_start_day: i32, last_day: i32,
               regular_transmission_rate: f64, high_transmission_rate: f64, death_rate: f64) -> Disease {
        Disease {
            regular_transmission_start_day,
            high_transmission_start_day,
            last_day,
            regular_transmission_rate,
            high_transmission_rate,
            death_rate,
        }
    }

    pub fn get_current_transmission_rate(&self, infection_day: i32) -> f64 {
        if self.regular_transmission_start_day < infection_day && infection_day <= self.high_transmission_start_day {
            return self.regular_transmission_rate;
        } else if self.high_transmission_start_day < infection_day && infection_day <= self.last_day {
            return self.high_transmission_rate;
        }
        0.0
    }

    pub fn to_be_quarantined(&self, infection_day: i32) -> bool {
        let transmission_rate = self.get_current_transmission_rate(infection_day);
        if transmission_rate >= self.high_transmission_rate {
            return true;
        }
        false
    }

    pub fn get_disease_last_day(&self) -> i32 {
        self.last_day
    }

    pub fn to_be_deceased(&self, rng: &mut RandomWrapper) -> bool {
        if rng.get().gen_bool(self.death_rate) {
            return true;
        }
        false
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
    pub fn new(population_param: String, values: Vec<String>, disease: Disease) -> DiseaseOverride {
        DiseaseOverride {
            population_param, values, disease
        }
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
        let actual = disease.to_be_quarantined(12);
        assert_eq!(actual, false);

        let actual = disease.to_be_quarantined(22);
        assert_eq!(actual, true);
    }

    #[test]
    fn read_smallpox_config() {
        let disease = Disease::init("config/diseases.yaml", &String::from("small_pox"));
        let expected = Disease {
            regular_transmission_start_day: 10,
            high_transmission_start_day: 16,
            last_day: 22,
            regular_transmission_rate: 0.05,
            high_transmission_rate: 0.5,
            death_rate: 0.2,
        };
        assert_eq!(expected, disease)
    }
}
