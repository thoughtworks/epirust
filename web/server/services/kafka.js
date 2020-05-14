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
const config = require('../config');

class KafkaConsumerStream {
    constructor(host, topic, kafkaOptions) {
        const clientOptions = {kafkaHost: host};
        const client = new kafka.KafkaClient(clientOptions);
        this._consumerStream = new kafka.ConsumerStream(client, [topic], kafkaOptions);
    }

    getStream() { return this._consumerStream }
}

class KafkaProducerService {
    constructor() {
        const clientOptions = {kafkaHost: config.KAFKA_URL};
        const client = new kafka.KafkaClient(clientOptions);
        this.producer = new kafka.Producer(client);
    }

    send(topicName, message) {
        const producer = this.producer;
        return new Promise(function (resolve, reject) {
            producer.send([{
                topic: topicName,
                messages: JSON.stringify(message), // multi messages should be a array, single message can be just a string or a KeyedMessage instance
                timestamp: Date.now() // <-- defaults to Date.now() (only available with kafka v0.10+)
            }], (err, data) => {
                if(err) {
                    console.log('we consoled');
                    console.log(err);
                  reject(err)
                } else {
                    resolve(data)
                }
            })
        });
    }
}

module.exports = {
    KafkaProducerService, KafkaConsumerStream
};
