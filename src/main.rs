#[macro_use]
extern crate serde_derive;

use std::env;

mod constants;

mod agent;
mod epidemiology_simulation;
mod allocation_map;
mod geography;
mod disease;
mod csv_service;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut count = 10000;
    if args.len() > 1 {
        count = args[1].parse().unwrap_or(10000);
    }

    if count <= 100 {
        println!("Executing for 100 agents");
        sim100();
    } else if count <= 1000 {
        println!("Executing for 1000 agents");
        sim1000();
    } else if count <= 10000 {
        println!("Executing for 10,000 agents");
        sim10000();
    } else if count <= 100000 {
        println!("Executing for 100,000 agents");
        sim100_000();
    } else if count <= 1_000_000 {
        println!("Executing for 1,000,000 agents");
        sim1_000_000();
    } else if count <= 2_000_000 {
        println!("Executing for 2,000,000 agents");
        sim2_000_000();
    } else {
        println!("Executing for 5,000,000 agents");
        sim5_000_000();
    }
    println!("Done");
}

fn sim100() {
    const GRID_SIZE: i32 = 25;
    const NUMBER_OF_AGENTS: i32 = 100;
    const SIMULATION_LIFE_TIME: i32 = 10000;
    const VACCINATION_TIME: i32 = 5000;
    const VACCINATION_PERCENTAGE: f64 = 0.2;
    const PUBLIC_TRANSPORT_PERCENTAGE: f64 = 0.2;
    const WORKING_PERCENTAGE: f64 = 0.7;
    const OUTPUT_FILE_NAME: &str = "simulation_100.csv";

    let mut epidemiology = epidemiology_simulation::Epidemiology::new(GRID_SIZE, NUMBER_OF_AGENTS, PUBLIC_TRANSPORT_PERCENTAGE, WORKING_PERCENTAGE);
    epidemiology.run(SIMULATION_LIFE_TIME, VACCINATION_TIME, VACCINATION_PERCENTAGE, OUTPUT_FILE_NAME);
}

fn sim1000() {
    const GRID_SIZE: i32 = 80;
    const NUMBER_OF_AGENTS: i32 = 1000;
    const SIMULATION_LIFE_TIME: i32 = 10000;
    const VACCINATION_TIME: i32 = 5000;
    const VACCINATION_PERCENTAGE: f64 = 0.2;
    const PUBLIC_TRANSPORT_PERCENTAGE: f64 = 0.2;
    const WORKING_PERCENTAGE: f64 = 0.7;
    const OUTPUT_FILE_NAME: &str = "simulation_1000.csv";

    let mut epidemiology = epidemiology_simulation::Epidemiology::new(GRID_SIZE, NUMBER_OF_AGENTS, PUBLIC_TRANSPORT_PERCENTAGE, WORKING_PERCENTAGE);
    epidemiology.run(SIMULATION_LIFE_TIME, VACCINATION_TIME, VACCINATION_PERCENTAGE, OUTPUT_FILE_NAME);
}

fn sim10000() {
    const GRID_SIZE: i32 = 250;
    const NUMBER_OF_AGENTS: i32 = 10000;
    const SIMULATION_LIFE_TIME: i32 = 10000;
    const VACCINATION_TIME: i32 = 5000;
    const VACCINATION_PERCENTAGE: f64 = 0.2;
    const PUBLIC_TRANSPORT_PERCENTAGE: f64 = 0.2;
    const WORKING_PERCENTAGE: f64 = 0.7;
    const OUTPUT_FILE_NAME: &str = "simulation_10000.csv";

    let mut epidemiology = epidemiology_simulation::Epidemiology::new(GRID_SIZE, NUMBER_OF_AGENTS, PUBLIC_TRANSPORT_PERCENTAGE, WORKING_PERCENTAGE);
    epidemiology.run(SIMULATION_LIFE_TIME, VACCINATION_TIME, VACCINATION_PERCENTAGE, OUTPUT_FILE_NAME);
}

fn sim100_000() {
    const GRID_SIZE: i32 = 800;
    const NUMBER_OF_AGENTS: i32 = 100_000;
    const SIMULATION_LIFE_TIME: i32 = 10000;
    const VACCINATION_TIME: i32 = 5000;
    const VACCINATION_PERCENTAGE: f64 = 0.2;
    const PUBLIC_TRANSPORT_PERCENTAGE: f64 = 0.2;
    const WORKING_PERCENTAGE: f64 = 0.7;
    const OUTPUT_FILE_NAME: &str = "simulation_100_000.csv";

    let mut epidemiology = epidemiology_simulation::Epidemiology::new(GRID_SIZE, NUMBER_OF_AGENTS, PUBLIC_TRANSPORT_PERCENTAGE, WORKING_PERCENTAGE);
    epidemiology.run(SIMULATION_LIFE_TIME, VACCINATION_TIME, VACCINATION_PERCENTAGE, OUTPUT_FILE_NAME);
}

fn sim1_000_000() {
    const GRID_SIZE: i32 = 2500;
    const NUMBER_OF_AGENTS: i32 = 1_000_000;
    const SIMULATION_LIFE_TIME: i32 = 10000;
    const VACCINATION_TIME: i32 = 5000;
    const VACCINATION_PERCENTAGE: f64 = 0.2;
    const PUBLIC_TRANSPORT_PERCENTAGE: f64 = 0.2;
    const WORKING_PERCENTAGE: f64 = 0.7;
    const OUTPUT_FILE_NAME: &str = "simulation_1_000_000.csv";

    let mut epidemiology = epidemiology_simulation::Epidemiology::new(GRID_SIZE, NUMBER_OF_AGENTS, PUBLIC_TRANSPORT_PERCENTAGE, WORKING_PERCENTAGE);
    epidemiology.run(SIMULATION_LIFE_TIME, VACCINATION_TIME, VACCINATION_PERCENTAGE, OUTPUT_FILE_NAME);
}

fn sim2_000_000() {
    const GRID_SIZE: i32 = 3550;
    const NUMBER_OF_AGENTS: i32 = 2_000_000;
    const SIMULATION_LIFE_TIME: i32 = 10000;
    const VACCINATION_TIME: i32 = 5000;
    const VACCINATION_PERCENTAGE: f64 = 0.2;
    const PUBLIC_TRANSPORT_PERCENTAGE: f64 = 0.2;
    const WORKING_PERCENTAGE: f64 = 0.7;
    const OUTPUT_FILE_NAME: &str = "simulation_2_000_000.csv";

    let mut epidemiology = epidemiology_simulation::Epidemiology::new(GRID_SIZE, NUMBER_OF_AGENTS, PUBLIC_TRANSPORT_PERCENTAGE, WORKING_PERCENTAGE);
    epidemiology.run(SIMULATION_LIFE_TIME, VACCINATION_TIME, VACCINATION_PERCENTAGE, OUTPUT_FILE_NAME);
}


fn sim5_000_000() {
    const GRID_SIZE: i32 = 5660;
    const NUMBER_OF_AGENTS: i32 = 5_000_000;
    const SIMULATION_LIFE_TIME: i32 = 10000;
    const VACCINATION_TIME: i32 = 5000;
    const VACCINATION_PERCENTAGE: f64 = 0.2;
    const PUBLIC_TRANSPORT_PERCENTAGE: f64 = 0.2;
    const WORKING_PERCENTAGE: f64 = 0.7;
    const OUTPUT_FILE_NAME: &str = "simulation_5_000_000.csv";

    let mut epidemiology = epidemiology_simulation::Epidemiology::new(GRID_SIZE, NUMBER_OF_AGENTS, PUBLIC_TRANSPORT_PERCENTAGE, WORKING_PERCENTAGE);
    epidemiology.run(SIMULATION_LIFE_TIME, VACCINATION_TIME, VACCINATION_PERCENTAGE, OUTPUT_FILE_NAME);
}
