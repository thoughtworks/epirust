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

use crate::config::Config;
use crate::listeners::events::counts::Counts;
use crate::constants;

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub enum Intervention {
    Vaccinate(Vaccinate),
    Lockdown(LockdownConfig),
    BuildNewHospital(BuildNewHospitalConfig),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub struct Vaccinate {
    pub at_hour: i32,
    pub percent: f64,
}

impl Vaccinate {
    #[cfg(test)]
    pub fn new(at_hour: i32, percent: f64) -> Vaccinate {
        Vaccinate { at_hour, percent }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub struct LockdownConfig {
    pub at_number_of_infections: i32,
    pub essential_workers_population: f64,
    pub lock_down_period: i32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub struct BuildNewHospitalConfig {
    pub spread_rate_threshold: i32
}

impl Intervention {
    pub fn get_hospital_intervention(config: &Config) -> Option<BuildNewHospitalConfig> {
        return config.get_interventions().iter().filter_map(|i| {
            match i {
                Intervention::BuildNewHospital(x) => Some(x),
                _ => None
            }
        }).next().copied();
    }

    pub fn get_lock_down_intervention(config: &Config) -> Option<LockdownConfig> {
        return config.get_interventions().iter().filter_map(|i| {
            match i {
                Intervention::Lockdown(x) => Some(x),
                _ => None
            }
        }).next().copied();
    }
}

pub struct BuildNewHospital {
    new_infections_in_a_day: i32,
    intervention: Option<BuildNewHospitalConfig>,
}

impl BuildNewHospital {
    pub fn init(config: &Config) -> BuildNewHospital {
        let intervention = Intervention::get_hospital_intervention(config);
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

pub struct LockdownIntervention {
    is_locked_down: bool,
    locked_till_hr: i32,
    intervention: Option<LockdownConfig>,
}

impl LockdownIntervention {
    pub fn init(config: &Config) -> LockdownIntervention {
        LockdownIntervention {
            is_locked_down: false,
            locked_till_hr: 0,
            intervention: Intervention::get_lock_down_intervention(config),
        }
    }

    pub fn should_apply(&self, counts: &Counts) -> bool {
        !self.is_locked_down && match self.intervention {
            Some(i) => { counts.get_infected() > i.at_number_of_infections }
            None => false
        }
    }

    pub fn should_unlock(&self, counts: &Counts) -> bool {
        self.is_locked_down && self.locked_till_hr == counts.get_hour()
    }

    pub fn apply(&mut self, counts: &Counts) {
        match self.intervention {
            Some(i) => {
                self.is_locked_down = true;
                self.locked_till_hr = counts.get_hour() + i.lock_down_period * constants::NUMBER_OF_HOURS
            }
            None => { panic!("Tried to apply lockdown when intervention is not present"); }
        }
    }

    pub fn get_config(&self) -> &Option<LockdownConfig> {
        &self.intervention
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn should_apply_lockdown_at_threshold() {
        let config = LockdownConfig {
            at_number_of_infections: 20,
            essential_workers_population: 0.1,
            lock_down_period: 7,
        };
        let mut lockdown = LockdownIntervention {
            is_locked_down: false,
            locked_till_hr: 0,
            intervention: Some(config),
        };

        assert!(!lockdown.should_apply(&Counts::new_test(0, 99, 1, 0, 0, 0)));
        assert!(!lockdown.should_apply(&Counts::new_test(22, 80, 20, 0, 0, 0)));
        assert!(lockdown.should_apply(&Counts::new_test(28, 79, 21, 0, 0, 0)));

        lockdown.apply(&Counts::new_test(28, 79, 21, 0, 0, 0));
        assert_eq!(lockdown.locked_till_hr, 196);
        assert_eq!(lockdown.is_locked_down, true);
    }

    #[test]
    fn should_not_apply_lockdown_when_already_locked_down() {
        let config = LockdownConfig {
            at_number_of_infections: 20,
            essential_workers_population: 0.1,
            lock_down_period: 7,
        };
        let mut lockdown = LockdownIntervention {
            is_locked_down: false,
            locked_till_hr: 0,
            intervention: Some(config),
        };

        assert!(lockdown.should_apply(&Counts::new_test(28, 79, 21, 0, 0, 0)));
        lockdown.apply(&Counts::new_test(28, 79, 21, 0, 0, 0));
        assert!(!lockdown.should_apply(&Counts::new_test(29, 75, 25, 0, 0, 0)));
    }

    #[test]
    fn should_lift_lockdown_at_threshold() {
        let config = LockdownConfig {
            at_number_of_infections: 20,
            essential_workers_population: 0.1,
            lock_down_period: 7,
        };
        let mut lockdown = LockdownIntervention {
            is_locked_down: false,
            locked_till_hr: 0,
            intervention: Some(config),
        };
        assert!(lockdown.should_apply(&Counts::new_test(28, 79, 21, 0, 0, 0)));

        lockdown.apply(&Counts::new_test(28, 79, 21, 0, 0, 0));
        assert!(!lockdown.should_unlock(&Counts::new_test(42, 79, 21, 0, 0, 0)));
        assert!(lockdown.should_unlock(&Counts::new_test(196, 79, 21, 0, 0, 0)));
    }

    #[test]
    fn should_not_reapply_lockdown() {
        let config = LockdownConfig {
            at_number_of_infections: 20,
            essential_workers_population: 0.1,
            lock_down_period: 7,
        };
        let mut lockdown = LockdownIntervention {
            is_locked_down: false,
            locked_till_hr: 0,
            intervention: Some(config),
        };
        lockdown.apply(&Counts::new_test(28, 79, 21, 0, 0, 0));
        assert!(lockdown.should_unlock(&Counts::new_test(196, 79, 21, 0, 0, 0)));
        assert!(!lockdown.should_apply(&Counts::new_test(200, 70, 30, 0, 0, 0)));
    }
}
