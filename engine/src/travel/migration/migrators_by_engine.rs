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

use common::models::travel_plan::TravelPlan;
use common::models::MigrationPlan;

use crate::travel::migration::Migrator;

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

    pub fn create(to_engine_id: &str) -> MigratorsByRegion {
        MigratorsByRegion { to_engine_id: to_engine_id.to_owned(), migrators: Vec::new() }
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
