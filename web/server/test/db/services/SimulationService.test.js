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
const SimulationService = require('../../../db/services/SimulationService');
const {SimulationStatus, Simulation} = require('../../../db/models/Simulation');
const {Job} = require('../../../db/models/Job')
const NotFound = require("../../../db/exceptions/NotFound")
const {mockObjectId} = require("../../helpers")

describe('Simulation Service', () => {

  beforeAll(async () => await dbHandler.connect());
  afterEach(async () => await dbHandler.clearDatabase());
  afterAll(async () => await dbHandler.closeDatabase());

  const createNewJob = (...simulationStatuses) => {
    return new Job({
      simulations: simulationStatuses.map(s => ({status: s}))
    }).save();
  }

  const createNewSimulation = (simulationStatus, jobId = mockObjectId()) => {
    return new Simulation({job_id: jobId, status: simulationStatus}).save();
  }

  describe('updateSimulationStatus', () => {
    it('should set simulation status to ended', async () => {
      const {_id: jobId, simulations} = await createNewJob(SimulationStatus.RUNNING)

      await SimulationService.updateSimulationStatus(simulations[0]._id, SimulationStatus.FAILED);

      const s = await Job.findOne({_id: jobId}).exec();
      expect(s.toObject().simulations[0].status).toBe(SimulationStatus.FAILED);
    });
  });

  describe('markGridConsumptionFinished', () => {
    it('should set grid consumption finished to true', async () => {
      const {_id: jobId, simulations} = await createNewJob(SimulationStatus.RUNNING)

      await SimulationService.markGridConsumptionFinished(simulations[0]._id);

      const s = await Job.findOne({_id: jobId}).exec();
      expect(s.simulations[0].grid_consumption_finished).toEqual(true);
    });
  });

  describe('fetchSimulation', () => {
    it('should find simulation with given id', async () => {
      const job = await createNewJob(SimulationStatus.RUNNING, SimulationStatus.FINISHED)
      const simId1 = job.simulations[0]._id;

      const doc = await SimulationService.fetchSimulation(simId1)

      expect(doc.status).toBe(SimulationStatus.RUNNING)
      expect(doc._id).toEqual(simId1)
    });

    it('should throw NotFound error when simulation does not exist', async () => {
      await createNewJob(SimulationStatus.RUNNING)

      await expect(SimulationService.fetchSimulation(mockObjectId())).rejects.toBeInstanceOf(NotFound)
    });
  });

  it('should group all the simulations by given jobId and return with their statuses', async () => {
    const job1 = await createNewJob(SimulationStatus.FINISHED, SimulationStatus.INQUEUE, SimulationStatus.FINISHED)
    await createNewJob(SimulationStatus.RUNNING, SimulationStatus.FINISHED);
    const groupedJobStatus = await SimulationService.groupSimulationsByJobId([job1._id]).exec();

    expect(groupedJobStatus).toHaveLength(1);
    expect(groupedJobStatus[0]._id).toEqual(job1._id);
    expect(groupedJobStatus[0].simulations[0].status).toBe("finished");
    expect(groupedJobStatus[0].simulations[1].status).toBe("in-queue");
    expect(groupedJobStatus[0].simulations[2].status).toBe("finished");
  })
})
