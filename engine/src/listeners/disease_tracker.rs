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

use fnv::FnvHashMap;
use std::any::Any;

use crate::geography::Point;
use crate::listeners::listener::Listener;
use crate::models::events::Counts;

pub struct Hotspot {
    disease_hotspot_tracker: FnvHashMap<Point, i32>,
}

impl Hotspot {
    pub fn new() -> Hotspot {
        let disease_hotspot_tracker = FnvHashMap::default();
        Hotspot { disease_hotspot_tracker }
    }
}

impl Listener for Hotspot {
    fn counts_updated(&mut self, _counts: Counts) {}

    fn simulation_ended(&mut self) {}

    fn citizen_got_infected(&mut self, cell: &Point) {
        let counter = self.disease_hotspot_tracker.entry(*cell).or_insert(0);
        *counter += 1;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::geography::Point;
    use crate::listeners::disease_tracker::Hotspot;
    use crate::listeners::listener::Listener;

    #[test]
    fn should_initialize() {
        let tracker = Hotspot::new();
        assert_eq!(tracker.disease_hotspot_tracker.len(), 0);
    }

    #[test]
    fn should_add_new_entry() {
        let mut tracker = Hotspot::new();
        let current_point = Point::new(0, 1);

        tracker.citizen_got_infected(&current_point);

        assert_eq!(*tracker.disease_hotspot_tracker.get(&current_point).unwrap(), 1);
    }

    #[test]
    fn should_update_tracker() {
        let mut tracker = Hotspot::new();
        let current_point = Point::new(0, 1);

        tracker.citizen_got_infected(&current_point);
        tracker.citizen_got_infected(&current_point);

        assert_eq!(*tracker.disease_hotspot_tracker.get(&current_point).unwrap(), 2);
    }
}
