use crate::geography::{Area, Point};
use crate::agent::Citizen;
use crate::agent;

pub struct Grid {
    pub housing_area: Area,
    pub work_area: Area,
    pub transport_area: Area,
    pub hospital: Area,
}

impl Grid {
    pub fn generate_population(&self, number_of_agents: i32, public_transport_percentage: f64, working_percentage: f64)
                               -> (Vec<Point>, Vec<Citizen>) {

        //        TODO: fix the hack
        let number_of_agents_using_public_transport = number_of_agents as f64 * (public_transport_percentage + 0.1) * (working_percentage + 0.1);

        let home_locations = self.housing_area.random_points(number_of_agents as i32);

        // assumes that housing starts at 0,0 and work area is the same size as housing area
        // layout: housing | transport | hospital | work
        let scaling_factor = self.hospital.end_offset.x + 1;
        let work_locations: Vec<Point> = home_locations.iter()
            .map(|p| *p + Point::new(scaling_factor, 0)).collect();

        let transport_locations = self.transport_area.random_points(number_of_agents_using_public_transport.ceil() as i32);

        let agent_list = agent::citizen_factory(&home_locations, &work_locations, &transport_locations, public_transport_percentage, working_percentage);
        (home_locations, agent_list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_population() {
        let housing_area = Area::new(Point::new(0, 0), Point::new(10, 10));
        let transport_area = Area::new(Point::new(11, 0), Point::new(20, 10));
        let hospital = Area::new(Point::new(21, 0), Point::new(25, 10));
        let work_area = Area::new(Point::new(26, 0), Point::new(36, 10));

        let grid = Grid { housing_area, work_area, transport_area, hospital };
        let (home_locations, agent_list) = grid.generate_population(10, 0.2, 0.2);

        assert_eq!(home_locations.len(), 10);
        assert_eq!(agent_list.len(), 10);

        for agent in agent_list {
            assert!(housing_area.contains(&agent.home_location));
            assert!(work_area.contains(&agent.work_location)
                || housing_area.contains(&agent.home_location)); //for citizens that are not working
            assert!(transport_area.contains(&agent.transport_location)
                || housing_area.contains(&agent.transport_location)) //for citizens that aren't using public transport
        }
    }
}
