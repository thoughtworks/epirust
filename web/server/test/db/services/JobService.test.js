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
const {mockObjectId} = require('../../helpers');

describe('Job Service', () => {
  describe('saveJob', () => {
    it('should save the job with config', async () => {

      const writeAck = await JobService.saveJob(testConfig)

      const receivedJob = (await Job.findOne({}).exec()).toObject()
      expect(receivedJob.config).toEqual(testConfig)
      expect(writeAck).toHaveProperty('_id');
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

  const testConfig = {'key': 'value', 'field1': 12}

  const createTestJob = () => {
    return new Job({config: testConfig}).save()
  }

  beforeAll(async () => await dbHandler.connect());
  afterEach(async () => await dbHandler.clearDatabase());
  afterAll(async () => await dbHandler.closeDatabase());
});
