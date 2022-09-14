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

use uuid::Uuid;

use crate::models::constants;
use crate::models::custom_types::Hour;
use crate::disease_state_machine::DiseaseStateMachine;
use crate::geography::{Area, Point};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Commuter {
    pub id: Uuid,
    pub immunity: i32,
    pub home_location: Area,
    pub work_location: Area,
    pub vaccinated: bool,
    pub uses_public_transport: bool,
    pub working: bool,
    pub state_machine: DiseaseStateMachine,
}

impl PartialEq for Commuter {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CommutersByRegion {
    to_engine_id: String,
    pub commuters: Vec<Commuter>,
}

impl CommutersByRegion {
    pub fn to_engine_id(&self) -> &String {
        &self.to_engine_id
    }

    pub fn get_commuters(self) -> Vec<Commuter> {
        self.commuters
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Commute {
    pub enabled: bool,
    pub matrix: Option<Vec<Vec<u32>>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CommutePlan {
    pub regions: Vec<String>,
    pub matrix: Vec<Vec<u32>>,
}

impl CommutePlan {
    pub fn incoming_regions_count(&self, engine_id: &String) -> u32 {
        let index = self.get_position(engine_id);
        self.column(index).filter(|val| *val > 0).count() as u32
    }

    pub fn get_outgoing(&self, from_region: &String, to_region: &String) -> u32 {
        let from_index = self.get_position(from_region);
        let to_index = self.get_position(to_region);

        let row = self.matrix.get(from_index).unwrap();
        *row.get(to_index).unwrap()
    }

    fn get_position(&self, engine_id: &String) -> usize {
        self.regions.iter().position(|i| i.eq(engine_id)).expect("Could not find region with specified name")
    }

    fn column(&self, index: usize) -> impl Iterator<Item = u32> + '_ {
        self.matrix.iter().map(move |row| *row.get(index).unwrap())
    }

    pub fn get_total_commuters_by_region(&self, from_region: String) -> Vec<(String, u32)> {
        let mut commuters_by_region: Vec<(String, u32)> = vec![];
        for region in &self.regions {
            commuters_by_region.push((region.to_string(), self.get_outgoing(&from_region, region)))
        }
        commuters_by_region
    }

    pub fn get_commuters_by_region(&self, commuters: &Vec<(Point, Commuter)>, simulation_hour: Hour) -> Vec<CommutersByRegion> {
        let mut commuters_by_region: Vec<CommutersByRegion> = Vec::new();
        for region in &self.regions {
            let mut commuters_for_region: Vec<Commuter> = Vec::new();
            for (_point, commuter) in commuters {
                if simulation_hour % 24 == constants::ROUTINE_TRAVEL_START_TIME && commuter.work_location.location_id == *region {
                    commuters_for_region.push(commuter.clone())
                }
                if simulation_hour % 24 == constants::ROUTINE_TRAVEL_END_TIME && commuter.home_location.location_id == *region {
                    commuters_for_region.push(commuter.clone())
                }
            }
            commuters_by_region.push(CommutersByRegion { to_engine_id: region.clone(), commuters: commuters_for_region })
        }
        commuters_by_region
    }
}
