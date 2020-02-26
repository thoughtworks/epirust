const kafka = require('kafka-node');

module.exports = class KafkaConsumerService{
    constructor(host, topic, id) {
        const consumerOptions = {
            kafkaHost: host,
            id: id
        };
        const client = new kafka.KafkaClient(consumerOptions);
        this.consumer = new kafka.Consumer(client, [{topic: topic}], {});

        this.consumer.on('message', function (message) {
            console.log(message);
        });
    }
};