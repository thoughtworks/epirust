use crate::constants;

mod area;
mod grid;
mod point;

pub use self::area::Area;
pub use self::grid::Grid;
pub use self::point::Point;

pub fn define_geography(grid_size: i32) -> Grid {
    let bound = grid_size - 1;
    let x_offset_for_home = (bound as f32 * constants::HOUSE_AREA_RELATIVE_SIZE).ceil() as i32 - 1;
    let x_offset_for_transport = x_offset_for_home + (bound as f32 * constants::TRANSPORT_AREA_RELATIVE_SIZE).ceil() as i32;
    let x_offset_for_work_area = x_offset_for_transport + (bound as f32 * constants::WORK_AREA_RELATIVE_SIZE).ceil() as i32;
    let x_offset_for_hospital = x_offset_for_work_area + (bound as f32 * constants::INITIAL_HOSPITAL_RELATIVE_SIZE).ceil() as i32;

    let housing_area = Area::new(Point::new(0, 0), Point::new(x_offset_for_home, bound));
    let transport_area = Area::new(Point::new(x_offset_for_home + 1, 0), Point::new(x_offset_for_transport, bound));
    let work_area = Area::new(Point::new(x_offset_for_transport + 1, 0), Point::new(x_offset_for_work_area, bound));
    let hospital_area = Area::new(Point::new(x_offset_for_work_area + 1, 0), Point::new(x_offset_for_hospital, bound));
    Grid { grid_size, housing_area, transport_area, hospital_area, work_area }
}

