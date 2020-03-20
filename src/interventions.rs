use crate::config::Config;

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub enum Intervention {
    Vaccinate(Vaccinate),
    Lockdown(Lockdown),
    BuildNewHospital(BuildNewHospital)
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

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
pub struct BuildNewHospital {
    pub at_hour: i32,
    pub new_scale_factor: i32
}

impl Intervention {
    pub fn get_hospital_intervention(config: &Config) -> Option<BuildNewHospital> {
        return config.get_interventions().iter().filter_map(|i| {
            match i {
                Intervention::BuildNewHospital(x) => Some(x),
                _ => None
            }
        }).next().copied();
    }
}