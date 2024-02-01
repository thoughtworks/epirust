# EpiRust Setup with MPI

Welcome to the setup guide for EpiRust using MPI (Message Passing Interface)! Before you can start using the application, you need to set up MPI on your local machine. Follow the steps below to get everything up and running.

## Prerequisites

1. Install an MPI library compatible with your operating system. Common MPI libraries include Open MPI and MPICH.
   Follow MPI library requirements of rsmpi (https://github.com/rsmpi/rsmpi) since that's the library we are using in the EpiRust.
2. Install mpirun (https://github.com/AndrewGaspar/cargo-mpirun)

## Application Setup

Now that MPI is set up, you can proceed to run EpiRust. Follow the instructions below to configure and run the application using MPI.

1. You can directly run engines since in this implementation you don't need to run orchestrator
    `cargo mpirun -n 2 --bin engine-app -- -m mpi -c <path/to/simulation/config>`

   Note:
   You have to run the engine command for each of the engines. e.g., if your simulation **config** has **3 engines** you have to run the command **3 times**

