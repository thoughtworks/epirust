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
use crate::citizen::Citizen;
use crate::disease_state_machine::DiseaseStateMachine;
use crate::geography::Area;

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
