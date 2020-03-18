use crate::geography::{Area, Point};
use crate::agent::Citizen;
use crate::agent;
use crate::random_wrapper::RandomWrapper;
use crate::config::{AutoPopulation, CsvPopulation};

pub struct Grid {
    pub housing_area: Area,
    pub work_area: Area,
    pub transport_area: Area,
    pub hospital: Area,
}

impl Grid {
    pub fn generate_population(&self, auto_pop: &AutoPopulation, rng: &mut RandomWrapper) -> (Vec<Point>, Vec<Citizen>) {

        let number_of_agents = auto_pop.number_of_agents;
        let working_percentage = auto_pop.working_percentage;
        let public_transport_percentage = auto_pop.public_transport_percentage;

        //        TODO: fix the hack
        let number_of_agents_using_public_transport = number_of_agents as f64 * (public_transport_percentage + 0.1) * (working_percentage + 0.1);

        let home_locations = self.housing_area.random_points(number_of_agents as i32, rng);

        // assumes that housing starts at 0,0 and work area is the same size as housing area
        // layout: housing | transport | hospital | work
        let scaling_factor = self.hospital.end_offset.x + 1;
        let work_locations: Vec<Point> = home_locations.iter()
            .map(|p| *p + Point::new(scaling_factor, 0)).collect();

        let transport_locations = self.transport_area.random_points(number_of_agents_using_public_transport.ceil() as i32, rng);

        let agent_list = agent::citizen_factory(&home_locations, &work_locations, &transport_locations, public_transport_percentage, working_percentage, rng);
        (home_locations, agent_list)
    }

    pub fn read_population(&self, csv_pop: &CsvPopulation) -> (Vec<Point>, Vec<Citizen>) {
        panic!("Not yet implemented");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_population() {
        let mut rng = RandomWrapper::new();
        let housing_area = Area::new(Point::new(0, 0), Point::new(10, 10));
        let transport_area = Area::new(Point::new(11, 0), Point::new(20, 10));
        let hospital = Area::new(Point::new(21, 0), Point::new(25, 10));
        let work_area = Area::new(Point::new(26, 0), Point::new(36, 10));

        let grid = Grid { housing_area, work_area, transport_area, hospital };
        let pop = AutoPopulation {
            number_of_agents: 10,
            public_transport_percentage: 0.2,
            working_percentage: 0.2,
        };
        let (home_locations, agent_list) = grid.generate_population(&pop, &mut rng);

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
