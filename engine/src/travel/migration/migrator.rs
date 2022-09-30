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

use crate::citizen::Citizen;
use crate::disease_state_machine::DiseaseStateMachine;
use uuid::Uuid;

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

impl PartialEq for Migrator {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
