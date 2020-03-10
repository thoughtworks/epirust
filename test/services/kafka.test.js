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