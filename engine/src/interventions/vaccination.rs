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

use crate::config::Config;
use crate::interventions::Intervention;
use crate::listeners::events::counts::Counts;
use crate::interventions::intervention_type::InterventionType;

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub struct VaccinateConfig {
    pub at_hour: i32,
    pub percent: f64,
}

impl VaccinateConfig {
    #[cfg(test)]
    pub fn new(at_hour: i32, percent: f64) -> VaccinateConfig {
        VaccinateConfig { at_hour, percent }
    }
}

pub struct VaccinateIntervention {
    intervention: HashMap<i32, f64>,
}

impl VaccinateIntervention {
    pub fn init(config: &Config) -> VaccinateIntervention {
        VaccinateIntervention {
            intervention: VaccinateIntervention::prepare_vaccinations(config)
        }
    }

    fn prepare_vaccinations(config: &Config) -> HashMap<i32, f64> {
        let mut vaccinations: HashMap<i32, f64> = HashMap::new();
        config.get_interventions().iter().filter_map(|i| {
            match i {
                Intervention::Vaccinate(v) => Some(v),
                _ => None,
            }
        }).for_each(|v| {
            vaccinations.insert(v.at_hour, v.percent);
        });
        vaccinations
    }

    pub fn get_vaccination_percentage(&self, counts: &Counts) -> Option<&f64> {
        self.intervention.get(&counts.get_hour())
    }
}

impl InterventionType for VaccinateIntervention {
    fn name(&mut self) -> String {
        "vaccination".to_string()
    }

    fn json_data(&mut self) -> String {
        "{}".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::config;

    use super::*;

    fn get_test_vaccination_intervention() -> VaccinateIntervention {
        let config = config::read("config/test/auto_pop.json".to_string()).unwrap();
        return VaccinateIntervention::init(&config);
    }

    #[test]
    fn should_parse_vaccinations_from_config() {
        let config = config::read("config/test/auto_pop.json".to_string()).unwrap();
        let vaccinate_intervention = VaccinateIntervention::init(&config);

        let mut expected: HashMap<i32, f64> = HashMap::new();
        expected.insert(5000, 0.2);

        assert_eq!(expected, vaccinate_intervention.intervention);
    }

    #[test]
    fn should_get_vaccination_at_hour() {
        let config = config::read("config/test/auto_pop.json".to_string()).unwrap();
        let vaccinate_intervention = VaccinateIntervention::init(&config);

        let counts = Counts::new_test(5000, 10, 10, 10, 10, 10);
        assert_eq!(Some(&0.2), vaccinate_intervention.get_vaccination_percentage(&counts));

        let counts = Counts::new_test(5001, 10, 10, 10, 10, 10);
        assert_eq!(None, vaccinate_intervention.get_vaccination_percentage(&counts));
    }

    #[test]
    fn should_return_intervention_name_as_vaccination() {
        let mut vaccination_intervention = get_test_vaccination_intervention();

        assert_eq!(vaccination_intervention.name(), "vaccination")
    }

    #[test]
    fn should_return_empty_json_data() {
        let mut vaccination_intervention = get_test_vaccination_intervention();

        assert_eq!(vaccination_intervention.json_data(), "{}")
    }
}
