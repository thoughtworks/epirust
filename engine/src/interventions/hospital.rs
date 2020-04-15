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

use crate::listeners::events::counts::Counts;
use crate::interventions::Intervention;
use crate::interventions::intervention_type::InterventionType;
use crate::config::Config;

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub struct BuildNewHospitalConfig {
    pub spread_rate_threshold: i32
}

pub struct BuildNewHospital {
    new_infections_in_a_day: i32,
    intervention: Option<BuildNewHospitalConfig>,
}

impl BuildNewHospital {
    pub fn get_hospital_intervention(config: &Config) -> Option<BuildNewHospitalConfig> {
        return config.get_interventions().iter().filter_map(|i| {
            match i {
                Intervention::BuildNewHospital(x) => Some(x),
                _ => None
            }
        }).next().copied();
    }

    pub fn init(config: &Config) -> BuildNewHospital {
        let intervention = BuildNewHospital::get_hospital_intervention(config);
        BuildNewHospital {
            new_infections_in_a_day: 0,
            intervention,
        }
    }

    pub fn should_apply(&self, counts: &Counts) -> bool {
        let start_of_day = counts.get_hour() % 24 == 0;
        let exceeds_threshold = match self.intervention {
            None => { false }
            Some(i) => { self.new_infections_in_a_day >= i.spread_rate_threshold }
        };
        start_of_day && exceeds_threshold
    }

    pub fn counts_updated(&mut self, counts: &Counts) {
        if counts.get_hour() % 24 == 0 {
            self.new_infections_in_a_day = counts.get_infected() - self.new_infections_in_a_day;
        }
    }
}

impl InterventionType for BuildNewHospital {
    fn name(&mut self) -> String {
        return "build_new_hospital".to_string();
    }

    fn json_data(&mut self) -> String {
        return r#"{}"#.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_hospital_intervention() -> BuildNewHospital {
        let config = BuildNewHospitalConfig { spread_rate_threshold: 10 };
        return BuildNewHospital { new_infections_in_a_day: 0, intervention: Some(config) };
    }

    #[test]
    fn should_apply_hospital_intervention_when_threshold_increases_at_start_of_day() {
        let config = BuildNewHospitalConfig { spread_rate_threshold: 10 };
        let mut build_new_hospital = BuildNewHospital { new_infections_in_a_day: 0, intervention: Some(config) };
        let counts = Counts::new_test(0, 99, 1, 0, 0, 0);
        build_new_hospital.counts_updated(&counts);
        assert!(!build_new_hospital.should_apply(&counts));
        build_new_hospital.counts_updated(&Counts::new_test(24, 80, 20, 0, 0, 0));
        assert!(build_new_hospital.should_apply(&counts));
    }

    #[test]
    fn should_not_apply_hospital_intervention_when_absent() {
        let mut build_new_hospital = BuildNewHospital { new_infections_in_a_day: 0, intervention: None };
        let counts = Counts::new_test(0, 99, 1, 0, 0, 0);
        build_new_hospital.counts_updated(&counts);
        assert!(!build_new_hospital.should_apply(&counts));
        build_new_hospital.counts_updated(&Counts::new_test(24, 80, 20, 0, 0, 0));
        assert!(!build_new_hospital.should_apply(&counts));
    }

    #[test]
    fn should_not_apply_hospital_intervention_when_below_threshold() {
        let config = BuildNewHospitalConfig { spread_rate_threshold: 10 };
        let mut build_new_hospital = BuildNewHospital { new_infections_in_a_day: 0, intervention: Some(config) };
        let counts = Counts::new_test(0, 99, 1, 0, 0, 0);
        build_new_hospital.counts_updated(&counts);
        assert!(!build_new_hospital.should_apply(&counts));
        build_new_hospital.counts_updated(&Counts::new_test(24, 95, 5, 0, 0, 0));
        assert!(!build_new_hospital.should_apply(&counts));
    }

    #[test]
    fn should_return_intervention_name_as_build_new_hospital() {
        let mut build_new_hospital = get_test_hospital_intervention();

        assert_eq!(build_new_hospital.name(), "build_new_hospital")
    }

    #[test]
    fn should_return_json_data_as_empty() {
        let mut build_new_hospital = get_test_hospital_intervention();

        assert_eq!(build_new_hospital.json_data(), "{}")
    }
}
