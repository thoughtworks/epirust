extern crate clap;
#[macro_use]
extern crate serde_derive;

use clap::{App, Arg, value_t};

use crate::epidemiology_simulation::SimulationParams;
use crate::kafka_service::KafkaConsumer;

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

#[tokio::main]
async fn main() {
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
        .arg(Arg::with_name("daemon")
            .long("daemon")
            .help("Start the engine in daemon mode. It will wait for messages from Kafka. \
            Specifying this flag will cause other arguments to be ignored")
            .takes_value(false))
        .get_matches();

    let daemon = matches.is_present("daemon");

    if daemon {
        println!("Started in daemon mode");
        let consumer = KafkaConsumer::new();
        consumer.listen_loop().await;
    } else {
        let input_count = value_t!(matches, "agents", i32).unwrap_or(10000);
        let simulation_hours = value_t!(matches, "hours", i32).unwrap_or(10000);
        let public_transport = value_t!(matches, "public_transport", f64).unwrap_or(PUBLIC_TRANSPORT_PERCENTAGE);
        let working = value_t!(matches, "working", f64).unwrap_or(WORKING_PERCENTAGE);
        let vaccinate_at = value_t!(matches, "vaccinate_at", i32).unwrap_or(VACCINATION_TIME);
        let vaccinate_ratio = value_t!(matches, "vaccinate_ratio", f64).unwrap_or(VACCINATION_PERCENTAGE);
        let disease_name = matches.value_of("disease").unwrap_or("small_pox");

        let (count, grid_default) = match input_count {
            0..=100 => (100, 25),
            101..=1000 => (1000, 80),
            1001..=10000 => (10000, 250),
            10001..=100000 => (100000, 800),
            100001..=1_000_000 => (1_000_000, 2500),
            1_000_001..=2_000_000 => (2_000_000, 3550),
            2_000_001..=5_000_000 => (5_000_000, 5660),
            _ => panic!("Cannot run for {} agents", input_count)
        };
        let grid = value_t!(matches, "grid", i32).unwrap_or(grid_default);

        let output_file = format!("simulation_{}.csv", count);
        let params = SimulationParams::new(String::from(disease_name), grid, count, simulation_hours,
                                           public_transport, working, vaccinate_at, vaccinate_ratio, output_file);
        let mut epidemiology = epidemiology_simulation::Epidemiology::new(&params);
        epidemiology.run(&params);
        println!("Done");
    }
}
