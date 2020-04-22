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
const dbHandler = require('../db-handler');
const JobService = require('../../../db/services/JobService');
const {Job} = require('../../../db/models/Job');

describe('Job Service', () => {
  describe('saveJob', () => {
    it('should save the job with config', async () => {
      const config = {'key': 'value', 'field1': 12}

      const writeAck = await JobService.saveJob(config)

      const receivedJob = (await Job.findOne({}).exec()).toObject()
      expect(receivedJob.config).toEqual(config)
      expect(writeAck).toHaveProperty('_id');
    });
  });

  beforeAll(async () => await dbHandler.connect());
  afterEach(async () => await dbHandler.clearDatabase());
  afterAll(async () => await dbHandler.closeDatabase());
});
