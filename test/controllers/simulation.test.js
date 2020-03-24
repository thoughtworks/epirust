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
            "number_of_agents": 10000,
            "public_transport_percentage": 0.2,
            "working_percentage": 0.7,
            "grid_size": 250,
            "simulation_hrs": 10000,
            "vaccinate_at": 5000,
            "vaccinate_percentage": 0.2,
            "lockdown_at_number_of_infections": 100,
            "emergency_workers_population": 0.1,
            "hospital_spread_rate_threshold": 100,
            "disease_name": "small_pox",
            "regular_transmission_start_day": 10,
            "high_transmission_start_day": 16,
            "last_day": 22,
            "regular_transmission_rate": 0.05,
            "high_transmission_rate": 0.5,
            "death_rate": 0.2
        };
        const response = await request
            .post('/simulation/init')
            .send(postData);

        const kafkaPayload = {
            population:
            {
                Auto:
                {
                    number_of_agents: 10000,
                    public_transport_percentage: 0.2,
                    working_percentage: 0.7
                }
            },
            disease:
            {
                regular_transmission_start_day: 10,
                high_transmission_start_day: 16,
                last_day: 22,
                regular_transmission_rate: 0.05,
                high_transmission_rate: 0.5,
                death_rate: 0.2
            },
            grid_size: 250,
            hours: 10000,
            interventions:
                [{
                    Vaccinate: {
                        at_hour: 5000,
                        percent: 0.2
                      },
                      Lockdown: {
                        at_number_of_infections: 100,
                        emergency_workers_population: 0.1
                      },
                      BuildNewHospital: {
                        spread_rate_threshold: 100
                      }
                }]
        }

        expect(kafkaService.KafkaProducerService).toHaveBeenCalled();

        const producerService = kafkaService.KafkaProducerService.mock.instances[0];
        expect(producerService.send).toHaveBeenCalledWith("simulation_requests", kafkaPayload)

        expect(response.status).toBe(200);
        done();
    })
});