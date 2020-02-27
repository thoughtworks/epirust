extern crate clap;
#[macro_use]
extern crate serde_derive;

use clap::{App, Arg, value_t};

mod constants;

mod agent;
mod epidemiology_simulation;
mod allocation_map;
mod geography;
mod disease;
mod csv_service;
mod random_wrapper;
mod disease_tracker;
mod events;
mod kafka_service;
mod population;

const VACCINATION_TIME: i32 = 5000;
const VACCINATION_PERCENTAGE: f64 = 0.2;
const PUBLIC_TRANSPORT_PERCENTAGE: f64 = 0.2;
const WORKING_PERCENTAGE: f64 = 0.7;

fn main() {
    let matches = App::new("EpiRust")
        .version("0.1")
        .about("Epidemiology Simulations in Rust")
        .arg(Arg::with_name("agents")
            .short("a")
            .long("agents")
            .value_name("NUMBER")
            .help("Number of agents to run the simulation with. Valid values are 100 to 5000000")
            .default_value("10000")
            .takes_value(true))
        .arg(Arg::with_name("grid")
            .short("g")
            .long("grid")
            .value_name("NUMBER")
            .help("The size of the Grid. E.g. entering '80' will create an 80x80 grid")
            .takes_value(true))
        .arg(Arg::with_name("hours")
            .short("hr")
            .long("hours")
            .value_name("NUMBER")
            .help("The total hours of the simulation")
            .default_value("10000")
            .takes_value(true))
        .arg(Arg::with_name("public_transport")
            .short("pub")
            .long("public_transport")
            .value_name("NUMBER")
            .help("The ratio of people using public transport (values between 0-1)")
            .default_value("0.2")
            .takes_value(true))
        .arg(Arg::with_name("working")
            .short("work")
            .long("working")
            .value_name("NUMBER")
            .help("The ratio of people working (values between 0-1)")
            .default_value("0.7")
            .takes_value(true))
        .arg(Arg::with_name("vaccinate_at")
            .long("vaccinate_at")
            .value_name("NUMBER")
            .help("The hour at which to start vaccination")
            .default_value("5000")
            .takes_value(true))
        .arg(Arg::with_name("vaccinate_ratio")
            .long("vaccinate_ratio")
            .value_name("NUMBER")
            .help("The ratio of people to be vaccinated (values between 0-1)")
            .default_value("0.2")
            .takes_value(true))
        .arg(Arg::with_name("disease")
            .long("disease")
            .short("d")
            .value_name("NAME")
            .help("One of the disease names configured in config/diseases.yaml (e.g. small_pox, sars, covid_19)")
            .default_value("small_pox")
            .takes_value(true))
        .get_matches();

    let count = value_t!(matches, "agents", i32).unwrap_or(10000);
    let simulation_hours = value_t!(matches, "hours", i32).unwrap_or(10000);
    let public_transport = value_t!(matches, "public_transport", f64).unwrap_or(PUBLIC_TRANSPORT_PERCENTAGE);
    let working = value_t!(matches, "working", f64).unwrap_or(WORKING_PERCENTAGE);
    let vaccinate_at = value_t!(matches, "vaccinate_at", i32).unwrap_or(VACCINATION_TIME);
    let vaccinate_ratio = value_t!(matches, "vaccinate_ratio", f64).unwrap_or(VACCINATION_PERCENTAGE);
    let disease_name = matches.value_of("disease").unwrap_or("small_pox");
    let output_file: &str;
    let grid_default: i32;

    if count <= 100 {
        println!("Executing for 100 agents");
        grid_default = 25;
        output_file = "simulation_100.csv"
    } else if count <= 1000 {
        println!("Executing for 1000 agents");
        grid_default = 80;
        output_file = "simulation_1000.csv";
    } else if count <= 10000 {
        println!("Executing for 10,000 agents");
        grid_default = 250;
        output_file = "simulation_10000.csv";
    } else if count <= 100_000 {
        println!("Executing for 100,000 agents");
        grid_default = 800;
        output_file = "simulation_100_000.csv";
    } else if count <= 1_000_000 {
        println!("Executing for 1,000,000 agents");
        grid_default = 2500;
        output_file = "simulation_1_000_000.csv";
    } else if count <= 2_000_000 {
        println!("Executing for 2,000,000 agents");
        grid_default = 3550;
        output_file = "simulation_2_000_000.csv"
    } else {
        println!("Executing for 5,000,000 agents");
        grid_default = 5660;
        output_file = "simulation_5_000_000.csv"
    }
    let grid = value_t!(matches, "grid", i32).unwrap_or(grid_default);
    start(disease_name, grid, count, simulation_hours, public_transport, working, vaccinate_at,
          vaccinate_ratio, output_file);
    println!("Done");
}

fn start(disease_name: &str, grid: i32, agents: i32, simulation_hrs: i32, transport: f64, working: f64,
         vaccinate_at: i32, vaccinate_ratio: f64, output_file: &str) {
    let mut epidemiology = epidemiology_simulation::Epidemiology::new(disease_name, grid, agents, transport, working);
    epidemiology.run(simulation_hrs, vaccinate_at, vaccinate_ratio, output_file);
}
