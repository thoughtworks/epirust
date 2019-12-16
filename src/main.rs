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
    println!("Executing for 100 agents");
    sim100();
    println!("Executing for 1000 agents");
    sim1000();
    println!("Executing for 10000 agents");
    sim10000();
    println!("Executing for 100000 agents");
    sim100_000();
    println!("Executing for 100000 agents");
    sim1_000_000();
    println!("Done");
}

fn sim100(){
    const GRID_SIZE:i32 = 25;
    const NUMBER_OF_AGENTS:i32 = 100;
    const SIMULATION_LIFE_TIME:i32 = 10000;
    const VACCINATION_TIME:i32 = 5000;
    const VACCINATION_PERCENTAGE:f64 = 0.2;
    const PUBLIC_TRANSPORT_PERCENTAGE: f64 = 0.2;
    const WORKING_PERCENTAGE:f64 = 0.7;
    const OUTPUT_FILE_NAME:&str = "simulation_100.csv";

    let mut epidemiology = epidemiology_simulation::Epidemiology::new(GRID_SIZE, NUMBER_OF_AGENTS, PUBLIC_TRANSPORT_PERCENTAGE, WORKING_PERCENTAGE);
    epidemiology.run(SIMULATION_LIFE_TIME, VACCINATION_TIME, VACCINATION_PERCENTAGE, OUTPUT_FILE_NAME);
}

fn sim1000(){
    const GRID_SIZE:i32 = 80;
    const NUMBER_OF_AGENTS:i32 = 1000;
    const SIMULATION_LIFE_TIME:i32 = 10000;
    const VACCINATION_TIME:i32 = 5000;
    const VACCINATION_PERCENTAGE:f64 = 0.2;
    const PUBLIC_TRANSPORT_PERCENTAGE: f64 = 0.2;
    const WORKING_PERCENTAGE:f64 = 0.7;
    const OUTPUT_FILE_NAME:&str = "simulation_1000.csv";

    let mut epidemiology = epidemiology_simulation::Epidemiology::new(GRID_SIZE, NUMBER_OF_AGENTS, PUBLIC_TRANSPORT_PERCENTAGE, WORKING_PERCENTAGE);
    epidemiology.run(SIMULATION_LIFE_TIME, VACCINATION_TIME, VACCINATION_PERCENTAGE, OUTPUT_FILE_NAME);
}

fn sim10000(){
    const GRID_SIZE:i32 = 250;
    const NUMBER_OF_AGENTS:i32 = 10000;
    const SIMULATION_LIFE_TIME:i32 = 10000;
    const VACCINATION_TIME:i32 = 5000;
    const VACCINATION_PERCENTAGE:f64 = 0.2;
    const PUBLIC_TRANSPORT_PERCENTAGE: f64 = 0.2;
    const WORKING_PERCENTAGE:f64 = 0.7;
    const OUTPUT_FILE_NAME:&str = "simulation_10000.csv";

    let mut epidemiology = epidemiology_simulation::Epidemiology::new(GRID_SIZE, NUMBER_OF_AGENTS, PUBLIC_TRANSPORT_PERCENTAGE, WORKING_PERCENTAGE);
    epidemiology.run(SIMULATION_LIFE_TIME, VACCINATION_TIME, VACCINATION_PERCENTAGE, OUTPUT_FILE_NAME);
}

fn sim100_000(){
    const GRID_SIZE:i32 = 800;
    const NUMBER_OF_AGENTS:i32 = 100_000;
    const SIMULATION_LIFE_TIME:i32 = 10000;
    const VACCINATION_TIME:i32 = 5000;
    const VACCINATION_PERCENTAGE:f64 = 0.2;
    const PUBLIC_TRANSPORT_PERCENTAGE: f64 = 0.2;
    const WORKING_PERCENTAGE:f64 = 0.7;
    const OUTPUT_FILE_NAME:&str = "simulation_100_000.csv";

    let mut epidemiology = epidemiology_simulation::Epidemiology::new(GRID_SIZE, NUMBER_OF_AGENTS, PUBLIC_TRANSPORT_PERCENTAGE, WORKING_PERCENTAGE);
    epidemiology.run(SIMULATION_LIFE_TIME, VACCINATION_TIME, VACCINATION_PERCENTAGE, OUTPUT_FILE_NAME);
}

fn sim1_000_000(){
    const GRID_SIZE:i32 = 2500;
    const NUMBER_OF_AGENTS:i32 = 1_000_000;
    const SIMULATION_LIFE_TIME:i32 = 10000;
    const VACCINATION_TIME:i32 = 5000;
    const VACCINATION_PERCENTAGE:f64 = 0.2;
    const PUBLIC_TRANSPORT_PERCENTAGE: f64 = 0.2;
    const WORKING_PERCENTAGE:f64 = 0.7;
    const OUTPUT_FILE_NAME:&str = "simulation_1_000_000.csv";

    let mut epidemiology = epidemiology_simulation::Epidemiology::new(GRID_SIZE, NUMBER_OF_AGENTS, PUBLIC_TRANSPORT_PERCENTAGE, WORKING_PERCENTAGE);
    epidemiology.run(SIMULATION_LIFE_TIME, VACCINATION_TIME, VACCINATION_PERCENTAGE, OUTPUT_FILE_NAME);
}