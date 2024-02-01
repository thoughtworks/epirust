# Cloud Setup Guide for EpiRust with MPI on AWS ParallelCluster

## Prerequisites

Make sure you have the following prerequisites:

1. **AWS Account**: You need an active AWS account. If you don't have one, you can sign up [here](https://aws.amazon.com/).

2. **AWS CLI**: Install the AWS Command Line Interface (CLI) on your local machine. You can find installation instructions [here](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-files.html).

3. **AWS ParallelCluster**: Install AWS ParallelCluster on your local machine. You can find installation instructions [here](https://docs.aws.amazon.com/parallelcluster/latest/ug/install.html).

## Setup AWS ParallelCluster

### Configure AWS ParallelCluster

```bash
pcluster configure -c <path-to-your-config-file.yaml>
```

To configure AWS ParallelCluster follow steps mentioned [here](https://docs.aws.amazon.com/parallelcluster/latest/ug/install-v3-configuring.html).

While Configuring the pcluster keep following things in mind:

1. While choosing a scheduler - select slurm
2. Know your resource requirements for your tasks that you're going to run. Calculate how much CPU/Memory per compute node(the machines where your distributed tasks will run) you need. And while selecting machine types for your compute node, choose a machine type which fulfills your requirement.
3. And you should keep minvcpus set to 0. It will stop all the compute machines once they are done performing a task and are idle. 
4. In case if you are planning to run a heavy tasks, it would be good if you select a headnode machine with some more computing power than the default one(t2.micro).
5. Create a shared efs and attach it into the config
6. Make sure that head node's subnet, compute nodes' subnet and your shared storage are on the same VPC.
7. Make sure that head node's subnet is public so that it can be accessed from your machine

After configure done, your configure file should look like following: 

```yaml
Region: <aws-region>
Image:
  Os: ubuntu2004
HeadNode:
  InstanceType: t2.micro
  Networking:
    SubnetId: <public-subnet>
    ElasticIp: <elastic-ip>
  Ssh:
    KeyName: <key-name>
Scheduling:
  Scheduler: slurm
  SlurmQueues:
  - Name: <queue-name>
    ComputeResources:
    - Name: <prefix-of-compute-nodes>
      Instances:
      - InstanceType: t2.micro
      MinCount: 0
      MaxCount: 10
    Networking:
      PlacementGroup:
        Enabled: false
      SubnetIds:
      - <private-subnet-id-of-the-public-subnet-given-to-head-node>
SharedStorage:
  - MountDir: <path-on-the-machine-where-you-want-to-access-the-shared-storage e.g., /efs>
    Name: <name-of-the-efs>
    StorageType: Efs
    EfsSettings:
      FileSystemId: <file-system-id-of-your-efs> 
```

### Create an AWS ParallelCluster with your configuration

```bash
pcluster create-cluster --cluster-configuration <path-to-your-config-file.yaml> --cluster-name <cluster-name>
```

By running this it will create a cluster on your aws account with given name.

## Setup and Run EpiRust on the Cluster 

### Step 1: Connect to the Head Node of AWS ParallelCluster

Connect to the head node of your cluster using SSH:

```bash
pcluster ssh --cluster-name <cluster-name> -i <your-ssh.pem>
```

### Step 2: Install Rust and Clang

Ensure Rust and Clang are installed on the head node:

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Clang (if not already installed)
# For example, on Ubuntu:
sudo apt-get install clang
```

### Step 3: Build EpiRust

Clone EpiRust to the head node:
- Clone the EpiRust repository to the user home directory of the head node.

Navigate to the EpiRust directory and build the application:

```bash
cd ~/epirust
cargo build --release
```

### Step 4: Running EpiRust

Navigate to the EpiRust directory and submit a job using the following Slurm command (adjust parameters as needed):

```bash
sbatch -N 13 --ntasks=100 --cpus-per-task=2 --mem-per-cpu=1000 --export=ALL,MPI_MODULE=openmpi/4.1.5 ~/epirust/infra/submit_epirust_aws_mpi.sh
```

**Note:** The above command is specific to the configuration "100K_100". Modify the parameters accordingly for your specific setup.


## After Running EpiRust: Delete the Cluster

Terminate the cluster when you're done to avoid incurring additional charges:

```bash
pcluster delete <cluster-name>
```

## Additional Notes

- (To be confirmed) Install MPI on the head node if not already installed.
- Set `btl_tcp_if_include` in `submit_epirust_aws_mpi.sh`:
    - Ensure that `btl_tcp_if_include` is configured to use the local Ethernet port in the `submit_epirust_aws_mpi.sh` script.

