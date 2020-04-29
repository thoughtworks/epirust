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

const NotFound = require("../../../db/exceptions/NotFound");
const dbHandler = require('../db-handler');
const JobService = require('../../../db/services/JobService');
const {Job} = require('../../../db/models/Job');
const { mockObjectId } = require('../../helpers');
const SimulationService = require('../../../db/services/SimulationService')
const { SimulationStatus } = require('../../../db/models/Simulation')

jest.mock('../../../db/services/SimulationService')

describe('Job Service', () => {
  describe('saveJob', () => {
    it('should save the job with config and two simulations', async () => {

      const writeAck = await JobService.saveJob(testConfig, 2)

      const receivedJob = (await Job.findOne({}).exec()).toObject()
      expect(receivedJob.config).toEqual(testConfig)
      expect(receivedJob.simulations).toHaveLength(2)
      receivedJob.simulations
        .forEach(s => expect(s.status).toBe(SimulationStatus.INQUEUE));
      expect(writeAck).toHaveProperty('_id');
      writeAck.simulations.forEach(s => expect(s).toHaveProperty("_id"));
    });
  });

  describe('fetchJob', () => {
    it('should return job if present in database', async () => {
      const jobDoc = await createTestJob()

      const receivedJob = await JobService.fetchJob(jobDoc._id);

      expect(receivedJob).toBeDefined();
      expect(receivedJob.toObject().config).toEqual(testConfig);
    });

    it('should fail if the job is not present in db', async () => {
      const randomJobId = mockObjectId();

      await expect(JobService.fetchJob(randomJobId)).rejects.toBeInstanceOf(NotFound)
      await expect(JobService.fetchJob(randomJobId)).rejects.toEqual(new NotFound(randomJobId))
    });
  });
  describe('fetchJobsStatus', () => {
    const jobId = mockObjectId()
    
    it('should return job status as failed if any of the simulations fail for given job id', async () => {
      const simulationsStatusForJobId = [
        {
          _id: jobId,
          simulations: [
            { status: SimulationStatus.FAILED, id: mockObjectId() },
            { status: SimulationStatus.INQUEUE, id: mockObjectId() },
            { status: SimulationStatus.FINISHED, id: mockObjectId() },
          ]
        }
      ];
      SimulationService.groupSimulationsByJobId.mockImplementation(() => ({
        exec: jest.fn().mockResolvedValueOnce(simulationsStatusForJobId)
      }))
      const jobsStatus = await JobService.fetchJobsStatus([jobId])
      expect(jobsStatus[0].status).toBe(SimulationStatus.FAILED)
      expect(SimulationService.groupSimulationsByJobId).toHaveBeenCalledWith([jobId])
    });

    it('should return job status as finished when all simulations finish', async() => {
      const simulationsStatusForJobId = [
        {
          _id: jobId,
          simulations: [
            { status: SimulationStatus.FINISHED, id: mockObjectId() },
            { status: SimulationStatus.FINISHED, id: mockObjectId() },
          ]
        }
      ];
      SimulationService.groupSimulationsByJobId.mockImplementation(() => ({
        exec: jest.fn().mockResolvedValueOnce(simulationsStatusForJobId)
      }))

      const jobsStatus = await JobService.fetchJobsStatus([jobId])
      expect(jobsStatus[0].status).toBe(SimulationStatus.FINISHED)
    })

    it('should return job status as running when all simulations are running', async() => {
      const simulationsStatusForJobId = [
        {
          _id: jobId,
          simulations: [
            { status: SimulationStatus.RUNNING, id: mockObjectId() },
            { status: SimulationStatus.RUNNING, id: mockObjectId() },
          ]
        }
      ];
      SimulationService.groupSimulationsByJobId.mockImplementation(() => ({
        exec: jest.fn().mockResolvedValueOnce(simulationsStatusForJobId)
      }))

      const jobsStatus = await JobService.fetchJobsStatus([jobId])
      expect(jobsStatus[0].status).toBe(SimulationStatus.RUNNING)
    })

    it('should return job status as in queue when all simulations are in queue', async() => {
      const simulationsStatusForJobId = [
        {
          _id: jobId,
          simulations: [
            { status: SimulationStatus.INQUEUE, id: mockObjectId() },
            { status: SimulationStatus.INQUEUE, id: mockObjectId() },
          ]
        }
      ];
      SimulationService.groupSimulationsByJobId.mockImplementation(() => ({
        exec: jest.fn().mockResolvedValueOnce(simulationsStatusForJobId)
      }))

      const jobsStatus = await JobService.fetchJobsStatus([jobId])
      expect(jobsStatus[0].status).toBe(SimulationStatus.INQUEUE)
    })

    it('should return job status as running if few are in queue and even one is running', async() => {
      const simulationsStatusForJobId = [
        {
          _id: jobId,
          simulations: [
            { status: SimulationStatus.INQUEUE, id: mockObjectId() },
            { status: SimulationStatus.INQUEUE, id: mockObjectId() },
            { status: SimulationStatus.RUNNING, id: mockObjectId() },
          ]
        }
      ];
      SimulationService.groupSimulationsByJobId.mockImplementation(() => ({
        exec: jest.fn().mockResolvedValueOnce(simulationsStatusForJobId)
      }))

      const jobsStatus = await JobService.fetchJobsStatus([jobId])
      expect(jobsStatus[0].status).toBe(SimulationStatus.RUNNING)
    })

    it('should return job status as running if few are in queue and one is finished', async() => {
      const simulationsStatusForJobId = [
        {
          _id: jobId,
          simulations: [
            { status: SimulationStatus.INQUEUE, id: mockObjectId() },
            { status: SimulationStatus.INQUEUE, id: mockObjectId() },
            { status: SimulationStatus.FINISHED, id: mockObjectId() },
          ]
        }
      ];
      SimulationService.groupSimulationsByJobId.mockImplementation(() => ({
        exec: jest.fn().mockResolvedValueOnce(simulationsStatusForJobId)
      }))

      const jobsStatus = await JobService.fetchJobsStatus([jobId])
      expect(jobsStatus[0].status).toBe(SimulationStatus.RUNNING)
    })
  })


  describe('fetchJobs', () => {
    it('should return all the jobs in db when no ids specified', async () => {
      const job1 = await createTestJob()
      const job2 = await createTestJob()

      const jobs = await JobService.fetchJobs();

      const receivedJobs = [];
      for await (const job of jobs) {
        receivedJobs.push(job.toObject())
      }

      expect(receivedJobs).toHaveLength(2)
      expect(receivedJobs[0]).toEqual(job1.toObject())
      expect(receivedJobs[1]).toEqual(job2.toObject())
    });

    it('should return only jobs asked for', async () => {
      const job1 = await createTestJob()
      const job2 = await createTestJob()
      const job3 = await createTestJob()

      const jobs = await JobService.fetchJobs([job1._id, job2._id]);

      const receivedJobs = [];
      for await (const job of jobs) {
        receivedJobs.push(job.toObject())
      }

      expect(receivedJobs).toHaveLength(2)
      expect(receivedJobs[0]).toEqual(job1.toObject())
      expect(receivedJobs[1]).toEqual(job2.toObject())
    });
  });

  const testConfig = {'key': 'value', 'field1': 12}

  const createTestJob = () => {
    return new Job({ config: testConfig }).save()
  }

  beforeAll(async () => await dbHandler.connect());
  afterEach(async () => await dbHandler.clearDatabase());
  afterAll(async () => await dbHandler.closeDatabase());
});
