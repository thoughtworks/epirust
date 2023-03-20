# EpiRust
## An agent-based epidemiology simulation framework built in Rust

[![Build Status](https://travis-ci.org/thoughtworks/epirust.svg?branch=master)](https://travis-ci.org/thoughtworks/epirust)

This is a monorepo containing code for the EpiRust framework -

1. engine - The main EpiRust engine, which can be run as a CLI app
2. orchestrator - Coordinator for multiple engines to run large scale simulations

Note: We are no longer maintaining the Web UI for EpiRust which allows you to trigger simulations and grid visualization for upto 1000 agents. Hence, it has been removed from here. Please refer sims2020 branch for web. 

### Quickstart

The quickest way to run the application is using the CLI interface on the engine. Follow the docker or local setup to get the engine up and running.

#### Docker  

Prerequisites: 
- [Install Docker](https://docs.docker.com/install/) on your local host machine

##### Build EpiRust engine docker image 
```
docker build -t epirust-engine -f ./engine-app/Dockerfile .
```

##### Running EpiRust engine docker container
```
docker run --rm --name epirust-engine epirust-engine /bin/sh -c './epirust-engine -c config/default.json && ls *.csv'
```
- Copy the listed CSV produced from the container to the host machine
```
docker cp epirust-engine:/home/nonroot/app/<CSV_FILE> .
```
- Go to the visualization section to plot the copied CSV file


#### Local setup

Prerequisites: 
- Install Rust and Cargo (version 1.40.0 or above). Refer to https://rustup.rs/ for rust installation
- Install `cmake` (version 3.16.4 or later). Installation instructions will depend on your platform. On MacOS you can install it using `brew install cmake`. Refer to https://cmake.org/download/
- The following dependencies are needed on Debian: `ca-certificates curl file build-essential autoconf automake autotools-dev libtool xutils-dev cmake pkg-config libfreetype6-dev libfontconfig1-dev xclip`
- Python 3 (for visualization and charting). We also need the `pandas` and `matplotlib` libraries which can be installed using `pip` or `conda`.
    - `pip install pandas matplotlib`

Running:
- Go to the `engine-app` directory: `cd engine-app/`
- Run `RUST_LOG=info cargo run --release`
- To modify the settings, or run with custom settings, use `cargo run --release -- -c config/[your-config].json`. Refer to `default.json` for the available settings.

#### Visualization:
- After the simulation is run, it will generate a CSV file. We can plot this using a simple script included in the `engine/plot` directory
  - Ensure you're in the `engine` directory
  - Run `python plot/plot.py --data-path <PATH_TO_CSV_FILE>` - this will plot the csv you provide.

Sample output:
![Epicurves plot](https://user-images.githubusercontent.com/16804955/79959420-bf29a500-84a1-11ea-844e-11cc721b7b79.png)


#### Running multi-region simulations

You can run EpiRust for multi-region simulations (e.g. to simulate a group of cities). This will require the orchestrator, and each engine representing a region. The [architecture page](https://github.com/thoughtworks/epirust/wiki/Technical-Architecture#architecture-for-a-multi-city-simulation) on the wiki describes how this works at a high level.

Steps for running a multi-region simulation:
1. Prepare a configuration. A configuration is a json file which consists of two parts:
    - The simulation configuaration for each engine. (Note that currently there is some repetition needed in the configuration for each engine)
    - The travel plan, which defines a matrix containing the daily travellers from one region to another.
    You can take a look at the `orchestrator/config` directory for examples of the configuration.
2. Start Kafka. The engines and orchestrator will communicate using Kafka. (The topics should be created automatically when the first messages are sent).
3. Start the engines. If there are n regions in the config, n engines should be started with the name specified in the config. E.g. `./epirust-engine -d -i [engine-name]`, where `epirust-engine` is the engine-app binary.
4. Start the orchestrator, pointing to the config file. E.g. `./epirust-orchestrator -c [path_to_config]`. The simulation should now start.

It will generate output CSV and JSON files which you can use to for analysis and charting.

### License
EpiRust is an open source project licensed under [AGPL v3](https://www.gnu.org/licenses/agpl-3.0.en.html)
