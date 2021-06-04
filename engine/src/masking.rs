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
use crate::listeners::events::counts::Counts;

pub fn generate_mask_threshold(r: &mut RandomWrapper) -> f64 {
    //TODO: move this to a global location and use correct values for mean and std dev
    // we want min = 0.0 max = 1.0 so mean = 0.5 and stddev = 0.15
    // let MASK_THRESHOLD: Normal<f64> = Normal::new(0.001, 0.001).unwrap();
    let mask_threshold: Normal<f64> = Normal::new(0.2, 0.1).unwrap();
    let mut threshold = mask_threshold.sample(r.get());
    while !(threshold <= 1.0 && threshold >= 0.0) {
        threshold = mask_threshold.sample(r.get());
    }
    threshold
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MaskingBehavior {
    threshold_fraction: f64,
    efficiency_fraction: f64,
    is_mask_on: bool
}

impl MaskingBehavior {

    pub fn new(rng: &mut RandomWrapper) -> Self {
        Self {
            threshold_fraction: generate_mask_threshold(rng),
            efficiency_fraction: ((constants::MIN_MASK_EFFICIENCY..constants::MAX_MASK_EFFICIENCY+1).choose(rng.get()).unwrap_or(0) as f64)/100.0,
            is_mask_on: false
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

    pub fn update_mask_status(&mut self, current_counts: &Counts) {
        //TODO: this will depend on the strategy chosen
        if (current_counts.get_infected() as f64/current_counts.total() as f64) >= self.threshold_fraction {
            self.is_mask_on = true;
        }
        // else {
        //     self.is_mask_on = false
        // }
    }

    pub fn get_threshold(&self) -> f64 {
        self.threshold_fraction
    }

    pub fn is_wearing_mask(&self) -> bool {
        self.is_mask_on
    }
}