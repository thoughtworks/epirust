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

use crate::constants;

mod area;
mod grid;
mod point;

pub use self::area::Area;
pub use self::grid::Grid;
pub use self::point::Point;
use std::collections::HashMap;

pub fn define_geography(grid_size: i32) -> Grid {
    let home_width = (grid_size as f32 * constants::HOUSE_AREA_RELATIVE_SIZE).ceil() as i32;
    let transport_start = home_width;
    let transport_end = home_width + (grid_size as f32 * constants::TRANSPORT_AREA_RELATIVE_SIZE).ceil() as i32;
    let work_area_start = transport_end;
    let work_area_end = transport_end + (grid_size as f32 * constants::WORK_AREA_RELATIVE_SIZE).ceil() as i32;
    let hospital_start = work_area_end;
    let hospital_end = work_area_end + (grid_size as f32 * constants::INITIAL_HOSPITAL_RELATIVE_SIZE).ceil() as i32;

    let housing_area = Area::new(Point::new(0, 0), Point::new(home_width - 1, grid_size));
    let transport_area = Area::new(Point::new(transport_start, 0), Point::new(transport_end - 1, grid_size));
    let work_area = Area::new(Point::new(work_area_start, 0), Point::new(work_area_end - 1, grid_size));
    // let hospital_area = Area::new(Point::new(hospital_start, 0), Point::new(hospital_start + 1, grid_size)); // Reduced hospital size with hospital staff being 0.005
    let hospital_area = Area::new(Point::new(hospital_start, 0), Point::new(hospital_start - 1, grid_size)); // No intervention with hospital staff being 0
    // let hospital_area = Area::new(Point::new(hospital_start, 0), Point::new(hospital_end - 1, grid_size));

    let houses = area::area_factory(housing_area.start_offset, housing_area.end_offset, constants::HOME_SIZE);
    let offices = area::area_factory(work_area.start_offset, work_area.end_offset, constants::OFFICE_SIZE);

    Grid {
        grid_size,
        housing_area,
        transport_area,
        hospital_area,
        work_area,
        houses,
        offices,
        houses_occupancy: HashMap::new(),
        offices_occupancy: HashMap::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_define_geography(){
        let grid = define_geography(10);
        assert_eq!(grid.housing_area.start_offset, Point::new(0,0));
        assert_eq!(grid.housing_area.end_offset, Point::new(3,10));

        assert_eq!(grid.transport_area.start_offset, Point::new(4,0));
        assert_eq!(grid.transport_area.end_offset, Point::new(4,10));

        assert_eq!(grid.work_area.start_offset, Point::new(5,0));
        assert_eq!(grid.work_area.end_offset, Point::new(6,10));

        assert_eq!(grid.hospital_area.start_offset, Point::new(7,0));
        assert_eq!(grid.hospital_area.end_offset, Point::new(7,10));
    }
}
