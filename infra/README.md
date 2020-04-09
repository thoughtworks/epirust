# Setting up the server

## Setting up the dependencies

* MongoDB
* Kafka

To start these dependencies
```sh
docker-compose -f mongodb-kafka.yml up -d
```

To bring down these dependencies
```sh
docker-compose -f mongodb-kafka.yml down
```