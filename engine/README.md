# Epirust Simulation Engine

## Building
`cmake` is required to build the project (Required for `rdkafka`. See https://github.com/fede1024/rust-rdkafka#installation for more information). 
On MacOS you can install it using `brew install cmake`.


Use `cargo build` or `cargo build --release`.


## Running
### CLI
By default, the simulation will run as a commandline application. Use `cargo run --release -- --help` for a full list of options.

### Daemon
You can specify the `--daemon` parameter to run the engine in daemon mode. In this mode, the engine will wait for messages from Kafka to start the simulation.

### Output files 
Set "EPI_OUTPUT_DIR" environment variable to the path where you want to store your output files.
`Export EPI_OUTPUT_DIR="output"`

## Kafka setup
In the daemon mode, the engine will consume the `simulation_requests` topic for starting the simulation. It expects a JSON payload with the schema of `config/default.json`

When the simulation is running, it will produce events on the `counts_updated` topic on each simulation hour. The payload will look like:
```json
{"hour":4940,"susceptible":133,"infected":0,"quarantined":1,"recovered":7925,"deceased":1941}
```
