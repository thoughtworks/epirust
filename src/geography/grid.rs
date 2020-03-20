use plotters::drawing::SVGBackend;
use plotters::prelude::*;

use crate::{agent, constants};
use crate::agent::Citizen;
use crate::config::{AutoPopulation, CsvPopulation};
use crate::geography::{Area, area, Point};
use crate::random_wrapper::RandomWrapper;
use plotters::drawing::rasterizer::draw_rect;

pub struct Grid {
    pub grid_size: i32,
    pub housing_area: Area,
    pub work_area: Area,
    pub transport_area: Area,
    pub hospital_area: Area,
}

impl Grid {
    pub fn generate_population(&self, auto_pop: &AutoPopulation, rng: &mut RandomWrapper) -> (Vec<Point>, Vec<Citizen>) {

        let number_of_agents = auto_pop.number_of_agents;
        let working_percentage = auto_pop.working_percentage;
        let public_transport_percentage = auto_pop.public_transport_percentage;

        //        TODO: fix the hack
        let number_of_agents_using_public_transport = number_of_agents as f64 * (public_transport_percentage + 0.1) * (working_percentage + 0.1);

        let home_locations = self.housing_area.random_points(number_of_agents as i32, rng);
        let homes = area::area_factory(self.housing_area.start_offset, self.housing_area.end_offset, constants::HOME_SIZE);

        // assumes that housing starts at 0,0 and work area is the same size as housing area
        // layout: housing | transport | hospital | work
        let scaling_factor = self.hospital_area.end_offset.x + 1;

        let office_start_point = Point::new(self.hospital_area.end_offset.x + 1, self.housing_area.start_offset.y);
        let office_end_point = Point::new(scaling_factor + self.housing_area.end_offset.x + 1, self.hospital_area.end_offset.y + 1);

        let offices = area::area_factory(office_start_point, office_end_point, constants::OFFICE_SIZE);

        let transport_locations = self.transport_area.random_points(number_of_agents_using_public_transport.ceil() as i32, rng);

        let agent_list = agent::citizen_factory(number_of_agents, &homes, &offices, &transport_locations, public_transport_percentage, working_percentage, rng);

        let mut svg = SVGBackend::new("grid.svg", (self.grid_size as u32, self.grid_size as u32));
        Grid::draw_rect(&mut svg, &self.housing_area, &plotters::style::YELLOW);
        Grid::draw_rect(&mut svg, &self.transport_area, &plotters::style::RGBColor(121, 121, 121));
        Grid::draw_rect(&mut svg, &self.work_area, &plotters::style::BLUE);
        Grid::draw_rect(&mut svg, &self.hospital_area, &plotters::style::RED);
        for home in homes {
            Grid::draw_rect(&mut svg, &home, &plotters::style::RGBColor(204, 153, 0));
        }
        for office in offices {
            Grid::draw_rect(&mut svg, &office, &plotters::style::RGBColor(51, 153, 255));
        }
        (home_locations, agent_list)
    }

    fn draw_rect(svg: &mut SVGBackend, area: &Area, style: &RGBColor) {
        svg.draw_rect((area.start_offset.x, area.start_offset.y),
                      (area.end_offset.x, area.end_offset.y),
                      style, true).unwrap();
    }

    pub fn read_population(&self, csv_pop: &CsvPopulation) -> (Vec<Point>, Vec<Citizen>) {
        panic!("Not yet implemented");
    }

    pub fn increase_hospital_size(&mut self, grid_size: i32, scale_factor: i32) {
        let bound = grid_size - 1;
        let hospital_increased_relative_size = constants::HOSPITAL_RELATIVE_SIZE * scale_factor as f32;
        let new_x_offset = (bound as f32 * hospital_increased_relative_size).ceil() as i32;
        let start_offset = self.hospital_area.start_offset;
        let end_offset = Point::new(new_x_offset, bound);

        self.hospital_area = Area::new(start_offset, end_offset)
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
        let hospital_area = Area::new(Point::new(21, 0), Point::new(25, 10));
        let work_area = Area::new(Point::new(26, 0), Point::new(36, 10));

        let grid = Grid { grid_size: 36, housing_area, work_area, transport_area, hospital_area };
        let pop = AutoPopulation {
            number_of_agents: 10,
            public_transport_percentage: 0.2,
            working_percentage: 0.2,
        };
        let (home_locations, agent_list) = grid.generate_population(&pop, &mut rng);

        assert_eq!(home_locations.len(), 10);
        assert_eq!(agent_list.len(), 10);

        for agent in agent_list {
            assert!(housing_area.contains(&agent.home_location.start_offset));
            assert!(work_area.contains(&agent.work_location.end_offset)
                || housing_area.contains(&agent.home_location.start_offset)); //for citizens that are not working
            assert!(transport_area.contains(&agent.transport_location)
                || housing_area.contains(&agent.transport_location)) //for citizens that aren't using public transport
        }
    }
}
