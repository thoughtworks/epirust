# EpiRust setup with Kafka

Welcome to the setup guide for EpiRust! Before you can start using the application, you need to set up Apache Kafka locally. Follow the steps below to get everything up and running smoothly.

## Prerequisites

Make sure you have the following prerequisites installed on your machine:

1. **Java**: Ensure that you have Java installed on your system. Kafka requires Java to run. You can download and install Java from [here](https://www.oracle.com/java/technologies/javase-downloads.html).

2. **Apache Kafka**: Download and install Apache Kafka on your machine. You can find the latest version of Kafka [here](https://kafka.apache.org/downloads). Follow the installation instructions for your operating system.

### Kafka Setup

Once you have Java and Kafka installed, follow these steps to start Kafka locally:

1. Start Zookeeper

    ```bash
    /path/to/bin/zookeeper-server-start.sh /path/to/config/zookeeper.properties
    ```

2. Start Kafka Server

    Open a new terminal and start the Kafka server:

    ```bash
    /path/to/bin/kafka-server-start.sh /path/to/config/server.properties
    ```

## Application Setup

Now that Kafka is set up, you can proceed to set up and run EpiRust.

To Run EpiRust(with Kafka Implementation) follow below steps:

1. Go to root of project(EpiRust)
2. Run the following to start the orchestrator
    `cargo run --bin epirust-orchestrator -- -c <path/to/simulation-config>`
3. Run the following to start the engines:
    `cargo run --bin engine-app -- -m kafka -i <engine-id>`

   Note:
    You have to run the engine command for each of the engines. e.g., if your simulation **config** has **3 engines** you have to run the command **3 times**

## Additional Notes

- Make sure to stop Kafka and Zookeeper when you're done using the application. You can stop them by pressing `Ctrl+C` in their respective terminals.

- Ensure that the Kafka server is running whenever you want to use EpiRust.

