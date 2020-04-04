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

use geojson::{Feature, GeoJson};
use std::collections::HashMap;

pub const BUILDING:&str = &"building";

struct Buildings {
    building_type: String,
    allowed_subtypes: Vec<String>,
    features: Vec<Feature>
}

impl Buildings {

    pub fn new(building_type: String, allowed_subtypes: Vec<String>) -> Buildings {
        let features = Vec::new();
        Buildings {building_type, allowed_subtypes, features}
    }

    pub fn get_building_type(&self) -> &String{
        &self.building_type
    }

    pub fn buildings_factory(geo_json: &GeoJson, buildings_type_map: HashMap<String, Vec<String>>) -> HashMap<String, Buildings>{
        let mut feature_types_map: HashMap<String, Buildings> = HashMap::new();

        for (key, value) in buildings_type_map.iter(){
            feature_types_map.insert(key.clone(), Buildings::new(key.to_string(), value.clone()));
        }

        match *geo_json {
            GeoJson::FeatureCollection(ref ctn) => for feature in &ctn.features {
                if let Some(ref properties) = feature.properties {
                    let building_sub_type = properties.get(BUILDING).unwrap().as_str().unwrap();
                    let building_type = Buildings::get_key_based_on_sub_type(building_sub_type, &buildings_type_map);
                    let buildings_option = feature_types_map.get_mut(&building_type);

                    match buildings_option {
                        Some(x) => x.features.push(feature.clone()),
                        None => println!("{} Key not found", building_sub_type)
                    }
                }
            },
            _ => {}
        }

        feature_types_map
    }

    fn get_key_based_on_sub_type(building_sub_type: &str, map: &HashMap<String, Vec<String>>) -> String{
        for (key, values) in map.iter(){
            if values.contains(&building_sub_type.to_string()) {
                return key.to_string();
            }
        }
        "".to_string()
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::geojson_service::GeoJsonService;

    pub fn before_each() -> GeoJson{
        let file_name = "./test/resources/test_multipolygons.geojson";
        let geo_json_service = GeoJsonService::new(file_name.to_string());
        geo_json_service.geo_json
    }

    #[test]
    fn should_get_building_type(){
        let residential = Buildings::new("residential".to_string(), vec![String::from("residential"), String::from("apartments"), String::from("yes")]);

        assert_eq!(residential.get_building_type(), "residential");
    }

    #[test]
    fn should_generate_buildings(){
        let geo_json = before_each();

        let mut buildings_type: HashMap<String, Vec<String>> = HashMap::new();
        buildings_type.insert("residential".to_string(), vec![String::from("residential"), String::from("apartments"), String::from("yes")]);
        buildings_type.insert("commercial".to_string(), vec![String::from("industrial"), String::from("commercial")]);

        let expected_buildings = Buildings::buildings_factory(&geo_json, buildings_type);

        assert_eq!(expected_buildings.get("residential").unwrap().get_building_type(), "residential");
        assert_eq!(expected_buildings.get("residential").unwrap().features.len(), 3);
        assert_eq!(expected_buildings.get("commercial").unwrap().features.len(), 2);
    }
}
