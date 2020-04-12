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

use plotters::prelude::*;

use crate::{agent, constants};
use crate::agent::{Citizen, PopulationRecord};
use crate::config::{AutoPopulation, CsvPopulation};
use crate::geography::{Area, area, Point};
use crate::random_wrapper::RandomWrapper;
use std::fs::File;
use crate::geography::area::AreaPointIterator;
use std::collections::HashMap;
use rand::seq::SliceRandom;

#[derive(Serialize)]
pub struct Grid {
    pub grid_size: i32,
    pub housing_area: Area,
    pub work_area: Area,
    pub transport_area: Area,
    pub hospital_area: Area,
    pub houses: Vec<Area>,
    pub offices: Vec<Area>
}

impl Grid {
    pub fn generate_population(&self, auto_pop: &AutoPopulation, rng: &mut RandomWrapper) -> (Vec<Point>, Vec<Citizen>) {
        debug!("Generating Population");
        let number_of_agents = auto_pop.number_of_agents;
        let working_percentage = auto_pop.working_percentage;
        let public_transport_percentage = auto_pop.public_transport_percentage;
        let mut home_loc:Vec<Point> = Vec::new();

        //        TODO: fix the hack
        let number_of_agents_using_public_transport = number_of_agents as f64 * (public_transport_percentage + 0.1) * (working_percentage + 0.1);

        let transport_locations = self.transport_area.random_points(number_of_agents_using_public_transport.ceil() as i32, rng);
        debug!("Finished generating transport locations");

        let agent_list = agent::citizen_factory(number_of_agents, &self.houses, &self.offices, &transport_locations, public_transport_percentage, working_percentage, rng);
        debug!("Finished creating agent list");

        let agents_by_home_locations = Grid::group_agents_by_home_locations(&agent_list);
        debug!("Finished grouping agents by home locations");
        let mut agents_in_order:Vec<Citizen> = Vec::with_capacity(agent_list.len());

        for(home, agents) in agents_by_home_locations{
            trace!("home: {:?} {:?}", home.start_offset, home.end_offset);
            trace!("agents in home: {:?}", agents.len());
            let mut random_points_within_home = home.random_points(agents.len() as i32, rng);

            for agent in agents{
                agents_in_order.push(*agent);
            }
            home_loc.append(&mut random_points_within_home);
        }
        debug!("Assigned starting location to agents");

        self.draw(&home_loc, &self.houses, &self.offices);
        (home_loc, agents_in_order)
    }

    pub fn group_agents_by_home_locations(agent_list: &Vec<Citizen>) -> HashMap<&Area, Vec<&Citizen>> {
        let mut agents_by_home_locations: HashMap<&Area, Vec<&Citizen>> = HashMap::new();
        agent_list.iter().for_each(|agent| {
            match agents_by_home_locations.get(&agent.home_location) {
                None => {
                    agents_by_home_locations.insert(&agent.home_location, vec![&agent]);
                }
                Some(citizens) => {
                    let mut updated_citizens = citizens.clone();
                    updated_citizens.push(&agent);
                    agents_by_home_locations.insert(&agent.home_location, updated_citizens);
                }
            }
        });
        agents_by_home_locations
    }

    fn draw(&self, home_locations: &Vec<Point>, homes: &Vec<Area>, offices: &Vec<Area>) {
        let mut draw_backend = BitMapBackend::new("grid.png", (self.grid_size as u32, self.grid_size as u32));
        Grid::draw_rect(&mut draw_backend, &self.housing_area, &plotters::style::YELLOW);
        Grid::draw_rect(&mut draw_backend, &self.transport_area, &plotters::style::RGBColor(121, 121, 121));
        Grid::draw_rect(&mut draw_backend, &self.work_area, &plotters::style::BLUE);
        Grid::draw_rect(&mut draw_backend, &self.hospital_area, &plotters::style::RED);
        for home in homes {
            Grid::draw_rect(&mut draw_backend, home, &plotters::style::RGBColor(204, 153, 0));
        }
        for office in offices {
            Grid::draw_rect(&mut draw_backend, office, &plotters::style::RGBColor(51, 153, 255));
        }
        for home in home_locations {
            draw_backend.draw_pixel((home.x, home.y), &plotters::style::BLACK.to_rgba()).unwrap();
        }
    }

    fn draw_rect(svg: &mut impl DrawingBackend, area: &Area, style: &RGBColor) {
        svg.draw_rect((area.start_offset.x, area.start_offset.y),
                      (area.end_offset.x, area.end_offset.y),
                      style, true).unwrap();
    }

    pub fn read_population(&self, csv_pop: &CsvPopulation, rng: &mut RandomWrapper) -> (Vec<Point>, Vec<Citizen>) {
        let file = File::open(&csv_pop.file).expect("Could not read population file");
        let mut rdr = csv::Reader::from_reader(file);
        let mut homes = area::area_factory(self.housing_area.start_offset, self.housing_area.end_offset, constants::HOME_SIZE);
        let scaling_factor = self.hospital_area.end_offset.x + 1;

        let office_start_point = Point::new(self.hospital_area.end_offset.x + 1, self.housing_area.start_offset.y);
        let office_end_point = Point::new(scaling_factor + self.housing_area.end_offset.x + 1, self.hospital_area.end_offset.y + 1);

        let offices = area::area_factory(office_start_point, office_end_point, constants::OFFICE_SIZE);
        let mut offices_iter = offices.iter().cycle();

        let mut citizens = Vec::new();
        let mut home_loc = Vec::new();
        let mut home_points_iter = AreaPointIterator::init(&mut homes);

        for result in rdr.deserialize() {
            let record: PopulationRecord = result.expect("Could not deserialize population line");
            let (home_area, home_point) = home_points_iter.next().expect("Ran out of homes!");

            //TODO seems like transport point isn't being used on the routine() function
            let citizen = Citizen::from_record(record, home_area, *offices_iter.next().unwrap(), home_point, rng);
            citizens.push(citizen);
            home_loc.push(home_point);
        }
        self.draw(&home_loc, &homes, &offices);
        (home_loc, citizens)
    }

    pub fn increase_hospital_size(&mut self, grid_size: i32) {
        let start_offset = self.hospital_area.start_offset;
        let end_offset = Point::new(grid_size, grid_size);

        self.hospital_area = Area::new(start_offset, end_offset)
    }

    pub fn choose_house_with_free_space(&self, citizens: &[&Citizen], rng: &mut RandomWrapper) -> Area {
        let house_capacity = constants::HOME_SIZE * constants::HOME_SIZE;
        loop {
            let house = self.houses.choose(rng.get()).unwrap();
            let occupants = Grid::find_home_occupants(&citizens, house);
            if occupants < house_capacity {
                return *house;
            }
        }
    }

    pub fn choose_office_with_free_space(&self, citizens: &[&Citizen], rng: &mut RandomWrapper) -> Area {
        let office_capacity = constants::OFFICE_SIZE * constants::OFFICE_SIZE;
        loop {
            let office = self.offices.choose(rng.get()).unwrap();
            let occupants = Grid::find_office_occupants(&citizens, office);
            if occupants < office_capacity {
                return *office;
            }
        }
    }

    fn find_home_occupants(citizens: &[&Citizen], house: &Area) -> i32 {
        citizens.iter().filter(|citizen| citizen.home_location == *house).count() as i32
    }

    fn find_office_occupants(citizens: &[&Citizen], office: &Area) -> i32 {
        citizens.iter().filter(|citizen| citizen.work_location == *office).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geography::define_geography;

    #[test]
    fn should_generate_population() {
        let mut rng = RandomWrapper::new();

        let grid = define_geography(100);
        let housing_area = grid.housing_area;
        let transport_area = grid.transport_area;
        let work_area = grid.work_area;

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

    #[test]
    fn should_increase_hospital_size() {
        let mut grid = define_geography(100);

        grid.increase_hospital_size(120);

        assert_eq!(grid.hospital_area.start_offset, Point::new(70, 0));
        assert_eq!(grid.hospital_area.end_offset, Point::new(120, 120));
    }
}
