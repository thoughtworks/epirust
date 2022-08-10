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

use crate::agent::Citizen;
use crate::geography::Point;
use uuid::Uuid;
use crate::commute::Commuter;
use crate::custom_types::Count;
use crate::disease_state_machine::DiseaseStateMachine;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct MigrationPlan {
    regions: Vec<String>,
    matrix: Vec<Vec<u32>>,
}

impl MigrationPlan {
    pub fn new(regions: Vec<String>, matrix: Vec<Vec<u32>>) -> MigrationPlan {
        MigrationPlan { regions, matrix }
    }

    pub fn get_total_outgoing(&self, engine_id: &str) -> u32 {
        let index = self.get_position(engine_id);
        let row = self.matrix.get(index).unwrap();
        row.iter().sum()
    }

    pub fn incoming_regions_count(&self, engine_id: &str) -> u32 {
        let index = self.get_position(engine_id);
        self.column(index).filter(|val| *val > 0).count() as u32
    }

    // pub fn get_total_incoming(&self, engine_id: String) -> i32 {
    //     let index = self.get_position(&engine_id);
    //     self.matrix.iter().fold(0, |total, row| total + *row.get(index).unwrap())
    // }

    pub fn get_outgoing(&self, from_region: &str, to_region: &str) -> u32 {
        let from_index = self.get_position(from_region);
        let to_index = self.get_position(to_region);

        let row = self.matrix.get(from_index).unwrap();
        *row.get(to_index).unwrap()
    }

    fn get_position(&self, engine_id: &str) -> usize {
        self.regions.iter().position(|i| i.eq(engine_id)).expect("Could not find region with specified name")
    }

    fn column(&self, index: usize) -> impl Iterator<Item = u32> + '_ {
        self.matrix.iter().map(move |row| *row.get(index).unwrap())
    }
}

/// Travel plan in the context of the current engine
#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct EngineMigrationPlan {
    engine_id: String,
    migration_plan: Option<MigrationPlan>,
    current_total_population: Count,
}

impl EngineMigrationPlan {
    pub fn new(engine_id: String, migration_plan: Option<MigrationPlan>, current_population: Count) -> EngineMigrationPlan {
        EngineMigrationPlan { engine_id, migration_plan, current_total_population: current_population }
    }

    pub fn percent_outgoing(&self) -> f64 {
        match &self.migration_plan {
            None => 0.0,
            Some(tp) => tp.get_total_outgoing(&self.engine_id) as f64 / self.current_total_population as f64,
        }
    }

    pub fn alloc_outgoing_to_regions(
        &self,
        outgoing: &Vec<(Point, Migrator)>,
    ) -> (Vec<MigratorsByRegion>, Vec<(Point, Migrator)>) {
        let mut migrators: Vec<Migrator> = outgoing.iter().map(|x| x.1).collect();
        let total_outgoing = migrators.len();
        let outgoing_by_region = match &self.migration_plan {
            None => Vec::new(),
            Some(tp) => tp
                .regions
                .iter()
                .filter(|region| !self.engine_id.eq(*region))
                .filter(|region| tp.get_outgoing(self.engine_id(), region) > 0)
                .map(|region| {
                    let mut outgoing_by_region = MigratorsByRegion::create(region);
                    outgoing_by_region.alloc_citizens(&mut migrators, tp, &self.engine_id, total_outgoing as i32);
                    outgoing_by_region
                })
                .collect(),
        };
        let actual_outgoing_migrators: Vec<(Point, Migrator)> =
            outgoing.iter().filter(|x| !migrators.contains(&x.1)).map(|y| (y.0, y.1)).collect();

        //assign remaining citizens (if any) to last region
        // for remaining in travellers {
        //     outgoing_by_region.last_mut().unwrap().alloc_citizen(remaining);
        // }

        (outgoing_by_region, actual_outgoing_migrators)
    }

    pub fn incoming_regions_count(&self) -> u32 {
        match &self.migration_plan {
            None => 0,
            Some(tp) => tp.incoming_regions_count(self.engine_id()),
        }
    }

    pub fn engine_id(&self) -> &String {
        &self.engine_id
    }

    pub fn set_current_population(&mut self, val: Count) {
        self.current_total_population = val;
    }
}

#[derive(Serialize, Deserialize)]
pub struct MigratorsByRegion {
    to_engine_id: String,
    pub migrators: Vec<Migrator>,
}

impl MigratorsByRegion {
    /// Since the actual outgoing count doesn't exactly match the travel plan, we pick a proportion
    /// of the actual outgoing count
    fn actual_outgoing_count(&self, travel_plan: &MigrationPlan, total_outgoing: i32, engine_id: &String) -> i32 {
        let planned_outgoing_for_region = travel_plan.get_outgoing(engine_id, &self.to_engine_id);
        let planned_total_outgoing = travel_plan.get_total_outgoing(engine_id);
        let percent_outgoing = planned_outgoing_for_region as f64 / planned_total_outgoing as f64;
        (percent_outgoing * (total_outgoing as f64)) as i32
    }

    /// Note that this function mutates (drains) the total list of outgoing citizens
    pub fn alloc_citizens(
        &mut self,
        citizens: &mut Vec<Migrator>,
        travel_plan: &MigrationPlan,
        engine_id: &String,
        total_outgoing: i32,
    ) {
        let mut count = self.actual_outgoing_count(travel_plan, total_outgoing, engine_id) as usize;
        if count > citizens.len() {
            debug!("Limiting outgoing citizens to {} instead of {}", citizens.len(), count);
            count = citizens.len();
        }
        self.migrators = citizens.drain(0..count).collect();
    }

    #[cfg(test)]
    pub fn alloc_citizen(&mut self, traveller: Migrator) {
        self.migrators.push(traveller);
    }

    pub fn create(to_engine_id: &String) -> MigratorsByRegion {
        MigratorsByRegion { to_engine_id: to_engine_id.clone(), migrators: Vec::new() }
    }

    pub fn to_engine_id(&self) -> &String {
        &self.to_engine_id
    }

    pub fn get_migrators(self) -> Vec<Migrator> {
        self.migrators
    }

    pub fn get_migrators_slice(&self) -> &[Migrator] {
        self.migrators.as_slice()
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Migrator {
    pub id: Uuid,
    pub immunity: i32,
    pub vaccinated: bool,
    pub uses_public_transport: bool,
    pub working: bool,
    pub state_machine: DiseaseStateMachine,
}

impl Migrator {
    #[cfg(test)]
    pub fn new() -> Migrator {
        Migrator {
            id: Uuid::new_v4(),
            immunity: 0,
            vaccinated: false,
            uses_public_transport: false,
            working: false,
            state_machine: DiseaseStateMachine::new(),
        }
    }
}

impl From<&Citizen> for Migrator {
    fn from(citizen: &Citizen) -> Self {
        Migrator {
            id: citizen.id,
            immunity: citizen.get_immunity(),
            vaccinated: citizen.is_vaccinated(),
            uses_public_transport: citizen.uses_public_transport,
            working: citizen.is_working(),
            state_machine: citizen.state_machine,
        }
    }
}

impl From<&Citizen> for Commuter {
    fn from(citizen: &Citizen) -> Self {
        Commuter {
            id: citizen.id,
            immunity: citizen.get_immunity(),
            home_location: citizen.home_location.clone(),
            work_location: citizen.work_location.clone(),
            vaccinated: citizen.is_vaccinated(),
            uses_public_transport: citizen.uses_public_transport,
            working: citizen.is_working(),
            state_machine: citizen.state_machine,
        }
    }
}

impl PartialEq for Migrator {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_get_total_outgoing() {
        let travel_plan = create_travel_plan();
        assert_eq!(156 + 24, travel_plan.get_total_outgoing("engine1"));
        assert_eq!(108 + 221, travel_plan.get_total_outgoing("engine2"));
        assert_eq!(97 + 12, travel_plan.get_total_outgoing("engine3"));
    }

    // #[test]
    // fn should_get_total_incoming() {
    //     let travel_plan = create_travel_plan();
    //     assert_eq!(108 + 97, travel_plan.get_total_incoming("engine1".to_string()));
    //     assert_eq!(156 + 12, travel_plan.get_total_incoming("engine2".to_string()));
    //     assert_eq!(24 + 221, travel_plan.get_total_incoming("engine3".to_string()));
    // }

    #[test]
    fn should_get_incoming_regions_count() {
        let migration_plan = MigrationPlan {
            regions: vec!["engine1".to_string(), "engine2".to_string(), "engine3".to_string()],
            matrix: vec![vec![0, 0, 0], vec![108, 0, 0], vec![97, 12, 0]],
        };

        assert_eq!(2, migration_plan.incoming_regions_count("engine1"));
        assert_eq!(1, migration_plan.incoming_regions_count("engine2"));
        assert_eq!(0, migration_plan.incoming_regions_count("engine3"));
    }

    #[test]
    fn should_calc_outgoing_percent() {
        let engine_travel_plan = create_engine_with_travel_plan();
        assert_eq!(0.018, engine_travel_plan.percent_outgoing())
    }

    #[test]
    fn should_set_current_population() {
        let mut engine_migration_plan = EngineMigrationPlan::new(
            "engine1".into(),
            Some(MigrationPlan::new(vec!["engine1".into(), "engine2".into()], vec![vec![0, 1], vec![1, 0]])),
            10000,
        );
        engine_migration_plan.set_current_population(9000);
        assert_eq!(9000, engine_migration_plan.current_total_population);
    }

    #[test]
    fn should_assign_outgoing_citizens_to_regions() {
        let engine_travel_plan = create_engine_with_travel_plan();
        let mut outgoing = Vec::new();

        for _i in 0..180 {
            outgoing.push((Point::new(1, 1), create_traveller()));
        }

        let (outgoing_by_region, _) = engine_travel_plan.alloc_outgoing_to_regions(&outgoing);

        assert_eq!(2, outgoing_by_region.len());
        assert_eq!(156, outgoing_by_region.get(0).unwrap().migrators.len());
        assert_eq!(24, outgoing_by_region.get(1).unwrap().migrators.len());
    }

    // Removed the logic of adding remaining traveller in the last engine temporarily.
    // #[test]
    // fn should_handle_outgoing_with_actual_total_less_than_planned() {
    //     let engine_travel_plan = create_engine_with_travel_plan();
    //     let mut outgoing = Vec::new();
    //
    //     for _i in 0..147 {
    //         outgoing.push((Point::new(1, 1), create_traveller()));
    //     }
    //
    //     let (outgoing_by_region, _) = engine_travel_plan.alloc_outgoing_to_regions(&outgoing);
    //
    //     assert_eq!(2, outgoing_by_region.len());
    //     assert_eq!(127, outgoing_by_region.get(0).unwrap().travellers.len());
    //     assert_eq!(20, outgoing_by_region.get(1).unwrap().travellers.len());
    // }
    //
    // #[test]
    // fn should_handle_outgoing_with_actual_total_more_than_planned() {
    //     let engine_travel_plan = create_engine_with_travel_plan();
    //     let mut outgoing = Vec::new();
    //
    //     for _i in 0..202 {
    //         outgoing.push((Point::new(1, 1), create_traveller()));
    //     }
    //
    //     let (outgoing_by_region, _) = engine_travel_plan.alloc_outgoing_to_regions(&outgoing);
    //
    //     assert_eq!(2, outgoing_by_region.len());
    //     assert_eq!(175, outgoing_by_region.get(0).unwrap().travellers.len());
    //     assert_eq!(27, outgoing_by_region.get(1).unwrap().travellers.len());
    // }

    fn create_travel_plan() -> MigrationPlan {
        MigrationPlan {
            regions: vec!["engine1".to_string(), "engine2".to_string(), "engine3".to_string()],
            matrix: vec![vec![0, 156, 24], vec![108, 0, 221], vec![97, 12, 0]],
        }
    }

    fn create_traveller() -> Migrator {
        Migrator::new()
    }

    fn create_engine_with_travel_plan() -> EngineMigrationPlan {
        let migration_plan = create_travel_plan();
        EngineMigrationPlan::new("engine1".into(), Some(migration_plan), 10000)
    }
}
