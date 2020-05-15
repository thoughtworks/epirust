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
const through2 = require("through2");
const {KafkaConsumerStream} = require('../services/kafka')
const config = require('../config')

class KafkaStreamProcessor {
  constructor(consumer, topic) {
    const kafkaOptions = {
      autoCommit: true,
      autoCommitIntervalMs: 1000,
      autoCommitMsgCount: 100,
      groupId: config.KAFKA_GROUP
    };

    this._topic = topic;
    this._kafkaConsumer = new KafkaConsumerStream(config.KAFKA_URL, topic, kafkaOptions);
    this._dataConsumer = consumer
  }

  async start() {
    const consumerStream = this._kafkaConsumer.getStream();
    const commitStream = consumerStream.createCommitStream();

    console.log(`Started consumer stream on topic ${this._topic}`)

    consumerStream
      .pipe(through2.obj(async (data, enc, cb) => {
        await this._dataConsumer.handleMessage(data);
        cb(null, data);
      }))
      .pipe(commitStream);
  }
}

module.exports = {KafkaStreamProcessor}