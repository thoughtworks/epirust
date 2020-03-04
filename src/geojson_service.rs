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
