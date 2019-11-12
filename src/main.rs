mod geography;
mod agent;
mod simulation;

fn main() {
    const GRID_SIZE:i32 = 3;
    const NUMBER_OF_AGENTS:i32 = 3;
    const SIMULATION_LIFE_TIME:i32 = 5;

    let mut epidemiology = simulation::Epidemiology::new(GRID_SIZE, NUMBER_OF_AGENTS);
    epidemiology.run(SIMULATION_LIFE_TIME);
}