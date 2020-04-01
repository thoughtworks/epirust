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

const app = require('../../app');
const supertest = require('supertest');
const request = supertest(app);

jest.mock('../../services/kafka');

describe('simulation controller', () => {

    const postData = {
        "number_of_agents": 10000,
        "public_transport_percentage": 0.2,
        "working_percentage": 0.7,
        "grid_size": 250,
        "simulation_hrs": 10000,
        "vaccinate_at": 5000,
        "vaccinate_percentage": 0.2,
        "lockdown_at_number_of_infections": 100,
        "essential_workers_population": 0.1,
        "hospital_spread_rate_threshold": 100,
        "disease_name": "small_pox",
        "regular_transmission_start_day": 10,
        "high_transmission_start_day": 16,
        "last_day": 22,
        "regular_transmission_rate": 0.05,
        "high_transmission_rate": 0.5,
        "death_rate": 0.2
    }

    let kafkaService;
    let SimulationCountsConsumer;

    afterAll(async () => {
        await app.close()
    });

    beforeEach(() => {
        jest.mock('../../services/kafka');
    })

    afterEach(() => {
        jest.clearAllMocks()
    });


    test('should put init POST request params to kafka topic', async done => {
        kafkaService = require('../../services/kafka')

        const response = await request
            .post('/simulation/init')
            .send({ ...postData });

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
                },
                {
                    Lockdown: {
                        at_number_of_infections: 100,
                        essential_workers_population: 0.1,
                        lock_down_period: 21
                    }
                },
                {
                    BuildNewHospital: {
                        spread_rate_threshold: 100
                    }
                }]
        }

        expect(kafkaService.KafkaProducerService).toHaveBeenCalled();

        const producerService = kafkaService.KafkaProducerService.mock.instances[0];

        expect(producerService.send.mock.calls[0][0]).toBe("simulation_requests");
        const payload = producerService.send.mock.calls[0][1];
        delete payload["sim_id"]; //it is a timestamp, cannot test
        expect(payload).toEqual(kafkaPayload);

        expect(response.status).toBe(200);
        done();
    })

    test('should not put vaccination intervention in kafka topic if params not available in /init POST request', async done => {
        kafkaService = require('../../services/kafka')


        const { vaccinate_at, vaccinate_percentage, ...postDataWithoutVaccinationIntervention } = { ...postData };

        const response = await request
            .post('/simulation/init')
            .send(postDataWithoutVaccinationIntervention);

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
                    Lockdown: {
                        at_number_of_infections: 100,
                        essential_workers_population: 0.1,
                        lock_down_period: 21
                    }
                },
                {
                    BuildNewHospital: {
                        spread_rate_threshold: 100
                    }
                }]
        }

        expect(kafkaService.KafkaProducerService).toHaveBeenCalledTimes(1);

        const producerService = kafkaService.KafkaProducerService.mock.instances[0];

        expect(producerService.send.mock.calls[0][0]).toBe("simulation_requests");
        const payload = producerService.send.mock.calls[0][1];
        delete payload["sim_id"]; //it is a timestamp, cannot test
        expect(payload).toEqual(kafkaPayload);

        expect(response.status).toBe(200);
        done();
    })
});
