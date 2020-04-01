/*
 * EpiRust
 * Copyright (c) 2020  ThoughtWorks, Inc.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

const kafka = require('kafka-node');

class KafkaGroupConsumer {
    constructor(host, topic, id, groupId) {
        const consumerOptions = {
            kafkaHost: host,
            id: id,
            groupId: groupId
        };
        this.consumerStream = new kafka.ConsumerGroupStream(consumerOptions, [topic]);
    }
}

class KafkaConsumerService {
    constructor(host, topic, id) {
        const consumerOptions = {
            kafkaHost: host,
            id: id
        };
        const client = new kafka.KafkaClient(consumerOptions);
        this.consumer = new kafka.Consumer(client, [{ topic: topic }], {});
    }

    close() {
        this.consumer.close(() => {});
    }
}

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
}

module.exports = {
    KafkaConsumerService, KafkaProducerService, KafkaGroupConsumer
};
