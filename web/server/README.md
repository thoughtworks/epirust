External dependencies:

# Epirust
```cp epirust ./external/```

# Kafka
Download Kafka - https://kafka.apache.org/quickstart


```zookeeper-server-start /usr/local/etc/kafka/zookeeper.properties```

```kafka-server-start /usr/local/etc/kafka/server.properties```


# Create a topic
```./bin/kafka-topics.sh --create --bootstrap-server localhost:9092 --replication-factor 1 --partitions 1 --topic [NAME]```

# Consume all from topic
```./bin/kafka-console-consumer.sh --bootstrap-server localhost:9092 --topic [NAME] --from-beginning```

# Describe topic 
```./bin/kafka-topics.sh --describe --bootstrap-server localhost:9092 --topic [NAME]```

# Produce messages from CLI
```./bin/kafka-console-producer.sh --broker-list localhost:9092 --topic [NAME]```