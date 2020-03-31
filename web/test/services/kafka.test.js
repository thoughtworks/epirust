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

const kafkaService = require('../../services/kafka');
const kafka = require('kafka-node');

jest.mock('kafka-node');

describe('Kafka service', () => {

    test('Should setup kafka consumer', () => {
        let kafkaClientSpy = jest.spyOn(kafka, 'KafkaClient');
        kafkaClientSpy.mockReturnValue({client: "dummy"});
        let kafkaConsumerSpy = jest.spyOn(kafka, 'Consumer');
        const expectedClientArguments = {
            kafkaHost: "localhost:9000",
            id: 1
        };

        new kafkaService.KafkaConsumerService("localhost:9000", "test", 1);

        expect(kafkaClientSpy).toHaveBeenCalledWith(expectedClientArguments);
        expect(kafkaConsumerSpy).toHaveBeenCalledWith({client: "dummy"}, [{topic: 'test'}], {});
        // TODO: Add test for event handling - Jayanta
    });
});
