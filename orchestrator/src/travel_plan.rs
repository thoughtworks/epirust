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

#[derive(Deserialize)]
struct TravelPlan {
    regions: Vec<String>,
    matrix: Vec<Vec<i32>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn should_deserialize() {
        let file = File::open("config/test/travel_plan.json").unwrap();
        let travel_plan: TravelPlan = serde_json::from_reader(file).unwrap();

        assert_eq!(travel_plan.regions, vec!["engine1".to_string(), "engine2".to_string(),
                                             "engine3".to_string()]);
        assert_eq!(travel_plan.matrix, vec![
            vec![0, 156, 10],
            vec![108, 0, 290],
            vec![90, 75, 0]
        ]);
    }

}
