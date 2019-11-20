mod agent;
mod epidemiology_simulation;
mod allocation_map;
mod epidemiology_geography;

fn main() {
    const GRID_SIZE:i32 = 3;
    const NUMBER_OF_AGENTS:i32 = 3;
    const SIMULATION_LIFE_TIME:i32 = 5;

    let mut epidemiology = epidemiology_simulation::Epidemiology::new(GRID_SIZE, NUMBER_OF_AGENTS);
    epidemiology.run(SIMULATION_LIFE_TIME);
}