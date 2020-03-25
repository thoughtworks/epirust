const kafka = require('kafka-node');

class KafkaConsumerService {
    constructor(host, topic, id) {
        const consumerOptions = {
            kafkaHost: host,
            id: id
        };
        const client = new kafka.KafkaClient(consumerOptions);
        this.consumer = new kafka.Consumer(client, [{ topic: topic }], {});
    }
};

class KafkaProducerService {
    constructor() {
        const client = new kafka.KafkaClient();
        this.producer = new kafka.Producer(client);
    }

    send(topicName, message) {
        this.producer.send([{
            topic: topicName,
            messages: JSON.stringify(message), // multi messages should be a array, single message can be just a string or a KeyedMessage instance
            timestamp: Date.now() // <-- defaults to Date.now() (only available with kafka v0.10+)
        }], function(err,data){
            console.log("Message sent to topic");
        });
    }
};

module.exports = {
    KafkaConsumerService, KafkaProducerService
}