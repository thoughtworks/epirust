#[macro_use]
extern crate serde_derive;

mod constants;

mod agent;
mod epidemiology_simulation;
mod allocation_map;
mod geography;
mod disease;
mod utils;
mod csv_service;

fn main() {
    const GRID_SIZE:i32 = 120;
    const NUMBER_OF_AGENTS:i32 = 3000;
    const SIMULATION_LIFE_TIME:i32 = 100;
    const VACCINATION_TIME:i32 = 500;
    const VACCINATION_PERCENTAGE:f64 = 0.2;
    const PUBLIC_TRANSPORT_PERCENTAGE: f64 = 0.3;
    const OUTPUT_FILE_NAME:&str = "simulation.csv";

    let mut epidemiology = epidemiology_simulation::Epidemiology::new(GRID_SIZE, NUMBER_OF_AGENTS, PUBLIC_TRANSPORT_PERCENTAGE);
    epidemiology.run(SIMULATION_LIFE_TIME, VACCINATION_TIME, VACCINATION_PERCENTAGE, OUTPUT_FILE_NAME);
}