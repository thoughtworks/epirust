use hashbrown::HashMap;

use crate::agent;
use crate::geography::Area;
use crate::geography::Point;
use crate::utils;

pub struct AgentLocationMap {
    pub grid_size: i32,
    pub agent_cell: HashMap<Point, agent::Citizen>,
}

impl AgentLocationMap {
    pub fn new(size: i32, agent_list: &[agent::Citizen], points: &[Point]) -> AgentLocationMap {
        let mut map: HashMap<Point, agent::Citizen> = HashMap::new();
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

    pub fn goto_hospital(&self, area: Area, cell: Point, citizen: &mut agent::Citizen) -> Point {
        let area_dimensions = area.get_points_within();
        let vacant_cells = AgentLocationMap::get_empty_cells_from_map(&self.agent_cell, area_dimensions);
        self.move_agent(cell, utils::get_random_element_from(&vacant_cells, citizen.home_location))
    }

//    pub fn print(&self){
//        for (k,v) in self.agent_cell.iter(){
//            println!("x:{}, y:{} - id:{} infected:{} working:{} Transport:{}", k.x, k.y, v.id, v.is_infected(), v.working, v.uses_public_transport);
//        }
//    }

    pub fn get_empty_cells_from_map(hash_map: &HashMap<Point, agent::Citizen>, neighbors: Vec<Point>) -> Vec<Point> {
        neighbors.into_iter().filter(|key| !hash_map.contains_key(key)).collect()
    }

    pub fn get_agents_from(&self, neighbors: &Vec<Point>) -> Vec<&agent::Citizen> {
        let mut agent_list = Vec::with_capacity(8);
        for neighbor in neighbors {
            let agent = self.agent_cell.get(&neighbor);
            if let Some(x) = agent { agent_list.push(x) }
        }
        agent_list
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
    fn should_get_empty_cells() {
        let map = before_each();

        let empty_cells = AgentLocationMap::get_empty_cells_from_map(&map.agent_cell, Point { x: 0, y: 1 }.get_neighbor_cells(5));
        assert_eq!(empty_cells.len(), 4);
    }

    #[test]
    fn should_get_neighbor_agents() {
        let map = before_each();

        let neighbor_agents = map.get_agents_from(&Point { x: 0, y: 1 }.get_neighbor_cells(5));
        assert_eq!(neighbor_agents.len(), 1);
    }
}
