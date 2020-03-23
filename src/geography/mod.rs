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
