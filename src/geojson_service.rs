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

use geojson::GeoJson;
use std::fs;
use std::io::Read;

pub struct GeoJsonService{
    pub geo_json: GeoJson
}

impl GeoJsonService{

    pub fn new(file_name: String) -> GeoJsonService {
        let mut file = fs::File::open(file_name)
            .expect("file should open read only");

        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let geo_json = data.parse::<GeoJson>().unwrap();

        GeoJsonService{geo_json}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_read_geojson() {
        let file_name = "./test/resources/test_multipolygons.geojson";
        let geo_json_service = GeoJsonService::new(file_name.to_string());
        assert_eq!(geo_json_service.geo_json.to_string().len(), 1674);
    }
}
