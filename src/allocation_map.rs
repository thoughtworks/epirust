use fxhash::FxHashMap;

use crate::agent;
use crate::geography::Area;
use crate::geography::Point;

pub struct AgentLocationMap {
    pub grid_size: i32,
    pub agent_cell: FxHashMap<Point, agent::Citizen>,
}

impl AgentLocationMap {
    pub fn new(size: i32, agent_list: &[agent::Citizen], points: &[Point]) -> AgentLocationMap {
        let mut map: FxHashMap<Point, agent::Citizen> = FxHashMap::default();
        for i in 0..agent_list.len() {
            map.insert(points[i], agent_list[i]);
        }

        AgentLocationMap { grid_size: size, agent_cell: map }
    }

    pub fn move_agent(&self, old_cell: Point, new_cell: Point) -> Point {
        if self.agent_cell.contains_key(&new_cell) {
            return old_cell;
        }
        new_cell
    }

    pub fn goto_hospital(&self, hospital_area: &Area, cell: Point, citizen: &mut agent::Citizen) -> Point {
        let vacant_hospital_cell = hospital_area.into_iter().find(|cell| {
            !self.agent_cell.contains_key(&cell)
        });
        self.move_agent(cell, vacant_hospital_cell.unwrap_or(citizen.home_location))
    }

//    pub fn print(&self){
//        for (k,v) in self.agent_cell.iter(){
//            println!("x:{}, y:{} - id:{} infected:{} working:{} Transport:{}", k.x, k.y, v.id, v.is_infected(), v.working, v.uses_public_transport);
//        }
//    }

    pub fn get_agent_for(&self, cell: &Point) -> Option<&agent::Citizen> {
        self.agent_cell.get(cell)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn before_each() -> AgentLocationMap {
        let points = vec![Point { x: 0, y: 1 }, Point { x: 1, y: 0 }];
        let agents = vec![agent::Citizen::new_citizen(1, points[0], points[1], points[0], false, false), agent::Citizen::new_citizen(2, points[1], points[0], points[0], true, true)];
        AgentLocationMap::new(5, &agents, &points)
    }

    #[test]
    fn new() {
        let map = before_each();
        let actual_citizen = map.agent_cell.get(&Point { x: 0, y: 1 }).unwrap();

        assert_eq!(map.grid_size, 5);
        assert_eq!(actual_citizen.id, 1);
    }

    #[test]
    fn should_goto_hospital() {
        let points = vec![Point { x: 0, y: 1 }, Point { x: 1, y: 0 }];
        let mut citizen1 = agent::Citizen::new_citizen(1, points[0], points[1], points[0], false, false);
        let citizen2 = agent::Citizen::new_citizen(2, points[1], points[0], points[0], true, true);
        let agents = vec![citizen1, citizen2];
        let map = AgentLocationMap::new(5, &agents, &points);
        let hospital = Area::new(Point::new(2, 2), Point::new(4, 4));

        assert_eq!(map.goto_hospital(&hospital, points[0], &mut citizen1), Point::new(2, 2));
    }

    #[test]
    fn should_goto_home_location_when_hospital_full() {
        let points = vec![Point::new(0, 0), Point::new(0, 1), Point::new(1, 0), Point::new(1, 1)];
        let home = Point::new(2,0);
        let work = Point::new(2,1);
        let mut citizen1 = agent::Citizen::new_citizen(1, home, work, home, false, false);
        let citizen2 = agent::Citizen::new_citizen(2, home, work, home, false, false);
        let citizen3 = agent::Citizen::new_citizen(3, home, work, home, false, false);
        let citizen4 = agent::Citizen::new_citizen(4, home, work, home, false, false);
        let agents = vec![citizen1, citizen2, citizen3, citizen4];
        let map = AgentLocationMap::new(5, &agents, &points);
        let hospital = Area::new(Point::new(0, 0), Point::new(1, 1));

        assert_eq!(map.goto_hospital(&hospital, points[0], &mut citizen1), home);
    }
}
