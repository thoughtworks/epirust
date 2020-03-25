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

## Kafka setup
In the daemon mode, the engine will consume the `simulation_requests` topic for starting the simulation. It expects a JSON payload to be sent in the following format
```json
{"disease_name": "small_pox", "grid_size": 250, "number_of_agents": 10000, "simulation_hrs": 10000, "public_transport_percentage": 0.2, "working_percentage": 0.7, "vaccinate_at": 5000, "vaccinate_percentage": 0.2}
```

When the simulation is running, it will produce events on the `counts_updated` topic on each simulation hour. The payload will look like:
```json
{"hour":4940,"susceptible":133,"infected":0,"quarantined":1,"recovered":7925,"deceased":1941}
```

## Disease Configuration
Disease parameters can be configured in the `config/diseases.yaml` directory.
