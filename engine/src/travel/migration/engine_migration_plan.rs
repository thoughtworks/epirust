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

use bincode::deserialize;
use mpi::point_to_point::Source;
use mpi::Rank;
use mpi::topology::SystemCommunicator;
use mpi::traits::Communicator;
use crate::models::custom_types::{Count, Hour};
use crate::models::travel_plan::TravelPlan;

use crate::geography::Point;
use crate::models::events::Tick;
use crate::models::migration_plan::MigrationPlan;
use crate::travel::migration::{Migrator, MigratorsByRegion};

/// Travel plan in the context of the current engine
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
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

    pub fn alloc_outgoing_to_regions(&self, outgoing: &[(Point, Migrator)]) -> (Vec<MigratorsByRegion>, Vec<(Point, Migrator)>) {
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

    pub async fn receive_migrators(&self, hour: Hour, world: SystemCommunicator, engine_ranks: &Vec<Rank>) -> Vec<Migrator> {
        let mut incoming: Vec<Migrator> = Vec::new();
        if hour % 24 == 0 {
            let expected_incoming_regions = self.incoming_regions_count();
            info!("inside receive migrator");
            debug!("Receiving migrators from {} regions", expected_incoming_regions);
            let my_rank = world.rank();
            let mut buffer = vec![0u8; 1024];
            let receiving_ranks : Vec<_>  = engine_ranks.iter().filter(|&r|{*r != my_rank}).collect();
            info!("my rank - {}, receiving ranks - {:?}", my_rank, receiving_ranks);
            for &rank in receiving_ranks.iter() {
                let status = world.process_at_rank(*rank).receive_into(&mut buffer[..]);
                let received: MigratorsByRegion = deserialize(&buffer[..]).unwrap();
                info!("rank - {:?}, simulation_hour - {}, {}, {:?}", my_rank, hour, received.migrators.len(), status);
                incoming.extend(received.get_migrators());
            }
        }
        incoming
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    fn create_traveller() -> Migrator {
        Migrator::new()
    }

    fn create_travel_plan() -> MigrationPlan {
        MigrationPlan {
            regions: vec!["engine1".to_string(), "engine2".to_string(), "engine3".to_string()],
            matrix: vec![vec![0, 156, 24], vec![108, 0, 221], vec![97, 12, 0]],
        }
    }

    fn create_engine_with_travel_plan() -> EngineMigrationPlan {
        let migration_plan = create_travel_plan();
        EngineMigrationPlan::new("engine1".into(), Some(migration_plan), 10000)
    }
}
