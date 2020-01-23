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

        let scaling_factor = self.housing_area.end_offset.x + self.transport_area.end_offset.x;
        let work_locations: Vec<Point> = home_locations.iter()
            .map(|x| *x + Point::new(scaling_factor, 0)).collect();

        let transport_locations = self.transport_area.random_points(number_of_agents_using_public_transport.ceil() as i32);

        let agent_list = agent::citizen_factory(&home_locations, &work_locations, &transport_locations, public_transport_percentage, working_percentage);
        (home_locations, agent_list)
    }
}
