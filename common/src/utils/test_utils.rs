/*
 * EpiRust
 * Copyright (c) 2023  ThoughtWorks, Inc.
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
#![cfg(test)]

use crate::models::custom_types::Hour;
use crate::utils::Random;
use rand::distributions::uniform::SampleUniform;
use rand::distributions::{Alphanumeric, DistIter, Distribution, Standard};
use rand::prelude::ThreadRng;
use rand::Rng;
use std::panic;
use std::panic::UnwindSafe;

pub struct MockRandomWrapper<P> {
    bool: P,
    random_n: usize,
    random_ns: Vec<usize>,
}

impl<P> MockRandomWrapper<P>
where
    P: FnMut(f64) -> bool,
{
    pub fn new(bool: P, random_n: usize, random_ns: Vec<usize>) -> Self {
        MockRandomWrapper { bool, random_n, random_ns }
    }
}

impl<P> Random for MockRandomWrapper<P>
where
    P: FnMut(f64) -> bool,
{
    fn gen_bool(&mut self, probability: f64) -> bool {
        (self.bool)(probability)
    }

    fn choose<I>(&mut self, mut from: I) -> Option<I::Item>
    where
        I: Iterator + Sized,
    {
        from.nth(self.random_n)
    }

    fn choose_multiple<I>(&mut self, from: I, amount: usize) -> Vec<I::Item>
    where
        I: Iterator + Sized,
    {
        if amount != self.random_ns.len() {
            panic!("Not found a Impl for size of {}", amount);
        }
        from.enumerate().filter_map(|(i, x)| if self.random_ns.contains(&i) { Some(x) } else { None }).collect()
    }
}

pub fn random_number<T>(min: T, max: T) -> T
where
    T: PartialOrd<T> + SampleUniform,
    Standard: Distribution<T>,
{
    rand::thread_rng().gen_range(min..=max)
}

pub fn random_bool() -> bool {
    rand::thread_rng().gen_bool(0.5)
}

pub fn random_hour() -> Hour {
    rand::thread_rng().gen_range(1..=24)
}

pub fn random_string(len: usize) -> String {
    let iter: DistIter<Alphanumeric, ThreadRng, u8> = rand::thread_rng().sample_iter(Alphanumeric);
    iter.take(len).map(char::from).collect()
}

pub fn random_int_vec(len: usize) -> Vec<u32> {
    rand::thread_rng().sample_iter(Standard).take(len).collect()
}

pub fn panic_into_result<F: FnMut() -> R + UnwindSafe, R>(f: F) -> Result<R, String> {
    let result = panic::catch_unwind(f);
    result.map_err(|err| {
        let mut message_str = String::new();
        if let Some(string) = err.downcast_ref::<String>() {
            message_str = string.to_string()
        } else if let Some(string) = err.downcast_ref::<&str>() {
            message_str.push_str(string)
        };
        message_str
    })
}
