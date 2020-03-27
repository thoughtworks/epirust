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

let kafkaService = require('../../services/kafka');
const epirustService = require('../../services/epirust');

jest.mock('../../services/kafka');
jest.mock('../../services/epirust');

describe('simulation controller', () => {

    afterAll(async () => {
        await app.close()
    });

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
                },
                {
                    Lockdown: {
                        at_number_of_infections: 100,
                        essential_workers_population: 0.1,
                        lock_down_period: 21
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
