/*
 * EpiRust
 * Copyright (c) 2022  ThoughtWorks, Inc.
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

pub trait TravelPlan {
    fn get_regions(&self) -> &Vec<String>;
    fn get_matrix(&self) -> &Vec<Vec<u32>>;
    fn get_position(&self, engine_id: &String) -> usize {
        self.get_regions().iter().position(|i| i.eq(engine_id)).expect("Could not find region with specified name")
    }

    fn incoming_regions_count(&self, engine_id: &String) -> u32 {
        let index = self.get_position(engine_id);
        self.get_matrix().iter().map(move |row| *row.get(index).unwrap()).filter(|val| *val > 0).count() as u32
    }

    fn get_total_incoming(&self, region_id: &String) -> u32 {
        let region_index = self.get_position(region_id);
        self.get_matrix().iter().map(move |row| *row.get(region_index).unwrap()).sum()
    }

    fn get_total_outgoing(&self, region_id: &String) -> u32 {
        let from_index = self.get_position(region_id);
        let row = self.get_matrix().get(from_index).unwrap();
        row.iter().sum()
    }

    fn get_outgoing(&self, from_region: &String, to_region: &String) -> u32 {
        let from_index = self.get_position(from_region);
        let to_index = self.get_position(to_region);

        let row = self.get_matrix().get(from_index).unwrap();
        *row.get(to_index).unwrap()
    }
}
