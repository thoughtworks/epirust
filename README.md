# EpiRust
## An agent-based epidemiology simulation framework built in Rust

This is a monorepo containing code for the EpiRust framework -

1. engine - The main EpiRust engine, which can be run as a CLI app
2. server - The API server that talks to the engine and serves the frontend
3. web - The Web UI for EpiRust

### Quickstart

The quickest way to run the application is using the CLI interface on the engine.

Prerequisites: 
- Install Rust and Cargo (version 1.40.0 or above). Refer to https://rustup.rs/ for rust installation
- Install `cmake` (version 3.16.4 or later). Installation instructions will depend on your platform. On MacOS you can install it using `brew install cmake`. Refer to https://cmake.org/download/

Running:
- Go to the `engine` directory: `cd engine/`
- Run `cargo run --release`
- To modify the settings, or run with custom settings, use `cargo run --release -- -c config/[your-config].json`. Refer to `default.json` for the available settings.


### License
EpiRust is an open source project licensed under [AGPL v3](https://www.gnu.org/licenses/agpl-3.0.en.html)
