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

const jobs_controller = require("../../routes/router")
const express = require("express");
const app = express();
const supertest = require('supertest');
app.use(express.json());
app.use(express.urlencoded({extended: false}));
app.use("/", jobs_controller);
const request = supertest(app);
const NotFound = require('../../db/exceptions/NotFound')
const {updateSimulationStatus} = require('../../db/services/SimulationService');
const {saveJob, fetchJob, fetchJobs, fetchJobsWithTagDetails} = require('../../db/services/JobService');

jest.mock('../../services/kafka');
jest.mock("../../db/services/SimulationService");
jest.mock("../../db/services/JobService");

const KafkaServices = require("../../services/kafka");

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
    "number_of_simulations": 2,
    "tags": [],
    "hospital_beds_percentage": 0.003,
    "asymptomatic_last_day": 22,
    "mild_infected_last_day": 5,

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
      KafkaServices.KafkaProducerService.mockReturnValue({send: mockKafkaSend});

      const jobId = mockObjectId();
      saveJob.mockResolvedValue({_id: jobId, simulations: []});

      const response = await request
        .post('/jobs/init')
        .send({...postData, number_of_simulations: 1});

      expect(response.status).toBe(201)
      expect(saveJob).toHaveBeenCalledTimes(1);
      expect(saveJob.mock.calls[0][0]).toMatchSnapshot()
      expect(saveJob.mock.calls[0][1]).toEqual(1)
      expect(JSON.parse(response.text).jobId).toEqual(jobId.toString())
    })


    it('should update simulation as `failed`, when publishing message on kafka fails', async () => {
      const mockSend = jest.fn()
        .mockResolvedValueOnce()
        .mockRejectedValueOnce(new Error("because we want to"));

      KafkaServices.KafkaProducerService
        .mockReturnValueOnce({send: mockSend})

      const simId1 = mockObjectId();
      const simId2 = mockObjectId();

      const jobId = mockObjectId();
      saveJob.mockResolvedValue({_id: jobId, simulations: [{_id: simId1}, {_id: simId2}]});

      const response = await request
        .post('/jobs/init')
        .send({...postData});

      expect(updateSimulationStatus).toHaveBeenCalledTimes(1);
      expect(updateSimulationStatus).toHaveBeenCalledWith(simId2, 'failed')
      expect(response.status).toBe(500);
    });

    it('should write simulation start request on kafka topic after simulation db insert', async () => {
      const mockKafkaSend = jest.fn().mockReturnValueOnce(Promise.resolve());
      KafkaServices.KafkaProducerService.mockReturnValueOnce({send: mockKafkaSend});

      const simId = mockObjectId();

      const jobId = mockObjectId();
      saveJob.mockResolvedValue({_id: jobId, simulations: [{_id: simId}]});

      const response = await request
        .post('/jobs/init')
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
            asymptomatic_last_day: 22,
            mild_infected_last_day: 5,
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
        geography_parameters: {
          grid_size: 250,
          hospital_beds_percentage: 0.003
        },
        hours: 10000,
        number_of_simulations: 1,
        tags: [],
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

      const jobId = mockObjectId();
      saveJob.mockResolvedValue({_id: jobId, simulations: [{_id: simId1}]});

      const {vaccinate_at, vaccinate_percentage, ...postDataWithoutVaccinationIntervention} = {...postData};

      const response = await request
        .post('/jobs/init')
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
        disease: {
          asymptomatic_last_day: 22,
          mild_infected_last_day: 5,
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
        geography_parameters: {
          grid_size: 250,
          hospital_beds_percentage: 0.003
        },
        hours: 10000,
        interventions: [{
          Lockdown: {
            at_number_of_infections: 100,
            essential_workers_population: 0.1,
          }
        },
          {
            BuildNewHospital: {
              spread_rate_threshold: 100
            }
          }],
        number_of_simulations: 1,
        tags: []
      };

      expect(KafkaServices.KafkaProducerService).toHaveBeenCalledTimes(1);
      expect(mockKafkaSend.mock.calls[0][0]).toBe("simulation_requests");
      const payload = mockKafkaSend.mock.calls[0][1];
      delete payload["sim_id"]; //it is a timestamp, cannot test
      expect(payload).toEqual(kafkaPayload);

      expect(response.status).toBe(201);
    });
  });

  describe('/:job_id', () => {
    it('should return requested job if present in DB', async () => {
      const jobId = mockObjectId();
      const jobInDb = {_id: jobId, config: {configKey: "configValue"}};
      fetchJob.mockResolvedValueOnce(jobInDb)

      const response = await request.get(`/jobs/${jobId.toString()}`);

      expect(fetchJob).toHaveBeenCalledTimes(1);
      expect(fetchJob).toHaveBeenCalledWith(jobId)
      expect(response.status).toEqual(200);
      expect(response.body).toEqual({...jobInDb, _id: jobId.toString()});
    });

    it('should say not found if job does not exist', async () => {
      const jobId = mockObjectId();
      const errorToThrown = new NotFound(jobId);
      fetchJob.mockRejectedValueOnce(errorToThrown)

      const response = await request.get(`/jobs/${jobId.toString()}`);

      expect(fetchJob).toHaveBeenCalledTimes(1);
      expect(fetchJob).toHaveBeenCalledWith(jobId)
      expect(response.status).toEqual(404);
      expect(response.body).toEqual({message: errorToThrown.message});
    });

    it('should say internal server error if any other error occurs', async () => {
      const jobId = mockObjectId();
      fetchJob.mockRejectedValueOnce("some error")

      const response = await request.get(`/jobs/${jobId.toString()}`);

      expect(fetchJob).toHaveBeenCalledTimes(1);
      expect(fetchJob).toHaveBeenCalledWith(jobId)
      expect(response.status).toEqual(500);
    });
  });

  describe('/', () => {
    it('should return given jobs with their status when job ids are NOT specified', async () => {
      let jobsForIds = [
        {jobId: "jobId", config: {tags: [{}], grid_size: 100}, simulations: [{}]},
        {jobId: "jobId2", config: {tags: [{}], grid_size: 100}, simulations: [{}]}
      ];

      fetchJobsWithTagDetails.mockResolvedValueOnce(jobsForIds);

      const response = await request.get("/jobs");

      expect(response.status).toBe(200);
      expect(response.body).toEqual(jobsForIds);
      expect(fetchJobsWithTagDetails).toHaveBeenCalledTimes(1);
      expect(fetchJobsWithTagDetails).toHaveBeenCalledWith(undefined)
    });

    it('should return given jobs with their status when job ids are specified', async () => {
      const jobId = "jobId";
      const jobId2 = "jobId2";

      const jobStatus = {
        jobId: jobId,
        config: {tags: [{id: "test-id", name: 'test-disease'}], grid_size: 100},
        simulations: [{}]
      };
      const jobStatus2 = {
        jobId: jobId2,
        config: {tags: [{id: "test-id", name: 'test-disease'}], grid_size: 100},
        simulations: [{}]
      };

      fetchJobsWithTagDetails.mockResolvedValueOnce([jobStatus, jobStatus2]);

      const response = await request.get("/jobs").query({jobIds: `${jobId},${jobId2}`});

      expect(response.status).toBe(200);
      expect(response.body).toEqual([jobStatus, jobStatus2]);
      expect(fetchJobsWithTagDetails).toHaveBeenCalledTimes(1);
      expect(fetchJobsWithTagDetails).toHaveBeenCalledWith([jobId, jobId2])
    });

    it('should return error status for error occurred while fetching from db', async () => {
      const jobId = "jobId";
      const jobId2 = "jobId2";

      fetchJobsWithTagDetails.mockRejectedValueOnce({message: 'Problem while fetching'});

      const response = await request
        .get("/jobs")
        .query({jobIds: `${jobId},${jobId2}`});

      expect(response.status).toBe(500);
      expect(fetchJobsWithTagDetails).toHaveBeenCalledWith([jobId, jobId2])
    })
  });

  describe('/tags', () => {

    it('should return all tags', async () => {
      const response = await request
        .get("/jobs/tags");

      expect(response.status).toBe(200);
      expect(response.body).toEqual([
        {id: "small-pox", name: "Small Pox"},
        {id: "sars", name: "SARS"},
        {id: "covid", name: "COVID-19"}
      ]);
    })
  })
});
