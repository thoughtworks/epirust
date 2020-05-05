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
const { SimulationStatus } = require('../../../db/models/Simulation')
const {predefinedTags} =require("../../../db/resources/predefined-tags");

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

  describe('fetchJobsWithTagDetails', ()=>{
    it('should return jobs with config', async () => {
      const job1 = await createTestJob({tags:[predefinedTags[0].id]});
      const job2 = await createTestJob({tags:[predefinedTags[1].id, predefinedTags[2].id]});

      const jobs = await JobService.fetchJobsWithTagDetails([job1._id, job2._id]);

      expect(jobs).toHaveLength(2);
      expect(jobs[0].config.tags).toEqual([predefinedTags[0]]);
      expect(jobs[1].config.tags).toEqual([predefinedTags[1], predefinedTags[2]]);
    });
  });

  const testConfig = {'key': 'value', 'field1': 12}

  const createTestJob = (config = testConfig) => {
    return new Job({ config }).save()
  };

  beforeAll(async () => await dbHandler.connect());
  afterEach(async () => await dbHandler.clearDatabase());
  afterAll(async () => await dbHandler.closeDatabase());
});
