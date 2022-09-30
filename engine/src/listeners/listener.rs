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

use common::models::custom_types::Hour;
use std::any::Any;

use crate::citizen::Citizen;
use crate::geography::{Grid, Point};
use crate::interventions::intervention_type::InterventionType;
use crate::models::events::Counts;
use crate::travel::migration::MigratorsByRegion;

pub trait Listener {
    fn counts_updated(&mut self, _counts: Counts) {}
    fn simulation_ended(&mut self) {}
    fn citizen_got_infected(&mut self, _cell: &Point) {}
    fn citizen_state_updated(&mut self, _hr: Hour, _citizen: &Citizen, _location: &Point) {}
    fn grid_updated(&self, _grid: &Grid) {}
    fn intervention_applied(&mut self, _at_hour: Hour, _intervention: &dyn InterventionType) {}
    fn outgoing_migrators_added(&mut self, _hr: Hour, _travellers: &[MigratorsByRegion]) {}
    fn as_any(&self) -> &dyn Any;
}

pub struct Listeners {
    listeners: Vec<Box<dyn Listener>>,
}

impl Listeners {
    pub fn from(listeners: Vec<Box<dyn Listener>>) -> Listeners {
        Listeners { listeners }
    }
}

impl Listeners {
    pub fn counts_updated(&mut self, counts: Counts) {
        self.listeners.iter_mut().for_each(|listener| listener.counts_updated(counts));
    }

    pub fn simulation_ended(&mut self) {
        self.listeners.iter_mut().for_each(|listener| listener.simulation_ended());
    }

    pub fn citizen_got_infected(&mut self, cell: &Point) {
        self.listeners.iter_mut().for_each(|listener| listener.citizen_got_infected(cell));
    }

    pub fn citizen_state_updated(&mut self, hr: Hour, citizen: &Citizen, location: &Point) {
        self.listeners.iter_mut().for_each(|listener| {
            listener.citizen_state_updated(hr, citizen, location);
        })
    }

    pub fn grid_updated(&self, grid: &Grid) {
        self.listeners.iter().for_each(|l| l.grid_updated(grid))
    }

    pub fn intervention_applied(&mut self, _at_hour: Hour, _intervention: &dyn InterventionType) {
        self.listeners.iter_mut().for_each(|l| l.intervention_applied(_at_hour, _intervention))
    }

    pub fn outgoing_migrators_added(&mut self, hr: Hour, travellers: &[MigratorsByRegion]) {
        self.listeners.iter_mut().for_each(|l| l.outgoing_migrators_added(hr, travellers));
    }
}

#[cfg(test)]
mod tests {
    use std::any::Any;

    use crate::geography::Point;
    use crate::listeners::listener::{Listener, Listeners};
    use crate::models::events::Counts;

    struct MockListener {
        calls_counts_updated: u32,
        calls_simulation_ended: u32,
        calls_citizen_got_infected: u32,
    }

    impl MockListener {
        fn new() -> MockListener {
            MockListener { calls_counts_updated: 0, calls_simulation_ended: 0, calls_citizen_got_infected: 0 }
        }
    }

    impl Listener for MockListener {
        fn counts_updated(&mut self, _counts: Counts) {
            self.calls_counts_updated += 1;
        }

        fn simulation_ended(&mut self) {
            self.calls_simulation_ended += 1;
        }

        fn citizen_got_infected(&mut self, _cell: &Point) {
            self.calls_citizen_got_infected += 1;
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    #[test]
    fn should_notify_all_listeners() {
        let mock1 = Box::new(MockListener::new());
        let mock2 = Box::new(MockListener::new());

        let mocks: Vec<Box<dyn Listener>> = vec![mock1, mock2];
        let mut listeners = Listeners::from(mocks);

        listeners.counts_updated(Counts::new(10, 1, 0));
        listeners.citizen_got_infected(&Point::new(1, 1));
        listeners.simulation_ended();

        for i in 0..=1 {
            //ownership has moved. We need to read the value from the struct, and downcast to MockListener to assert
            let mock = listeners.listeners.get(i).unwrap().as_any().downcast_ref::<MockListener>().unwrap();
            assert_eq!(mock.calls_counts_updated, 1);
            assert_eq!(mock.calls_citizen_got_infected, 1);
            assert_eq!(mock.calls_simulation_ended, 1);
        }
    }
}
