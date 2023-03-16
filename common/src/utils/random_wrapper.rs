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

use rand::rngs::StdRng;
use rand::seq::IteratorRandom;
use rand::{Rng, SeedableRng};

pub trait Random {
    fn gen_bool(&mut self, probability: f64) -> bool;
    fn choose<I>(&mut self, from: I) -> Option<I::Item>
    where
        I: Iterator + Sized;
    fn choose_multiple<I>(&mut self, from: I, amount: usize) -> Vec<I::Item>
    where
        I: Iterator + Sized;
}

#[derive(Clone)]
pub struct RandomWrapper {
    rng: StdRng,
}

impl Random for RandomWrapper {
    fn gen_bool(&mut self, probability: f64) -> bool {
        self.rng.gen_bool(probability)
    }

    fn choose<I>(&mut self, from: I) -> Option<I::Item>
    where
        I: Iterator + Sized,
    {
        from.choose(&mut self.rng)
    }

    fn choose_multiple<I>(&mut self, from: I, amount: usize) -> Vec<I::Item>
    where
        I: Iterator + Sized,
    {
        from.choose_multiple(&mut self.rng, amount)
    }
}

impl Default for RandomWrapper {
    fn default() -> Self {
        RandomWrapper { rng: SeedableRng::from_entropy() }
    }
}
