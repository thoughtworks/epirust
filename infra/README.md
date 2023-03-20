# Running Epirust in k8s cluster using helm chart

## Pre-requisites

* Kafka 
* Some persistence storage

Make sure your kafka cluster is up and running, and your kafka service is accessible from the k8s cluster you plan to run simulation.

Create "config" and "output" directory in your storage.
Create PV and PVC in order to use that storage. You can refer the sample pv and pvc files at helm-chart/ 

Build the docker images for engine
```sh
docker build -t <image_repo:tag> -f engine-app/dockerfile .
```
Build the docker images for orchestrator
```sh
docker build -t <image_repo:tag> -f orchestrator/dockerfile .
```
Update the `helm-chart/epirust/values.yaml` with volume name, claim name, kafka url, config file, resources etc.

To start simulation
```sh
helm install epirust helm-chart/epirust
```

Output files will be stored at given `output_path` in `values.yaml` and Logs files would be stored at `output_path/logs` once the simulation is completed.  