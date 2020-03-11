const app = require('../../app');
const supertest = require('supertest');
const request = supertest(app);

let kafkaService = require('../../services/kafka');
const epirustService = require('../../services/epirust');

jest.mock('../../services/kafka');
jest.mock('../../services/epirust');

describe('simulation controller', () => {

    test('should post request to get per tick stats', async done => {

        const response = await request.post('/simulation/');
        //TODO: Add matcher for order of execution - Jayanta
        expect(kafkaService.KafkaConsumerService).toHaveBeenCalledWith('localhost:9092', 'counts_updated', 1);
        expect(response.status).toBe(200);
        done();
    });


    test('should put init POST request params to kafka topic', async done => {
        const postData = {
            disease_name: "::disease::",
            grid_size: 5000,
            number_of_agents: 10000,
            simulation_hrs: 5000,
            public_transport_percentage: 30,
            working_percentage: 30,
            vaccinate_at: "::unknown_property::",
            vaccinate_percentage: 76
        };
        const response = await request
            .post('/simulation/init')
            .send(postData);

        expect(kafkaService.KafkaProducerService).toHaveBeenCalled();

        const producerService = kafkaService.KafkaProducerService.mock.instances[0];
        expect(producerService.send).toHaveBeenCalledWith("simulation_requests", postData)

        expect(response.status).toBe(200);
        done();
    })
});