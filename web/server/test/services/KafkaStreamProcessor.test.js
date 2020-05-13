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

jest.mock("../../services/kafka")
const {KafkaConsumerStream} = require('../../services/kafka')
const {KafkaStreamProcessor} = require('../../services/KafkaStreamProcessor')
const {Readable} = require("stream");

describe('KafkaStreamProcessor', () => {
  it('should start kafka stream processor on initialization with options', () => {
    KafkaConsumerStream.mockReturnValueOnce({'getStream': jest.fn()})
    const consumer = {'handleMessage': jest.fn()}
    const kafkaOptions = {
      "autoCommit": true,
      "autoCommitIntervalMs": 1000,
      "autoCommitMsgCount": 100,
      "groupId": "dev_server_consumer"
    };

    new KafkaStreamProcessor(consumer, "test-topic")

    expect(KafkaConsumerStream).toHaveBeenCalledTimes(1)
    expect(KafkaConsumerStream).toHaveBeenCalledWith("localhost:9092", "test-topic", kafkaOptions)
  });

  it('should call the data processor for each stream message on start', (done) => {
    const testTopic = "test-topic";
    const dummyMessage = {key: testTopic, value: {key: "value"}};
    const readableStream = setupReadableStreamWithMessage(dummyMessage);
    readableStream.createCommitStream = () => jest.fn();
    const mockGetStream = jest.fn().mockReturnValueOnce(readableStream);
    KafkaConsumerStream.mockReturnValueOnce({'getStream': mockGetStream})
    const consumer = {'handleMessage': jest.fn()}
    const kafkaStreamProcessor = new KafkaStreamProcessor(consumer, testTopic)

    kafkaStreamProcessor.start()

    process.nextTick(() => {
      expect(consumer.handleMessage).toHaveBeenCalledTimes(1)
      const argument = consumer.handleMessage.mock.calls[0][0];
      expect(JSON.parse(argument)).toEqual(dummyMessage)
      done()
    })
  });

  function setupReadableStreamWithMessage(message) {
    const readableStream = new Readable();
    readableStream.push(JSON.stringify(message))
    readableStream.push(null)
    return readableStream;
  }
});