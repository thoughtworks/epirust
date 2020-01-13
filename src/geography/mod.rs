use crate::geography::point::Point;
use crate::agent;
use crate::geography::housing_area::HousingArea;
use crate::geography::work_area::WorkArea;
use crate::geography::transport_area::TransportArea;
use crate::geography::hospital::Hospital;
use crate::constants;

pub mod point;
pub mod housing_area;
pub mod work_area;
pub mod transport_area;
pub mod hospital;

pub trait Area{
    fn get_dimensions(&self, agent:agent::Citizen) -> Vec<Point>;
}

pub struct Grid{
    pub housing_area: HousingArea,
    pub work_area: WorkArea,
    pub transport_area: TransportArea,
    pub hospital: Hospital,
}

impl Grid{
    pub fn generate_population(&self, number_of_agents: i32, public_transport_percentage: f64, working_percentage: f64)
                               -> (Vec<Point>, Vec<agent::Citizen>) {
        let home_locations = point::point_factory(self.housing_area.start_offset,
                                                  self.housing_area.end_offset, number_of_agents);
        let scaling_factor = self.housing_area.end_offset.x + self.transport_area.end_offset.x;
        let work_locations = home_locations.iter()
            .map(|x| *x + point::Point::new(scaling_factor, 0)).collect();
//        TODO: fix the hack
        let number_of_agents_using_public_transport = number_of_agents as f64 * (public_transport_percentage + 0.1) * (working_percentage + 0.1);
        let transport_locations = point::point_factory(self.transport_area.start_offset, self.transport_area.end_offset, number_of_agents_using_public_transport as i32);
        let agent_list = agent::citizen_factory(&home_locations, &work_locations, &transport_locations, public_transport_percentage, working_percentage);
        (home_locations, agent_list)
    }
}

pub fn define_geography(grid_size: i32) -> Grid {
    let bound = grid_size - 1;
    let x_offset_for_home = (bound as f32 * constants::HOUSE_AREA_RELATIVE_SIZE).ceil() as i32 - 1;
    let x_offset_for_transport = x_offset_for_home + (bound as f32 * constants::TRANSPORT_AREA_RELATIVE_SIZE).ceil() as i32;
    let x_offset_for_hospital = x_offset_for_transport + (bound as f32 * constants::HOSPITAL_RELATIVE_SIZE).ceil() as i32;

    let housing_area = HousingArea::new(Point::new(0, 0), Point::new(x_offset_for_home, bound));
    let transport_area = TransportArea::new(Point::new(x_offset_for_home + 1, 0), Point::new(x_offset_for_transport, bound));
    let hospital = Hospital::new(Point::new(x_offset_for_transport + 1, 0), Point::new(x_offset_for_hospital, bound));
    let work_area = WorkArea::new(Point::new(x_offset_for_hospital + 1, 0), Point::new(bound, bound));
    Grid { housing_area, transport_area, hospital, work_area }
}