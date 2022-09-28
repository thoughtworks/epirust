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

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct MigrationPlan {
    pub(in crate::travel::migration) regions: Vec<String>,
    pub(in crate::travel::migration) matrix: Vec<Vec<u32>>,
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

#[cfg(test)]
mod test {
    use crate::travel::migration::MigrationPlan;

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

    fn create_travel_plan() -> MigrationPlan {
        MigrationPlan {
            regions: vec!["engine1".to_string(), "engine2".to_string(), "engine3".to_string()],
            matrix: vec![vec![0, 156, 24], vec![108, 0, 221], vec![97, 12, 0]],
        }
    }
}
