const app = require('../../app');
const supertest = require('supertest');
const request = supertest(app);

const kafkaConsumerService = require('../../services/kafka');
const epirustService = require('../../services/epirust');

jest.mock('../../services/kafka');
jest.mock('../../services/epirust');

describe('simulation controller', () => {

    test('should post request', async done => {
        const response = await request.post('/simulation/');
        //TODO: Add matcher for order of execution - Jayanta
        expect(kafkaConsumerService).toHaveBeenCalledWith('localhost:9092', 'counts_updated', 1);
        expect(epirustService).toHaveBeenCalledTimes(1);
        //TODO: validate start has been called
        expect(response.status).toBe(200);
        done();
    });
});