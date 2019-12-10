mod constants;

mod agent;
mod epidemiology_simulation;
mod allocation_map;
mod geography;
mod disease;
mod utils;

fn main() {
    const GRID_SIZE:i32 = 120;
    const NUMBER_OF_AGENTS:i32 = 3000;
    const SIMULATION_LIFE_TIME:i32 = 100;
    const VACCINATION_TIME:i32 = 500;
    const VACCINATION_PERCENTAGE:f64 = 0.2;

    let mut epidemiology = epidemiology_simulation::Epidemiology::new(GRID_SIZE, NUMBER_OF_AGENTS);
    epidemiology.run(SIMULATION_LIFE_TIME, VACCINATION_TIME, VACCINATION_PERCENTAGE);
}