use std::error::Error;
use std::fs::File;

use crate::disease::{Disease, DiseaseOverride};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    population: Population,
    disease: Disease,
    #[serde(default)]
    disease_overrides: Vec<DiseaseOverride>,
    grid: i32,
    hours: i32,
    interventions: Vec<Intervention>,
    output_file: Option<String>,
}

impl Config {
    pub fn get_disease(&self) -> Disease {
        self.disease
    }

    pub fn get_population(&self) -> Population {
        self.population.clone()
    }

    pub fn get_disease_overrides(&self) -> Vec<DiseaseOverride> {
        self.disease_overrides.clone()
    }

    pub fn get_grid(&self) -> i32 {
        self.grid
    }

    pub fn get_hours(&self) -> i32 {
        self.hours
    }

    pub fn get_interventions(&self) -> Vec<Intervention> {
        self.interventions.clone()
    }

    pub fn get_output_file(&self) -> Option<String> {
        self.output_file.clone()
    }

    pub fn new(population: Population, disease: Disease, disease_overrides: Vec<DiseaseOverride>,
               grid: i32, hours: i32, interventions: Vec<Intervention>, output_file: Option<String>)
        -> Config {
        Config {
            population,
            disease,
            disease_overrides,
            grid,
            hours,
            interventions,
            output_file,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum Population {
    Csv(CsvPopulation),
    Auto(AutoPopulation),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CsvPopulation {
    pub file: String,
    pub cols: Vec<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub struct AutoPopulation {
    pub number_of_agents: i32,
    pub public_transport_percentage: f64,
    pub working_percentage: f64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub enum Intervention {
    Vaccinate(Vaccinate),
    Lockdown(Lockdown),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub struct Vaccinate {
    pub at_hour: i32,
    pub percent: f64,
}

impl Vaccinate {
    pub fn new(at_hour: i32, percent: f64) -> Vaccinate {
        Vaccinate { at_hour, percent }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub struct Lockdown {
    at_hour: i32,
}

impl Lockdown {
    pub fn new(at_hour: i32) -> Lockdown {
        Lockdown { at_hour }
    }
}

pub fn read(filename: String) -> Result<Config, Box<dyn Error>> {
    let reader = File::open(filename)?;
    let v: Config = serde_json::from_reader(reader)?;
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_read_pune_config() {
        let read_config = read(String::from("config/pune.json")).unwrap();

        let vaccinate = Vaccinate::new(5000, 0.2);
        let disease_override = DiseaseOverride::new(
            String::from("age"),
            vec!["60-64".to_string(), "65-69".to_string(), "70-74".to_string(), "75-79".to_string(), "80+".to_string()],
            Disease::new(5, 20, 40, 0.025, 0.25, 0.2));

        let population = Population::Csv(CsvPopulation {
            file: "config/pune_population.csv".to_string(),
            cols: vec![String::from("age"), String::from("sex"), String::from("working"),
                       String::from("pub_transport")],
        });

        let expected_config = Config {
            population,
            disease: Disease::new(5, 20, 40, 0.025, 0.25, 0.035),
            disease_overrides: vec![disease_override],
            grid: 5660,
            hours: 10000,
            interventions: vec![Intervention::Vaccinate(vaccinate)],
            output_file: None,
        };

        assert_eq!(expected_config, read_config);
    }

    #[test]
    fn should_read_default_config() {
        let read_config = read(String::from("config/default.json")).unwrap();

        let vaccinate = Vaccinate::new(5000, 0.2);

        let population = Population::Auto(AutoPopulation {
            number_of_agents: 10000,
            public_transport_percentage: 0.2,
            working_percentage: 0.7,
        });

        let expected_config = Config {
            population,
            disease: Disease::new(5, 20, 40, 0.025, 0.25, 0.035),
            disease_overrides: vec![],
            grid: 250,
            hours: 10000,
            interventions: vec![Intervention::Vaccinate(vaccinate)],
            output_file: Some("simulation_default_config".to_string()),
        };

        assert_eq!(expected_config, read_config);
    }
}
