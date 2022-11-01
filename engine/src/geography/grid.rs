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

use common::config::{AutoPopulation, CsvPopulation, StartingInfections, TravelPlanConfig};
use common::models::custom_types::{CoOrdinate, Count, Size};
use common::utils::RandomWrapper;
use plotters::prelude::*;
use std::collections::HashMap;
use std::fs::File;

use crate::citizen;
use crate::citizen::{Citizen, CitizensData, PopulationRecord};
use crate::geography::{Area, Point};
use crate::models::constants;

#[derive(Serialize, Clone)]
pub struct Grid {
    pub grid_size: Size,
    pub housing_area: Area,
    pub work_area: Area,
    pub transport_area: Area,
    pub hospital_area: Area,
    pub houses: Vec<Area>,
    pub offices: Vec<Area>,
    //Occupancy based on home and work locations - updated when travellers arrive/depart
    #[serde(skip_serializing)]
    pub houses_occupancy: HashMap<Area, u32>,
    #[serde(skip_serializing)]
    pub offices_occupancy: HashMap<Area, u32>,
}

impl Grid {
    pub fn generate_population(
        &mut self,
        auto_pop: &AutoPopulation,
        start_infections: &StartingInfections,
        rng: &mut RandomWrapper,
        travel_plan_config: &Option<TravelPlanConfig>,
        region: String,
    ) -> (Vec<Point>, Vec<Citizen>) {
        debug!("Generating Population");
        let number_of_agents = auto_pop.number_of_agents;
        let working_percentage = auto_pop.working_percentage;
        let public_transport_percentage = auto_pop.public_transport_percentage;

        //        TODO: fix the hack
        let number_of_agents_using_public_transport =
            number_of_agents as f64 * (public_transport_percentage + 0.1) * (working_percentage + 0.1);
        let transport_locations = self.transport_area.random_points(number_of_agents_using_public_transport.ceil() as usize, rng);
        debug!("transport locations: {}", transport_locations.len());
        debug!("Finished generating transport locations");

        let ctz_data = CitizensData::new(
            region.clone(),
            number_of_agents,
            &self.houses,
            &self.offices,
            &transport_locations,
            public_transport_percentage,
            working_percentage,
            start_infections,
        );

        let agent_list = citizen::citizen_factory(ctz_data, travel_plan_config, rng);
        // info!("agent list - {:?} ", agent_list);
        debug!("Finished creating agent list");

        let (home_loc, agents_in_order) = self.set_start_locations_and_occupancies(rng, &agent_list, &region);

        self.draw(&home_loc, &self.houses, &self.offices);
        (home_loc, agents_in_order)
    }

    fn set_start_locations_and_occupancies(
        &mut self,
        rng: &mut RandomWrapper,
        agent_list: &Vec<Citizen>,
        region_name: &String,
    ) -> (Vec<Point>, Vec<Citizen>) {
        let mut home_loc: Vec<Point> = Vec::new();
        let agents_by_home_locations = Grid::group_agents_by_home_locations(agent_list);
        let house_capacity = constants::HOME_SIZE * constants::HOME_SIZE;
        debug!("Finished grouping agents by home locations");
        let mut agents_in_order: Vec<Citizen> = Vec::with_capacity(agent_list.len());
        for (home, agents) in agents_by_home_locations {
            trace!("home: {:?} {:?}", home.start_offset, home.end_offset);
            trace!("agents in home: {:?}", agents.len());

            if agents.len() as Count > house_capacity {
                panic!("There are {} agents assigned to a house, but house capacity is {}", agents.len(), house_capacity)
            }

            let mut random_points_within_home = home.random_points(agents.len(), rng);
            self.houses_occupancy.insert(home.clone(), agents.len() as u32);

            for agent in agents {
                agents_in_order.push(agent.clone());
            }
            home_loc.append(&mut random_points_within_home);
        }
        debug!("Assigned starting location to agents");
        self.offices_occupancy = self.group_office_locations_by_occupancy(agents_in_order.as_slice(), region_name);
        // info!("offices occupancy - {:?}", self.offices_occupancy);
        (home_loc, agents_in_order)
    }

    pub fn group_agents_by_home_locations(agent_list: &[Citizen]) -> HashMap<&Area, Vec<&Citizen>> {
        let mut agents_by_home_locations: HashMap<&Area, Vec<&Citizen>> = HashMap::new();
        agent_list.iter().for_each(|agent| match agents_by_home_locations.get(&agent.home_location) {
            None => {
                agents_by_home_locations.insert(&agent.home_location, vec![agent]);
            }
            Some(citizens) => {
                let mut updated_citizens = citizens.clone();
                updated_citizens.push(agent);
                agents_by_home_locations.insert(&agent.home_location, updated_citizens);
            }
        });
        agents_by_home_locations
    }

    fn draw(&self, home_locations: &Vec<Point>, homes: &Vec<Area>, offices: &Vec<Area>) {
        let mut draw_backend = BitMapBackend::new("grid.png", (self.grid_size as u32, self.grid_size as u32));
        Grid::draw_rect(&mut draw_backend, &self.housing_area, &YELLOW);
        Grid::draw_rect(&mut draw_backend, &self.transport_area, &RGBColor(121, 121, 121));
        Grid::draw_rect(&mut draw_backend, &self.work_area, &BLUE);
        Grid::draw_rect(&mut draw_backend, &self.hospital_area, &RED);
        for home in homes {
            Grid::draw_rect(&mut draw_backend, home, &RGBColor(204, 153, 0));
        }
        for office in offices {
            Grid::draw_rect(&mut draw_backend, office, &RGBColor(51, 153, 255));
        }
        for home in home_locations {
            draw_backend.draw_pixel((home.x as i32, home.y as i32), BLACK.to_backend_color()).unwrap();
        }
    }

    fn draw_rect(svg: &mut impl DrawingBackend, area: &Area, style: &RGBColor) {
        svg.draw_rect(
            (area.start_offset.x as i32, area.start_offset.y as i32),
            (area.end_offset.x as i32, area.end_offset.y as i32),
            style,
            true,
        )
        .unwrap();
    }

    pub fn read_population(
        &mut self,
        csv_pop: &CsvPopulation,
        starting_infections: &StartingInfections,
        rng: &mut RandomWrapper,
        region_name: &String,
    ) -> (Vec<Point>, Vec<Citizen>) {
        let file = File::open(&csv_pop.file).expect("Could not read population file");
        let mut rdr = csv::Reader::from_reader(file);
        let mut homes_iter = self.houses.iter().cycle();
        let mut offices_iter = self.offices.iter().cycle();

        let mut citizens = Vec::new();
        for result in rdr.deserialize() {
            let record: PopulationRecord = result.expect("Could not deserialize population line");

            //TODO seems like transport point isn't being used on the routine() function
            let home = homes_iter.next().unwrap();
            let office = offices_iter.next().unwrap();
            let citizen = Citizen::from_record(record, home.clone(), office.clone(), home.get_random_point(rng), rng);
            citizens.push(citizen);
        }
        let house_capacity = (constants::HOME_SIZE * constants::HOME_SIZE) as usize;
        if citizens.len() > house_capacity * self.houses.len() {
            panic!(
                "Cannot accommodate citizens into homes! There are {} citizens, but {} home points",
                citizens.len(),
                house_capacity * self.houses.len()
            );
        }

        let (home_loc, mut agents_in_order) = self.set_start_locations_and_occupancies(rng, &citizens, region_name);
        citizen::set_starting_infections(&mut agents_in_order, starting_infections, rng);

        self.draw(&home_loc, &self.houses, &self.offices);
        (home_loc, agents_in_order)
    }

    pub fn increase_hospital_size(&mut self, grid_size: Size, sim_id: String) {
        let start_offset = self.hospital_area.start_offset;
        let end_offset = Point::new(grid_size as CoOrdinate, grid_size as CoOrdinate);

        self.hospital_area = Area::new(&sim_id, start_offset, end_offset)
    }

    pub fn resize_hospital(
        &mut self,
        number_of_agents: i32,
        hospital_staff_percentage: f64,
        hospital_beds_percentage: f64,
        sim_id: String,
    ) {
        let hospital_bed_count = (number_of_agents as f64 * hospital_beds_percentage
            + number_of_agents as f64 * hospital_staff_percentage)
            .ceil() as Count;

        if hospital_bed_count <= self.hospital_area.get_number_of_cells() {
            let hospital_end_y: CoOrdinate =
                (hospital_bed_count / (self.hospital_area.end_offset.x - self.hospital_area.start_offset.x) as u32) as CoOrdinate;
            self.hospital_area =
                Area::new(&sim_id, self.hospital_area.start_offset, Point::new(self.hospital_area.end_offset.x, hospital_end_y));
            info!("Hospital capacity {}: ", hospital_bed_count);
        }
    }

    pub fn group_office_locations_by_occupancy(&self, citizens: &[Citizen], region_name: &String) -> HashMap<Area, u32> {
        let mut occupancy = HashMap::new();
        self.offices.iter().for_each(|office| {
            occupancy.insert(office.clone(), 0);
        });
        citizens.iter().filter(|citizen| citizen.is_working() && citizen.work_location.location_id == *region_name).for_each(
            |worker| {
                let office = worker.work_location.clone();
                *occupancy.get_mut(&office).expect("Unknown office! Doesn't exist in grid") += 1;
            },
        );
        occupancy
    }

    pub fn choose_house_with_free_space(&self, _rng: &mut RandomWrapper) -> Area {
        let house_capacity = constants::HOME_SIZE * constants::HOME_SIZE;
        self.houses_occupancy
            .iter()
            .find(|(_house, occupants)| **occupants < house_capacity)
            .expect("Couldn't find any house with free space!")
            .0
            .clone()
    }

    pub fn choose_office_with_free_space(&self, _rng: &mut RandomWrapper) -> Area {
        let office_capacity = constants::OFFICE_SIZE * constants::OFFICE_SIZE;
        self.offices_occupancy
            .iter()
            .find(|(_office, occupants)| **occupants < office_capacity)
            .expect("Couldn't find any offices with free space!")
            .0
            .clone()
    }

    pub fn add_house_occupant(&mut self, house: &Area) {
        *self.houses_occupancy.get_mut(house).expect("Could not find house!") += 1;
    }

    pub fn add_office_occupant(&mut self, office: &Area) {
        *self.offices_occupancy.get_mut(office).expect("Could not find office!") += 1;
    }

    pub fn remove_house_occupant(&mut self, house: &Area) {
        *self.houses_occupancy.get_mut(house).expect("Could not find house!") -= 1;
    }

    pub fn remove_office_occupant(&mut self, office: &Area) {
        *self.offices_occupancy.get_mut(office).expect("Could not find office!") -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geography::define_geography;

    #[test]
    fn should_generate_population() {
        let mut rng = RandomWrapper::new();

        let mut grid = define_geography(100, "engine1".to_string());
        let housing_area = grid.housing_area.clone();
        let transport_area = grid.transport_area.clone();
        let work_area = grid.work_area.clone();

        let pop = AutoPopulation { number_of_agents: 10, public_transport_percentage: 0.2, working_percentage: 0.2 };
        let start_infections = StartingInfections::new(0, 0, 0, 1);
        let (home_locations, agent_list) =
            grid.generate_population(&pop, &start_infections, &mut rng, &None, "engine1".to_string());

        assert_eq!(home_locations.len(), 10);
        assert_eq!(agent_list.len(), 10);

        for agent in agent_list {
            assert!(housing_area.contains(&agent.home_location.start_offset));
            assert!(
                work_area.contains(&agent.work_location.end_offset) || housing_area.contains(&agent.home_location.start_offset)
            ); //for citizens that are not working
            assert!(transport_area.contains(&agent.transport_location) || housing_area.contains(&agent.transport_location))
            //for citizens that aren't using public transport
        }
    }

    #[test]
    fn should_increase_hospital_size() {
        let mut grid = define_geography(100, "engine1".to_string());

        grid.increase_hospital_size(120, "engine1".to_string());

        assert_eq!(grid.hospital_area.start_offset, Point::new(80, 0));
        assert_eq!(grid.hospital_area.end_offset, Point::new(120, 120));
    }

    #[test]
    fn grid_should_be_serializable_and_should_not_serialize_skipped_keys() {
        let grid: Grid = define_geography(75, "engine1".to_string());

        let grid_message = serde_json::to_value(&grid).unwrap();

        let message = grid_message.as_object().unwrap();
        let keys = message.keys();
        assert_eq!(keys.len(), 7);
        assert!(message.contains_key("grid_size"));
        assert!(message.contains_key("housing_area"));
        assert!(message.contains_key("work_area"));
        assert!(message.contains_key("transport_area"));
        assert!(message.contains_key("hospital_area"));
        assert!(message.contains_key("houses"));
        assert!(message.contains_key("offices"));
    }

    #[test]
    fn should_resize_hospital() {
        let mut grid = define_geography(100, "engine1".to_string());
        grid.resize_hospital(1000, 0.02, 0.01, "engine1".to_string());

        assert_eq!(grid.hospital_area.start_offset, Point::new(80, 0));
        assert_eq!(grid.hospital_area.end_offset, Point::new(89, 3));
    }

    #[test]
    fn should_not_resize_hospital_if_population_is_too_high() {
        let mut grid = define_geography(100, "engine1".to_string());
        grid.resize_hospital(50000, 0.02, 0.01, "engine1".to_string());

        assert_eq!(grid.hospital_area.start_offset, Point::new(80, 0));
        assert_eq!(grid.hospital_area.end_offset, Point::new(89, 100));
    }
}
