/*
 * EpiRust
 * Copyright (c) 2021  ThoughtWorks, Inc.
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

use crate::constants;
use crate::random_wrapper::RandomWrapper;
use rand::seq::IteratorRandom;
use rand_distr::{Normal, Distribution};
use crate::listeners::events::counts::{Counts, CumulativeAverage};

pub fn sample_from_normal_distr(r: &mut RandomWrapper, mean: f64, std_dev: f64) -> f64 {
    let mask_threshold: Normal<f64> = Normal::new(mean, std_dev).unwrap();
    let mut threshold = mask_threshold.sample(r.get());
    while !(threshold <= 1.0 && threshold >= 0.0) {
        threshold = mask_threshold.sample(r.get());
    }
    threshold
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MaskingBehavior {
    total_infections_threshold: f64,
    efficiency_fraction: f64,
    is_mask_on: bool,
    moving_avg_range: i32,
    daily_new_infections_threshold: f64
}

impl MaskingBehavior {

    pub fn new(rng: &mut RandomWrapper) -> Self {
        Self {
            //TODO: move this to a global location and use correct values for mean and std dev
            total_infections_threshold: sample_from_normal_distr(rng, 0.2, 0.1),
            efficiency_fraction: ((constants::MIN_MASK_EFFICIENCY..constants::MAX_MASK_EFFICIENCY+1).choose(rng.get()).unwrap_or(0) as f64)/100.0,
            is_mask_on: false,
            moving_avg_range: (1..10).choose(rng.get()).unwrap_or(1),
            // moving_avg_range: 7,
            daily_new_infections_threshold: sample_from_normal_distr(rng, 0.005, 0.01), //TODO
            // daily_new_infections_threshold : 0.01,
        }
    }

    pub fn reduction_in_exposure_rate(&self) -> f64 {
        if self.is_mask_on {
            return self.efficiency_fraction
        }
        0.0
    }

    pub fn reduction_in_transmission_rate(&self) -> f64 {
        if self.is_mask_on {
            return self.efficiency_fraction
        }
        0.0
    }

    pub fn update_mask_status(&mut self, current_counts: &Counts, cumulative_counts: &CumulativeAverage, current_day: i32) {

        // strategy 1
        // if (current_counts.get_infected() as f64/current_counts.total() as f64) >= self.total_infections_threshold {
        //     self.is_mask_on = true;
        // }
        // else {
        //     self.is_mask_on = false
        // }

        // strategy 2
        if current_day <= self.moving_avg_range {
            return
        }
        let moving_avg = cumulative_counts.get_infected_moving_average(current_day - self.moving_avg_range, current_day);
        if moving_avg/(current_counts.total() as f64) >= self.daily_new_infections_threshold {
            self.is_mask_on = true;
        }
        else {
            self.is_mask_on = false
        }
    }

    pub fn is_wearing_mask(&self) -> bool {
        self.is_mask_on
    }
}