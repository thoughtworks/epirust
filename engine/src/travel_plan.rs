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

use crate::ticks_consumer::Tick;
use crate::agent::Citizen;
use crate::geography::Point;

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct TravelPlan {
    regions: Vec<String>,
    matrix: Vec<Vec<i32>>,
}

impl TravelPlan {
    #[cfg(test)]
    pub fn new(regions: Vec<String>, matrix: Vec<Vec<i32>>) -> TravelPlan {
        TravelPlan { regions, matrix }
    }

    pub fn get_total_outgoing(&self, engine_id: &String) -> i32 {
        let index = self.get_position(engine_id);
        let row = self.matrix.get(index).unwrap();
        row.iter().sum()
    }

    pub fn incoming_regions_count(&self, engine_id: &String) -> i32 {
        let index = self.get_position(engine_id);
        self.column(index).filter(|val| *val > 0).count() as i32
    }

    pub fn get_total_incoming(&self, engine_id: String) -> i32 {
        let index = self.get_position(&engine_id);
        self.matrix.iter().fold(0, |total, row| total + *row.get(index).unwrap())
    }

    pub fn get_outgoing(&self, from_region: &String, to_region: &String) -> i32 {
        let from_index = self.get_position(from_region);
        let to_index = self.get_position(to_region);

        let row = self.matrix.get(from_index).unwrap();
        *row.get(to_index).unwrap()
    }

    fn get_position(&self, engine_id: &String) -> usize {
        self.regions.iter().position(|i| i.eq(engine_id))
            .expect(format!("Could not find region named {}", engine_id).as_str())
    }

    fn column(&self, index: usize) -> impl Iterator<Item=i32> + '_ {
        self.matrix.iter().map(move |row| *row.get(index).unwrap())
    }
}

/// Travel plan in the context of the current engine
pub struct EngineTravelPlan {
    engine_id: String,
    travel_plan: Option<TravelPlan>,
    current_total_population: i32,
    outgoing: Vec<(Point, Citizen)>,
}

impl EngineTravelPlan {
    pub fn new(engine_id: &String, current_population: i32) -> EngineTravelPlan {
        EngineTravelPlan {
            engine_id: engine_id.clone(),
            travel_plan: None,
            current_total_population: current_population,
            outgoing: Vec::new(),
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
                tp.get_total_outgoing(&self.engine_id) as f64 / self.current_total_population as f64
            }
        }
    }

    pub fn add_outgoing(&mut self, citizen: Citizen, loc: Point) {
        self.outgoing.push((loc, citizen));
    }

    pub fn clear_outgoing(&mut self) {
        self.outgoing.clear();
    }

    pub fn get_outgoing(&self) -> &Vec<(Point, Citizen)> {
        &self.outgoing
    }

    pub fn alloc_outgoing_to_regions(&self) -> Vec<TravellersByRegion> {
        let mut citizens: Vec<Citizen> = self.outgoing.iter().map(|x| x.1).collect();
        let total_outgoing = citizens.len();
        let mut outgoing_by_region = match &self.travel_plan {
            None => { Vec::new() }
            Some(tp) => {
                tp.regions.iter()
                    .filter(|region| !self.engine_id.eq(*region))
                    .map(|region| {
                        let mut outgoing_by_region = TravellersByRegion::create(region);
                        outgoing_by_region.alloc_citizens(&mut citizens, tp, &self.engine_id, total_outgoing as i32);
                        outgoing_by_region
                    }).collect()
            }
        };

        //assign remaining citizens (if any) to last region
        for remaining in citizens {
            outgoing_by_region.last_mut().unwrap().alloc_citizen(remaining);
        }

        outgoing_by_region
    }

    pub fn incoming_regions_count(&self) -> i32 {
        match &self.travel_plan {
            None => { 0 }
            Some(tp) => { tp.incoming_regions_count(self.engine_id()) }
        }
    }

    pub fn engine_id(&self) -> &String {
        &self.engine_id
    }

    fn set_current_population(&mut self, val: i32) {
        self.current_total_population = val;
    }
}

#[derive(Serialize, Deserialize)]
pub struct TravellersByRegion {
    to_engine_id: String,
    citizens: Vec<Citizen>,
}

impl TravellersByRegion {
    /// Since the actual outgoing count doesn't exactly match the travel plan, we pick a proportion
    /// of the actual outgoing count
    fn actual_outgoing_count(&self, travel_plan: &TravelPlan, actual_total_outgoing: i32, engine_id: &String) -> i32 {
        let planned_outgoing_for_region = travel_plan.get_outgoing(engine_id, &self.to_engine_id);
        let planned_total_outgoing = travel_plan.get_total_outgoing(engine_id);
        let percent_outgoing = planned_outgoing_for_region as f64 / planned_total_outgoing as f64;
        (percent_outgoing * (actual_total_outgoing as f64)) as i32
    }

    /// Note that this function mutates (drains) the total list of outgoing citizens
    pub fn alloc_citizens(&mut self, citizens: &mut Vec<Citizen>, travel_plan: &TravelPlan,
                          engine_id: &String, total_outgoing: i32) {
        let mut count = self.actual_outgoing_count(travel_plan, total_outgoing, engine_id) as usize;
        if count > citizens.len() {
            debug!("Limiting outgoing citizens to {} instead of {}", citizens.len(), count);
            count = citizens.len();
        }
        self.citizens = citizens.drain(0..count).collect();
    }

    pub fn alloc_citizen(&mut self, citizen: Citizen) {
        self.citizens.push(citizen);
    }

    pub fn create(to_engine_id: &String) -> TravellersByRegion {
        TravellersByRegion {
            to_engine_id: to_engine_id.clone(),
            citizens: Vec::new(),
        }
    }

    pub fn to_engine_id(&self) -> &String {
        &self.to_engine_id
    }

    pub fn get_citizens(self) -> Vec<Citizen> {
        self.citizens
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
        assert_eq!(156 + 24, travel_plan.get_total_outgoing(&"engine1".to_string()));
        assert_eq!(108 + 221, travel_plan.get_total_outgoing(&"engine2".to_string()));
        assert_eq!(97 + 12, travel_plan.get_total_outgoing(&"engine3".to_string()));
    }

    #[test]
    fn should_get_total_incoming() {
        let travel_plan = create_travel_plan();
        assert_eq!(108 + 97, travel_plan.get_total_incoming("engine1".to_string()));
        assert_eq!(156 + 12, travel_plan.get_total_incoming("engine2".to_string()));
        assert_eq!(24 + 221, travel_plan.get_total_incoming("engine3".to_string()));
    }

    #[test]
    fn should_get_incoming_regions_count() {
        let travel_plan = TravelPlan {
            regions: vec!["engine1".to_string(), "engine2".to_string(), "engine3".to_string()],
            matrix: vec![
                vec![0, 0, 0],
                vec![108, 0, 0],
                vec![97, 12, 0],
            ],
        };

        assert_eq!(2, travel_plan.incoming_regions_count(&"engine1".to_string()));
        assert_eq!(1, travel_plan.incoming_regions_count(&"engine2".to_string()));
        assert_eq!(0, travel_plan.incoming_regions_count(&"engine3".to_string()));
    }

    #[test]
    fn should_keep_previous_travel_plan_on_new_tick() {
        let mut engine_travel_plan = create_engine_with_travel_plan();

        let tick = Tick::new(1, None);
        engine_travel_plan.receive_tick(Some(tick));
        assert_eq!(create_travel_plan(), engine_travel_plan.travel_plan.unwrap());
    }

    #[test]
    fn should_calc_outgoing_percent() {
        let engine_travel_plan = create_engine_with_travel_plan();
        assert_eq!(0.018, engine_travel_plan.percent_outgoing())
    }

    #[test]
    fn should_add_outgoing_citizen() {
        let mut engine_travel_plan = EngineTravelPlan::new(&"engine1".to_string(), 10000);
        engine_travel_plan.add_outgoing(create_citizen(), Point::new(2, 2));
        let citizen_id = engine_travel_plan.outgoing.get(0).unwrap().1.id;

        assert_eq!(create_citizen().id, citizen_id);
    }

    #[test]
    fn should_clear_outgoing_citizens() {
        let mut engine_travel_plan = EngineTravelPlan::new(&"engine1".to_string(), 10000);
        engine_travel_plan.add_outgoing(create_citizen(), Point::new(2, 2));
        engine_travel_plan.clear_outgoing();

        assert!(engine_travel_plan.outgoing.is_empty())
    }

    #[test]
    fn should_set_current_population() {
        let mut engine_travel_plan = EngineTravelPlan::new(&"engine1".to_string(), 10000);
        engine_travel_plan.set_current_population(9000);
        assert_eq!(9000, engine_travel_plan.current_total_population);
    }

    #[test]
    fn should_assign_outgoing_citizens_to_regions() {
        let mut engine_travel_plan = create_engine_with_travel_plan();

        for i in 0..180 {
            engine_travel_plan.add_outgoing(create_citizen_with_id(i + 1), Point::new(1, 1));
        }

        let outgoing_by_region = engine_travel_plan.alloc_outgoing_to_regions();

        assert_eq!(2, outgoing_by_region.len());
        assert_eq!(156, outgoing_by_region.get(0).unwrap().citizens.len());
        assert_eq!(24, outgoing_by_region.get(1).unwrap().citizens.len());
    }

    #[test]
    fn should_handle_outgoing_with_actual_total_less_than_planned() {
        let mut engine_travel_plan = create_engine_with_travel_plan();

        for i in 0..147 {
            engine_travel_plan.add_outgoing(create_citizen_with_id(i + 1), Point::new(1, 1));
        }

        let outgoing_by_region = engine_travel_plan.alloc_outgoing_to_regions();

        assert_eq!(2, outgoing_by_region.len());
        assert_eq!(127, outgoing_by_region.get(0).unwrap().citizens.len());
        assert_eq!(20, outgoing_by_region.get(1).unwrap().citizens.len());
    }

    #[test]
    fn should_handle_outgoing_with_actual_total_more_than_planned() {
        let mut engine_travel_plan = create_engine_with_travel_plan();

        for i in 0..202 {
            engine_travel_plan.add_outgoing(create_citizen_with_id(i + 1), Point::new(1, 1));
        }

        let outgoing_by_region = engine_travel_plan.alloc_outgoing_to_regions();

        assert_eq!(2, outgoing_by_region.len());
        assert_eq!(175, outgoing_by_region.get(0).unwrap().citizens.len());
        assert_eq!(27, outgoing_by_region.get(1).unwrap().citizens.len());
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

    fn create_citizen() -> Citizen {
        create_citizen_with_id(1)
    }

    fn create_citizen_with_id(id: i32) -> Citizen {
        let area = Area::new(Point::new(0, 0), Point::new(10, 10));
        Citizen::new_citizen(id, area, area, Point::new(5, 5), false, false, &mut RandomWrapper::new())
    }

    fn create_engine_with_travel_plan() -> EngineTravelPlan {
        let mut engine_travel_plan = EngineTravelPlan::new(&"engine1".to_string(), 10000);
        let travel_plan = create_travel_plan();
        let tick = Tick::new(0, Some(travel_plan));
        engine_travel_plan.receive_tick(Some(tick));
        engine_travel_plan
    }
}
