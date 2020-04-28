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
jest.mock("../../db/services/SimulationService");
jest.mock("../../db/services/JobService");

const KafkaServices = require("../../services/kafka");

const { updateSimulationStatus, saveSimulation } = require('../../db/services/SimulationService');
const { saveJob } = require('../../db/services/JobService');
const {mockObjectId} = require('../helpers');

describe('jobs controller', () => {

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
        "death_rate": 0.2,
        "enable_citizen_state_messages": false,
        "percentage_asymptomatic_population": 0.3,
        "percentage_severe_infected_population": 0.3,
        "exposed_duration": 48,
        "pre_symptomatic_duration": 0,
        "number_of_simulations": 2
    };
    afterAll(async () => {
        await app.close()
    });

    afterEach(() => {
        jest.clearAllMocks()
    });

    describe('/init', function () {
        it('should create a job with id and config', async () => {
            const mockKafkaSend = jest.fn().mockReturnValue(Promise.resolve());
            KafkaServices.KafkaProducerService.mockReturnValue({ send: mockKafkaSend });

            saveSimulation.mockResolvedValue({_id: mockObjectId()});

            const jobId = mockObjectId();
            saveJob.mockResolvedValue({_id: jobId});

            await request
              .post('/api/jobs/init')
              .send({ ...postData, number_of_simulations: 1 });

            expect(saveJob).toHaveBeenCalledTimes(1);
            expect(saveJob.mock.calls[0][0]).toMatchSnapshot()
        })

        it('should create two simulations on successful job creation', async () => {
            const mockKafkaSend = jest.fn().mockReturnValue(Promise.resolve());
            KafkaServices.KafkaProducerService.mockReturnValue({ send: mockKafkaSend });

            const mockSimulationSave = [{_id: mockObjectId()}, {_id: mockObjectId()}]
            saveSimulation.mockImplementation(() => Promise.resolve(mockSimulationSave.shift()));

            const jobId = mockObjectId();
            saveJob.mockResolvedValue({_id: jobId});

            const response = await request
              .post('/api/jobs/init')
              .send({ ...postData });

            expect(saveSimulation).toHaveBeenCalledTimes(2);
            expect(saveSimulation).toHaveBeenNthCalledWith(1, {status: 'in-queue', job_id: jobId})
            expect(saveSimulation).toHaveBeenNthCalledWith(2, {status: 'in-queue', job_id: jobId})
            expect(response.status).toBe(201);
            expect(JSON.parse(response.text).jobId).toEqual(jobId.toString())
        });

        it('should update simulation as `failed`, when publishing message on kafka fails', async () => {
          const mockSend = jest.fn()
            .mockResolvedValueOnce()
            .mockRejectedValueOnce(new Error("because we want to"));

          KafkaServices.KafkaProducerService
            .mockReturnValueOnce({send: mockSend})

          const simId1 = mockObjectId();
          const simId2 = mockObjectId();
          saveSimulation
            .mockResolvedValueOnce({_id: simId1})
            .mockResolvedValueOnce({_id: simId2});

          const jobId = mockObjectId();
          saveJob.mockResolvedValue({_id: jobId});

          const response = await request
            .post('/api/jobs/init')
            .send({...postData});

          expect(updateSimulationStatus).toHaveBeenCalledTimes(1);
          expect(updateSimulationStatus).toHaveBeenCalledWith(simId2, 'failed')
          expect(response.status).toBe(500);
        });

        it('should write simulation start request on kafka topic after simulation db insert', async () => {
          const mockKafkaSend = jest.fn().mockReturnValueOnce(Promise.resolve());
          KafkaServices.KafkaProducerService.mockReturnValueOnce({send: mockKafkaSend});

          const simId = mockObjectId();
          saveSimulation
            .mockResolvedValueOnce({_id: simId})

          const jobId = mockObjectId();
          saveJob.mockResolvedValue({_id: jobId});

          const response = await request
            .post('/api/jobs/init')
            .send({...postData, number_of_simulations: 1});

          const kafkaPayload = {
            enable_citizen_state_messages: false,
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
                death_rate: 0.2,
                percentage_asymptomatic_population: 0.3,
                percentage_severe_infected_population: 0.3,
                exposed_duration: 48,
                pre_symptomatic_duration: 0
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

          expect(KafkaServices.KafkaProducerService).toHaveBeenCalledTimes(1);

          expect(mockKafkaSend).toHaveBeenCalledTimes(1);
          expect(mockKafkaSend.mock.calls[0][0]).toBe("simulation_requests");
          const payload = mockKafkaSend.mock.calls[0][1];
          expect(payload).toEqual({...kafkaPayload, sim_id: simId.toString()});

          expect(response.status).toBe(201);
        });

        it('should not put vaccination intervention in kafka topic if params not available in /init POST request', async () => {
          const mockKafkaSend = jest.fn().mockReturnValueOnce(Promise.resolve());
          KafkaServices.KafkaProducerService.mockReturnValueOnce({send: mockKafkaSend});

          const simId1 = mockObjectId();
          saveSimulation
            .mockResolvedValueOnce({_id: simId1})

          const jobId = mockObjectId();
          saveJob.mockResolvedValue({_id: jobId});

          const {vaccinate_at, vaccinate_percentage, ...postDataWithoutVaccinationIntervention} = {...postData};

          const response = await request
            .post('/api/jobs/init')
            .send({...postDataWithoutVaccinationIntervention, number_of_simulations: 1});

          const kafkaPayload = {
            enable_citizen_state_messages: false,
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
                death_rate: 0.2,
                percentage_asymptomatic_population: 0.3,
                percentage_severe_infected_population: 0.3,
                exposed_duration: 48,
                pre_symptomatic_duration: 0
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
          };

          expect(KafkaServices.KafkaProducerService).toHaveBeenCalledTimes(1);
          expect(mockKafkaSend.mock.calls[0][0]).toBe("simulation_requests");
          const payload = mockKafkaSend.mock.calls[0][1];
          delete payload["sim_id"]; //it is a timestamp, cannot test
          expect(payload).toEqual(kafkaPayload);

          expect(response.status).toBe(201);
        });
    });
});