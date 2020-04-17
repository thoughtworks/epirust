# EpiRust
## An agent-based epidemiology simulation framework built in Rust

[![Build Status](https://travis-ci.org/thoughtworks/epirust.svg?branch=master)](https://travis-ci.org/thoughtworks/epirust)

This is a monorepo containing code for the EpiRust framework -

1. engine - The main EpiRust engine, which can be run as a CLI app
2. orchestrator - WIP - Coordinator for multiple engines to run large scale simulations
3. web - The Web UI for EpiRust

### Quickstart

The quickest way to run the application is using the CLI interface on the engine. Follow the docker or local setup to get the engine up and running.

#### Docker  

Prerequisites: 
- [Install Docker](https://docs.docker.com/install/) on your local host machine

##### Build EpiRust engine docker image 
```
cd engine
docker build -t epirust-engine .
```

##### Running EpiRust engine docker container
```
docker run --rm --name epirust-engine epirust-engine /bin/bash -c 'cargo run --release -- -c config/default.json && ls *.csv'
```
- Copy the listed CSV produced from the container to the host machine
```
docker cp epirust-engine:/engine/<CSV_FILE> .
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
- Go to the `engine` directory: `cd engine/`
- Run `RUST_LOG=info cargo run --release`
- To modify the settings, or run with custom settings, use `cargo run --release -- -c config/[your-config].json`. Refer to `default.json` for the available settings.

#### Visualization:
- After the simulation is run, it will generate a CSV file. We can plot this using a simple script included in the `engine/plot` directory
  - Ensure you're in the `engine` directory
  - Run `python plot/plot.py --data-path <PATH_TO_CSV_FILE>` - this will plot the csv you provide.

Sample output:
![Epicurves plot](https://user-images.githubusercontent.com/16804955/79537240-79cf3700-809f-11ea-911e-dd7bc4d047e3.png)


### License
EpiRust is an open source project licensed under [AGPL v3](https://www.gnu.org/licenses/agpl-3.0.en.html)
