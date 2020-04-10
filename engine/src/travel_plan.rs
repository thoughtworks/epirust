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

use std::collections::HashMap;
use crate::ticks_consumer::Tick;
use crate::agent::Citizen;
use crate::geography::Point;

#[derive(Debug, Deserialize, PartialEq)]
pub struct TravelPlan {
    regions: Vec<String>,
    matrix: Vec<Vec<i32>>,
}

impl TravelPlan {
    #[cfg(test)]
    pub fn new(regions: Vec<String>, matrix: Vec<Vec<i32>>) -> TravelPlan {
        TravelPlan { regions, matrix }
    }

    pub fn get_total_outgoing(&self, engine_id: String) -> i32 {
        let index = self.get_position(engine_id);
        let row = self.matrix.get(index).unwrap();
        row.iter().sum()
    }

    pub fn get_total_incoming(&self, engine_id: String) -> i32 {
        let index = self.get_position(engine_id);
        self.matrix.iter().fold(0, |total, row| total + *row.get(index).unwrap())
    }

    fn get_position(&self, engine_id: String) -> usize {
        self.regions.iter().position(|i| i.eq(&engine_id))
            .expect(format!("Could not find region named {}", engine_id).as_str())
    }
}

/// Travel plan in the context of the current engine
pub struct EngineTravelPlan {
    engine_id: String,
    travel_plan: Option<TravelPlan>,
    current_total_population: i32,
    outgoing: HashMap<Point, Citizen>,
}

impl EngineTravelPlan {
    pub fn new(engine_id: &String, current_population: i32) -> EngineTravelPlan {
        EngineTravelPlan {
            engine_id: engine_id.clone(),
            travel_plan: None,
            current_total_population: current_population,
            outgoing: HashMap::new(),
        }
    }

    pub fn receive_tick(&mut self, tick: Option<Tick>) {
        match tick {
            None => {}
            Some(t) => {
                self.clear_outgoing();
                match t.travel_plan() {
                    None => {}
                    Some(tp) => { self.travel_plan = Some(tp) }
                }
            }
        }
    }

    pub fn percent_outgoing(&self) -> f64 {
        match &self.travel_plan {
            None => { 0.0 }
            Some(tp) => {
                tp.get_total_outgoing(self.engine_id.clone()) as f64 / self.current_total_population as f64
            }
        }
    }

    pub fn add_outgoing(&mut self, citizen: Citizen, loc: Point) {
        self.outgoing.insert(loc, citizen);
    }

    pub fn clear_outgoing(&mut self) {
        self.outgoing.clear();
    }

    pub fn get_outgoing(&self) -> &HashMap<Point, Citizen> {
        &self.outgoing
    }

    fn set_current_population(&mut self, val: i32) {
        self.current_total_population = val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geography::Area;
    use crate::random_wrapper::RandomWrapper;

    #[test]
    fn should_get_total_outgoing() {
        let travel_plan = create_travel_plan();
        assert_eq!(156 + 24, travel_plan.get_total_outgoing("engine1".to_string()));
        assert_eq!(108 + 221, travel_plan.get_total_outgoing("engine2".to_string()));
        assert_eq!(97 + 12, travel_plan.get_total_outgoing("engine3".to_string()));
    }

    #[test]
    fn should_get_total_incoming() {
        let travel_plan = create_travel_plan();
        assert_eq!(108 + 97, travel_plan.get_total_incoming("engine1".to_string()));
        assert_eq!(156 + 12, travel_plan.get_total_incoming("engine2".to_string()));
        assert_eq!(24 + 221, travel_plan.get_total_incoming("engine3".to_string()));
    }

    fn create_travel_plan() -> TravelPlan {
        TravelPlan {
            regions: vec!["engine1".to_string(), "engine2".to_string(), "engine3".to_string()],
            matrix: vec![
                vec![0, 156, 24],
                vec![108, 0, 221],
                vec![97, 12, 0],
            ],
        }
    }

    #[test]
    fn should_keep_previous_travel_plan_on_new_tick() {
        let mut engine_travel_plan = EngineTravelPlan::new(&"engine1".to_string(), 10000);
        let travel_plan = create_travel_plan();
        let tick = Tick::new(0, Some(travel_plan));
        engine_travel_plan.receive_tick(Some(tick));

        let tick = Tick::new(1, None);
        engine_travel_plan.receive_tick(Some(tick));
        assert_eq!(create_travel_plan(), engine_travel_plan.travel_plan.unwrap());
    }

    #[test]
    fn should_calc_outgoing_percent() {
        let mut engine_travel_plan = EngineTravelPlan::new(&"engine1".to_string(), 10000);
        let travel_plan = create_travel_plan();
        let tick = Tick::new(0, Some(travel_plan));
        engine_travel_plan.receive_tick(Some(tick));
        assert_eq!(0.018, engine_travel_plan.percent_outgoing())
    }

    #[test]
    fn should_add_outgoing_citizen() {
        let mut engine_travel_plan = EngineTravelPlan::new(&"engine1".to_string(), 10000);
        engine_travel_plan.add_outgoing(create_citizen(), Point::new(2,2));
        let citizen_id = engine_travel_plan.outgoing.get(&Point::new(2, 2)).unwrap().id;

        assert_eq!(create_citizen().id, citizen_id);
    }

    #[test]
    fn should_clear_outgoing_citizens() {
        let mut engine_travel_plan = EngineTravelPlan::new(&"engine1".to_string(), 10000);
        engine_travel_plan.add_outgoing(create_citizen(), Point::new(2,2));
        engine_travel_plan.clear_outgoing();

        assert!(engine_travel_plan.outgoing.is_empty())
    }

    #[test]
    fn should_set_current_population() {
        let mut engine_travel_plan = EngineTravelPlan::new(&"engine1".to_string(), 10000);
        engine_travel_plan.set_current_population(9000);
        assert_eq!(9000, engine_travel_plan.current_total_population);
    }

    fn create_citizen() -> Citizen {
        let area = Area::new(Point::new(0,0), Point::new(10,10));
        Citizen::new_citizen(1, area, area, Point::new(5,5), false, false, &mut RandomWrapper::new())
    }

}
