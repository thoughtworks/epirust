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

use common::config::intervention_config::{InterventionConfig, LockdownConfig};
use common::config::Config;
use common::models::custom_types::Hour;

use crate::interventions::intervention_type::InterventionType;
use crate::models::constants;
use crate::models::events::Counts;

pub struct LockdownIntervention {
    is_locked_down: bool,
    intervention: Option<LockdownConfig>,
    pub zero_infection_hour: Hour,
}

impl LockdownIntervention {
    pub fn get_lock_down_intervention(config: &Config) -> Option<LockdownConfig> {
        return config
            .get_interventions()
            .iter()
            .filter_map(|i| match i {
                InterventionConfig::Lockdown(x) => Some(x),
                _ => None,
            })
            .next()
            .copied();
    }

    pub fn init(config: &Config) -> LockdownIntervention {
        LockdownIntervention {
            is_locked_down: false,
            intervention: LockdownIntervention::get_lock_down_intervention(config),
            zero_infection_hour: 0,
        }
    }

    pub fn should_apply(&self, counts: &Counts) -> bool {
        !self.is_locked_down && counts.get_hour() % constants::HOURS_IN_A_DAY == 0 && self.above_threshold(counts)
    }

    fn above_threshold(&self, counts: &Counts) -> bool {
        matches!(self.intervention, Some(i) if counts.get_infected() > i.at_number_of_infections)
    }

    pub fn set_zero_infection_hour(&mut self, zero_infection_hour: Hour) {
        if self.zero_infection_hour == 0 {
            self.zero_infection_hour = zero_infection_hour;
        }
    }

    pub fn should_unlock(&self, counts: &Counts) -> bool {
        let unlock_hour =
            self.zero_infection_hour + (constants::QUARANTINE_DAYS as f64 * 1.5).round() as Hour * constants::HOURS_IN_A_DAY;
        self.is_locked_down && counts.get_hour() == unlock_hour
    }

    pub fn apply(&mut self) {
        match self.intervention {
            Some(_i) => {
                self.is_locked_down = true;
            }
            None => {
                panic!("Tried to apply lockdown when intervention is not present");
            }
        }
    }

    pub fn unapply(&mut self) {
        self.is_locked_down = false;
        self.zero_infection_hour = 0;
    }

    pub fn get_essential_workers_percentage(&self) -> f64 {
        match self.intervention {
            Some(x) => x.essential_workers_population,
            _ => 0.0,
        }
    }

    pub fn is_locked_down(&self) -> bool {
        self.is_locked_down
    }
}

impl InterventionType for LockdownIntervention {
    fn name(&self) -> String {
        "lockdown".to_string()
    }

    fn json_data(&self) -> String {
        if self.is_locked_down {
            r#"{"status": "locked_down"}"#.to_string()
        } else {
            r#"{"status": "lockdown_revoked"}"#.to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_lockdown_intervention(is_locked_down: bool) -> LockdownIntervention {
        let config = LockdownConfig { at_number_of_infections: 20, essential_workers_population: 0.1 };
        LockdownIntervention { is_locked_down, intervention: Some(config), zero_infection_hour: 0 }
    }

    #[test]
    fn should_apply_lockdown_at_threshold() {
        let config = LockdownConfig { at_number_of_infections: 20, essential_workers_population: 0.1 };
        let mut lockdown = LockdownIntervention { is_locked_down: false, intervention: Some(config), zero_infection_hour: 0 };

        assert!(!lockdown.should_apply(&Counts::new_test(0, 99, 0, 1, 0, 0, 0)));
        assert!(!lockdown.should_apply(&Counts::new_test(22, 80, 0, 20, 0, 0, 0)));
        assert!(!lockdown.should_apply(&Counts::new_test(28, 79, 0, 21, 0, 0, 0)));
        assert!(lockdown.should_apply(&Counts::new_test(48, 79, 0, 21, 0, 0, 0)));

        lockdown.apply();
        assert!(lockdown.is_locked_down);
    }

    #[test]
    fn should_not_apply_lockdown_when_already_locked_down() {
        let config = LockdownConfig { at_number_of_infections: 20, essential_workers_population: 0.1 };
        let mut lockdown = LockdownIntervention { is_locked_down: false, intervention: Some(config), zero_infection_hour: 0 };

        assert!(lockdown.should_apply(&Counts::new_test(48, 79, 0, 21, 0, 0, 0)));
        lockdown.apply();
        assert!(!lockdown.should_apply(&Counts::new_test(48, 75, 0, 25, 0, 0, 0)));
    }

    #[test]
    fn should_lift_lockdown_at_after_time_elapsed_and_infections_below_threshold() {
        let config = LockdownConfig { at_number_of_infections: 20, essential_workers_population: 0.1 };
        let mut lockdown = LockdownIntervention { is_locked_down: false, intervention: Some(config), zero_infection_hour: 0 };
        assert!(lockdown.should_apply(&Counts::new_test(48, 79, 0, 21, 0, 0, 0)));

        lockdown.apply();
        let lockdown_until = 48 + (7 * 24);
        lockdown.set_zero_infection_hour(lockdown_until);
        for hr in 48..lockdown_until {
            assert!(!lockdown.should_unlock(&Counts::new_test(hr, 80, 0, 20, 0, 0, 0)));
        }
        let remove_lockdown = lockdown_until + 21 * 24;
        assert!(lockdown.should_unlock(&Counts::new_test(remove_lockdown, 80, 0, 20, 0, 0, 0)));
    }

    #[test]
    fn should_extend_lockdown_until_infections_below_threshold() {
        let config = LockdownConfig { at_number_of_infections: 20, essential_workers_population: 0.1 };
        let mut lockdown = LockdownIntervention { is_locked_down: false, intervention: Some(config), zero_infection_hour: 0 };
        assert!(lockdown.should_apply(&Counts::new_test(48, 79, 0, 21, 0, 0, 0)));

        lockdown.apply();
        let lockdown_until = 48 + (7 * 24);
        lockdown.set_zero_infection_hour(lockdown_until);
        for hr in 48..lockdown_until {
            assert!(!lockdown.should_unlock(&Counts::new_test(hr, 79, 0, 21, 0, 0, 0)));
        }
        assert!(!lockdown.should_unlock(&Counts::new_test(lockdown_until, 79, 0, 21, 0, 0, 0)));
        assert!(!lockdown.should_unlock(&Counts::new_test(lockdown_until + 1, 79, 0, 20, 0, 0, 0)));
        assert!(lockdown.should_unlock(&Counts::new_test(lockdown_until + 504, 79, 0, 20, 0, 0, 0)));
    }

    #[test]
    fn should_not_reapply_lockdown() {
        let config = LockdownConfig { at_number_of_infections: 20, essential_workers_population: 0.1 };
        let mut lockdown = LockdownIntervention { is_locked_down: false, intervention: Some(config), zero_infection_hour: 0 };
        lockdown.apply();
        lockdown.set_zero_infection_hour(28);
        assert!(lockdown.should_unlock(&Counts::new_test(532, 80, 0, 20, 0, 0, 0)));
        assert!(!lockdown.should_apply(&Counts::new_test(540, 70, 0, 30, 0, 0, 0)));
    }

    #[test]
    fn should_return_intervention_name_as_lockdown() {
        let lockdown_intervention = get_test_lockdown_intervention(false);

        assert_eq!(lockdown_intervention.name(), "lockdown")
    }

    #[test]
    fn should_return_json_data_with_lockdown_state_as_locked_down_when_city_is_locked_down() {
        let lockdown_intervention = get_test_lockdown_intervention(true);

        assert_eq!(lockdown_intervention.json_data(), r#"{"status": "locked_down"}"#)
    }

    #[test]
    fn should_return_json_data_with_lockdown_state_as_lockdown_revoked_when_city_is_not_locked_down() {
        let lockdown_intervention = get_test_lockdown_intervention(false);

        assert_eq!(lockdown_intervention.json_data(), r#"{"status": "lockdown_revoked"}"#)
    }

    #[test]
    fn unapply_should_set_is_locked_down_to_false() {
        let mut intervention = get_test_lockdown_intervention(true);

        intervention.unapply();

        assert!(!intervention.is_locked_down);
    }
}
