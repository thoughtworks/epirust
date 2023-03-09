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

use std::collections::hash_map::{Iter, IterMut};

use common::config::{Config, TravelPlanConfig};
use common::models::custom_types::{CoOrdinate, Count, Hour};
use common::utils::RandomWrapper;
use fnv::FnvHashMap;

use rand::seq::IteratorRandom;
use rand::Rng;
use rayon::prelude::*;

use crate::citizen::Citizen;

use crate::geography::Point;
use crate::geography::{Area, Grid};
use crate::interventions::vaccination::VaccinateIntervention;
use crate::interventions::Interventions;
use crate::listeners::listener::Listeners;
use crate::models::constants;
use crate::models::events::Counts;
use crate::state_machine::{DiseaseHandler, State};
use crate::travel::commute::Commuter;
use crate::travel::migration::Migrator;

#[derive(Clone)]
pub struct CitizenLocationMap {
    pub grid: Grid,
    current_locations: FnvHashMap<Point, Citizen>,
    upcoming_locations: FnvHashMap<Point, Citizen>,
}

impl CitizenLocationMap {
    pub fn new(grid: Grid, agent_list: &[Citizen], points: &[Point]) -> Self {
        debug!("{} agents and {} starting points", agent_list.len(), points.len());
        let mut map: FnvHashMap<Point, Citizen> = FnvHashMap::with_capacity_and_hasher(agent_list.len(), Default::default());
        agent_list.iter().enumerate().for_each(|(i, _)| {
            map.insert(points[i], agent_list[i].clone());
        });

        let capacity = grid.grid_size as usize;
        CitizenLocationMap {
            grid,
            current_locations: map,
            upcoming_locations: FnvHashMap::with_capacity_and_hasher(capacity, Default::default()),
        }
    }

    pub fn simulate<T: DiseaseHandler + Sync>(
        &mut self,
        csv_record: &mut Counts,
        simulation_hour: Hour,
        listeners: &mut Listeners,
        rng: &mut RandomWrapper,
        percent_outgoing: f64,
        outgoing_migrators: &mut Vec<(Point, Migrator)>,
        outgoing_commuters: &mut Vec<(Point, Commuter)>,
        publish_citizen_state: bool,
        travel_plan_config: Option<&TravelPlanConfig>,
        region_name: &String,
        disease_handler: &T,
    ) {
        csv_record.clear();
        let updates: Vec<((Point, Point), Citizen, bool)> = self.par_iter().map(|(cell, agent)| {
            let mut rng_thread = RandomWrapper::new();
            let mut current_agent = *agent;
            let infection_status = current_agent.state_machine.is_infected();
            let point = current_agent.perform_operation(*cell, simulation_hour, &self.grid, self, &mut rng_thread, disease_handler);
            ((*cell, point), current_agent, infection_status)
        }).collect();
        updates.iter().for_each(|pair| {
            let old_cell = pair.0.0;
            let new_cell = pair.0.1;
            let agent = pair.1;
            let mut new_location = &new_cell;
            let agent_at_new_cell = *self.upcoming_locations.entry(new_cell).or_insert(agent);
            if agent_at_new_cell.id != agent.id {
                self.upcoming_locations.insert(old_cell, agent);
                new_location = &old_cell;
            }
            Counts::update_counts(csv_record, &agent);
            if let Some(travel_plan) = travel_plan_config {
                let is_migration_enabled = travel_plan.migration.enabled;
                let is_commute_enabled = travel_plan.commute.enabled;

                if is_migration_enabled
                    && agent.can_migrate(region_name, simulation_hour, travel_plan)
                    && rng.get().gen_bool(percent_outgoing)
                {
                    let migrator = Migrator::from(&agent);
                    outgoing_migrators.push((*new_location, migrator));
                }

                if is_commute_enabled && agent.is_commuter(region_name, simulation_hour) {
                    let commuter = Commuter::from(&agent);
                    outgoing_commuters.push((*new_location, commuter));
                }
            }

            if publish_citizen_state {
                listeners.citizen_state_updated(simulation_hour, &agent, new_location);
            }
        });
        debug!("done with iter upcoming");
        self.swap();
        assert_eq!(csv_record.total(), self.current_population());
    }

    fn swap(&mut self) {
        self.current_locations.clear();
        std::mem::swap(&mut self.current_locations, &mut self.upcoming_locations);
    }

    pub fn move_agent(&self, old_cell: Point, new_cell: Point) -> Point {
        if self.is_cell_vacant(&new_cell) {
            new_cell
        } else {
            old_cell
        }
    }

    pub fn goto_hospital(&self, hospital_area: &Area, cell: Point, citizen: &mut Citizen) -> (bool, Point) {
        let vacant_hospital_cell = hospital_area.iter().find(|cell| self.is_cell_vacant(cell));
        match vacant_hospital_cell {
            Some(x) => (true, self.move_agent(cell, x)),
            None => (false, self.move_agent(cell, citizen.home_location.get_random_point(&mut RandomWrapper::new()))),
        }
    }

    pub fn get_agent_for(&self, cell: &Point) -> Option<&Citizen> {
        self.current_locations.get(cell)
    }

    pub fn is_point_in_grid(&self, point: &Point) -> bool {
        let end_coordinate_of_grid = self.grid.grid_size as CoOrdinate;
        point.x >= 0 && point.y >= 0 && point.x < end_coordinate_of_grid && point.y < end_coordinate_of_grid
    }

    pub fn is_cell_vacant(&self, cell: &Point) -> bool {
        !self.current_locations.contains_key(cell)
    }

    pub fn remove_migrators(&mut self, outgoing: &Vec<(Point, Migrator)>, counts: &mut Counts) {
        if outgoing.is_empty() {
            return;
        }
        debug!("Removing {} outgoing travellers", outgoing.len());
        for (point, migrator) in outgoing {
            CitizenLocationMap::decrement_counts(&migrator.state_machine.state, counts);
            match self.current_locations.remove(point) {
                None => {
                    panic!(
                        "Trying to remove citizen {:?} from location {:?}, but no citizen is present at this location!",
                        migrator.id, point
                    )
                }
                Some(citizen) => {
                    self.grid.remove_house_occupant(&citizen.home_location);
                    if citizen.is_working() {
                        self.grid.remove_office_occupant(&citizen.work_location);
                    }
                }
            }
        }
    }

    pub fn remove_commuters(&mut self, outgoing: &Vec<(Point, Commuter)>, counts: &mut Counts) {
        if outgoing.is_empty() {
            return;
        }
        debug!("Removing {} outgoing commuters", outgoing.len());
        for (point, commuter) in outgoing {
            CitizenLocationMap::decrement_counts(&commuter.state_machine.state, counts);
            match self.current_locations.remove(point) {
                None => {
                    panic!(
                        "Trying to remove citizen {:?} from location {:?}, but no citizen is present at this location!",
                        commuter.id, point
                    )
                }
                Some(citizen) => {
                    trace!("removed the commuter successfully {:?}", citizen);
                }
            }
        }
    }

    pub fn assimilate_migrators(&mut self, incoming: &mut Vec<Migrator>, counts: &mut Counts, rng: &mut RandomWrapper) {
        if incoming.is_empty() {
            return;
        }
        debug!("Assimilating {} incoming migrators", incoming.len());

        let migration_locations = self.select_starting_points(&self.grid.housing_area, incoming.len(), rng);
        if migration_locations.len() < incoming.len() {
            panic!("Not enough housing locations are available for migrators")
        };

        for (migrator, migration_location) in incoming.iter().zip(migration_locations) {
            let house = self.grid.choose_house_with_free_space(rng);
            let office = if migrator.working { self.grid.choose_office_with_free_space(rng) } else { house.clone() };
            let citizen = Citizen::from_migrator(
                migrator,
                house.clone(),
                office.clone(),
                migration_location,
                self.grid.housing_area.clone(),
            );
            self.grid.add_house_occupant(&house.clone());
            if migrator.working {
                self.grid.add_office_occupant(&office.clone())
            }

            CitizenLocationMap::increment_counts(&citizen.state_machine.state, counts);
            let result = self.current_locations.insert(migration_location, citizen);
            assert!(result.is_none());
        }
    }

    pub fn assimilate_commuters(
        &mut self,
        incoming: &mut Vec<Commuter>,
        counts: &mut Counts,
        rng: &mut RandomWrapper,
        simulation_hour: Hour,
    ) {
        if incoming.is_empty() {
            return;
        }
        debug!("Assimilating {} incoming commuters", incoming.len());

        let transport_locations = self.select_starting_points(&self.grid.transport_area, incoming.len(), rng);
        if transport_locations.len() < incoming.len() {
            panic!("Not enough transport location are available for commuters")
        };

        for (commuter, transport_location) in incoming.iter().zip(transport_locations) {
            let work_area: Option<Area> = if simulation_hour == constants::ROUTINE_TRAVEL_START_TIME {
                trace!("inside if of simulation hour");
                let office = self.grid.choose_office_with_free_space(rng);
                trace!("got the office space - {:?}", office.clone());
                self.grid.add_office_occupant(&office.clone());
                trace!("added the office occupant");
                Some(office.clone())
            } else {
                None
            };

            let citizen = Citizen::from_commuter(commuter, transport_location, self.grid.housing_area.clone(), work_area);

            CitizenLocationMap::increment_counts(&citizen.state_machine.state, counts);

            let result = self.current_locations.insert(citizen.transport_location, citizen);
            trace!("citizen inserted");
            assert!(result.is_none());
            trace!("assert passes");
        }

        debug!("For loop ended");
    }

    fn increment_counts(state: &State, counts: &mut Counts) {
        match state {
            State::Susceptible { .. } => counts.update_susceptible(1),
            State::Exposed { .. } => counts.update_exposed(1),
            State::Infected { .. } => counts.update_infected(1),
            State::Recovered { .. } => counts.update_recovered(1),
            State::Deceased { .. } => {
                panic!("Should not receive deceased agent!")
            }
        }
    }

    fn decrement_counts(state: &State, counts: &mut Counts) {
        match state {
            State::Susceptible { .. } => counts.remove_susceptible(1),
            State::Exposed { .. } => counts.remove_exposed(1),
            State::Infected { .. } => counts.remove_infected(1),
            State::Recovered { .. } => counts.remove_recovered(1),
            State::Deceased { .. } => {
                panic!("Deceased agent should not travel!")
            }
        }
    }

    pub fn process_interventions(
        &mut self,
        interventions: &mut Interventions,
        counts_at_hr: &Counts,
        listeners: &mut Listeners,
        rng: &mut RandomWrapper,
        config: &Config,
        engine_id: &String,
    ) {
        self.apply_vaccination_intervention(&interventions.vaccinate, counts_at_hr, rng, listeners);

        if interventions.lockdown.should_apply(counts_at_hr) {
            interventions.lockdown.apply();
            self.lock_city(counts_at_hr.get_hour());
            listeners.intervention_applied(counts_at_hr.get_hour(), &interventions.lockdown)
        }
        if interventions.lockdown.should_unlock(counts_at_hr) {
            self.unlock_city(counts_at_hr.get_hour());
            interventions.lockdown.unapply();
            listeners.intervention_applied(counts_at_hr.get_hour(), &interventions.lockdown)
        }

        interventions.build_new_hospital.counts_updated(counts_at_hr);
        if interventions.build_new_hospital.should_apply(counts_at_hr) {
            info!("Increasing the hospital size");
            self.grid.increase_hospital_size(config.get_grid_size(), engine_id.to_owned());
            interventions.build_new_hospital.apply();

            listeners.grid_updated(&self.grid);
            listeners.intervention_applied(counts_at_hr.get_hour(), &interventions.build_new_hospital);
        }
    }

    fn select_starting_points(&self, area: &Area, no_of_incoming: usize, rng: &mut RandomWrapper) -> Vec<Point> {
        let empty_spaces = (area.start_offset.x..area.end_offset.x).flat_map(|x| {
            (area.start_offset.y..area.end_offset.y)
                .map(move |y| Point { x, y })
                .filter(|z| !self.current_locations.contains_key(z))
        });

        empty_spaces.choose_multiple(rng.get(), no_of_incoming)
    }

    pub fn lock_city(&mut self, hr: Hour) {
        info!("Locking the city. Hour: {}", hr);
        self.iter_mut().for_each(|(_,  r)| {
            if !r.is_essential_worker() {
                (*r).set_isolation(true);
            }
        });
    }

    pub fn unlock_city(&mut self, hr: Hour) {
        info!("Unlocking city. Hour: {}", hr);
        self.iter_mut().for_each(|(_,  r)| {
            if r.is_isolated() {
                (*r).set_isolation(false);
            }
        });
    }

    pub fn apply_vaccination_intervention(
        &mut self,
        vaccinations: &VaccinateIntervention,
        counts: &Counts,
        rng: &mut RandomWrapper,
        listeners: &mut Listeners,
    ) {
        if let Some(vac_percent) = vaccinations.get_vaccination_percentage(counts) {
            info!("Vaccination");
            self.vaccinate(*vac_percent, rng);
            listeners.intervention_applied(counts.get_hour(), vaccinations)
        };
    }

    pub(crate) fn vaccinate(&mut self, vaccination_percentage: f64, rng: &mut RandomWrapper) {
        self.iter_mut().for_each(|( _,  r)| {
            if r.state_machine.is_susceptible() && rng.get().gen_bool(vaccination_percentage) {
                (*r).set_vaccination(true);
            }
        });
    }

    pub fn current_population(&self) -> Count {
        self.current_locations.len() as Count
    }

    pub fn iter_upcoming_locations(
        &self,
    ) -> Iter<'_, Point, Citizen> {
        self.upcoming_locations.iter()
    }

    pub fn par_iter(&self) -> rayon::collections::hash_map::Iter<Point, Citizen> {
        self.current_locations.par_iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Point, Citizen> {
        self.current_locations.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use crate::citizen::WorkStatus;
    use crate::geography::define_geography;

    use super::*;

    fn before_each() -> CitizenLocationMap {
        let mut rng = RandomWrapper::new();
        let points = vec![Point { x: 0, y: 1 }, Point { x: 1, y: 0 }];
        let engine_id = "engine1".to_string();
        let home_locations = vec![
            Area::new(&engine_id.clone(), Point::new(0, 0), Point::new(2, 2)),
            Area::new(&engine_id.clone(), Point::new(3, 0), Point::new(4, 2)),
        ];

        let work_locations = vec![
            Area::new(&engine_id.clone(), Point::new(5, 0), Point::new(6, 2)),
            Area::new(&engine_id.clone(), Point::new(7, 0), Point::new(8, 2)),
        ];
        let working = WorkStatus::NA;
        let non_working = WorkStatus::Normal;

        let agents = vec![
            Citizen::new(home_locations[0].clone(), work_locations[0].clone(), points[0], false, non_working, &mut rng),
            Citizen::new(home_locations[1].clone(), work_locations[0].clone(), points[0], true, working, &mut rng),
        ];
        let grid = define_geography(5, engine_id);
        CitizenLocationMap::new(grid, &agents, &points)
    }

    #[test]
    fn new() {
        let map = before_each();

        assert_eq!(map.grid.grid_size, 5);
    }

    #[test]
    fn should_goto_hospital() {
        let mut rng = RandomWrapper::new();
        let points = vec![Point { x: 0, y: 1 }, Point { x: 1, y: 0 }];
        let working = WorkStatus::NA;
        let non_working = WorkStatus::Normal;
        let engine_id = "engine1".to_string();
        let home_locations = vec![
            Area::new(&engine_id.clone(), Point::new(0, 0), Point::new(2, 2)),
            Area::new(&engine_id.clone(), Point::new(3, 0), Point::new(4, 2)),
        ];

        let work_locations = vec![
            Area::new(&engine_id.clone(), Point::new(5, 0), Point::new(6, 2)),
            Area::new(&engine_id.clone(), Point::new(7, 0), Point::new(8, 2)),
        ];
        let mut citizen1 =
            Citizen::new(home_locations[0].clone(), work_locations[1].clone(), points[0], false, non_working, &mut rng);
        let citizen2 = Citizen::new(home_locations[1].clone(), work_locations[0].clone(), points[0], true, working, &mut rng);
        let agents = vec![citizen1.clone(), citizen2];
        let grid = define_geography(5, engine_id.clone());
        let map = CitizenLocationMap::new(grid, &agents, &points);
        let hospital = Area::new(&engine_id, Point::new(2, 2), Point::new(4, 4));
        let result = map.goto_hospital(&hospital, points[0], &mut citizen1);

        assert!(result.0);
        assert_eq!(result.1, Point::new(2, 2));
    }

    #[test]
    fn should_goto_home_location_when_hospital_full() {
        let mut rng = RandomWrapper::new();
        let engine_id = "engine1".to_string();
        let points = vec![Point::new(0, 0), Point::new(0, 1), Point::new(1, 0), Point::new(1, 1)];
        let home = Area::new(&engine_id.clone(), Point::new(0, 0), Point::new(2, 2));
        let work_status = WorkStatus::NA;

        let work = Area::new(&engine_id.clone(), Point::new(5, 0), Point::new(6, 2));
        let citizen1 = Citizen::new(home.clone(), work.clone(), points[0], false, work_status, &mut rng);
        let citizen2 = Citizen::new(home.clone(), work.clone(), points[0], false, work_status, &mut rng);
        let citizen3 = Citizen::new(home.clone(), work.clone(), points[0], false, work_status, &mut rng);
        let citizen4 = Citizen::new(home, work, points[0], false, work_status, &mut rng);
        let agents = vec![citizen1.clone(), citizen2, citizen3, citizen4];
        let grid = define_geography(5, engine_id.clone());
        let map = CitizenLocationMap::new(grid, &agents, &points);
        let hospital = Area::new(&engine_id, Point::new(0, 0), Point::new(1, 1));

        let result = map.goto_hospital(&hospital, points[0], &mut citizen1.clone());

        assert!(!result.0);
        assert!(citizen1.home_location.contains(&result.1));
    }

    #[test]
    fn should_return_true_when_point_is_in_grid() {
        let map = before_each();
        let points = vec![Point::new(0, 0), Point::new(4, 4), Point::new(2, 2)];
        for point in points {
            assert!(map.is_point_in_grid(&point))
        }
    }

    #[test]
    fn should_return_false_when_point_is_out_of_grid() {
        let map = before_each();
        let points = vec![Point::new(-1, -1), Point::new(5, 5), Point::new(2, 12)];
        for point in points {
            assert!(!map.is_point_in_grid(&point))
        }
    }
}
