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
const {Simulation, SimulationStatus}  = require('../../../db/models/Simulation');
const {mockObjectId} = require('../../helpers');

describe('Simulation Service', () => {
  describe('updateSimulationStatus', () => {
    it('should set simulation status to ended', async () => {
      const {_id: simulationId} = await createNewSimulation(SimulationStatus.RUNNING)

      await SimulationService.updateSimulationStatus(simulationId, SimulationStatus.FAILED);

      const s = await Simulation.findOne({_id: simulationId}).exec();
      expect(s.toObject().status).toBe(SimulationStatus.FAILED);
    });
  });

  describe('markGridConsumptionFinished', () => {
    it('should set grid consumption finished to true', async () => {
      const {_id: simulationId} = await createNewSimulation(SimulationStatus.RUNNING)

      await SimulationService.markGridConsumptionFinished(simulationId);

      const s = await Simulation.findOne({_id: simulationId}).exec();
      expect(s.grid_consumption_finished).toEqual(true);
    });
  });

  describe('fetchSimulation', () => {
    it('should return a simulation with projection fields specified', async () => {
      const {_id: simulationId} = await createNewSimulation(SimulationStatus.RUNNING)

      const receivedSimulation = await SimulationService.fetchSimulation(simulationId, ['_id', 'status']);

      expect(receivedSimulation.status).toBe(SimulationStatus.RUNNING);
      expect(receivedSimulation._id).toEqual(simulationId);
      expect(receivedSimulation).not.toHaveProperty('job_id');
    });

    it('should return a simulation with all fields if not specified projection fields', async () => {
      const {_id: simulationId, job_id} = await createNewSimulation(SimulationStatus.RUNNING)

      const receivedSimulation = await SimulationService.fetchSimulation(simulationId);

      expect(receivedSimulation.status).toBe(SimulationStatus.RUNNING);
      expect(receivedSimulation._id).toEqual(simulationId);
      expect(receivedSimulation.job_id).toEqual(job_id)
    });

    it('should throw error with error message if no simulation exists',  () => {
      const simulationId = mockObjectId();
      const expectedError = `Simulation with id: ${simulationId} not found`;

      expect(SimulationService.fetchSimulation(simulationId)).rejects.toEqual(new Error(expectedError));
    });
  });

  describe('saveSimulation', () => {
    it('should save simulation in database', async () => {
      const jobId = mockObjectId();
      const simulation = {status: SimulationStatus.RUNNING, job_id: jobId};

      await SimulationService.saveSimulation(simulation);

      const savedSimulation = await Simulation.findOne({}, {__v: 0, _id: 0}).exec();
      expect(savedSimulation.toObject()).toEqual(simulation);
    });
  });

  describe('fetchSimulationsWithJobId', () => {
    it('should return all the simulations with given jobId', async () => {
      const jobId = mockObjectId();
      const sim1 = await createNewSimulation(SimulationStatus.INQUEUE, jobId)
      const sim2 = await createNewSimulation(SimulationStatus.INQUEUE, jobId)

      const simulations = (await SimulationService.fetchSimulationsWithJobId(jobId));
      expect(simulations).toHaveLength(2)
      expect(simulations[0]._id).toEqual(sim1._id)
      expect(simulations[1]._id).toEqual(sim2._id)
    });
  });

  describe('groupSimulationsByJob', () => {
    it('should group all the simulations by jobId and return with their statuses', async() => {
      const jobId1 = mockObjectId();
      const jobId2 = mockObjectId();
      const simId1 = (await createNewSimulation(SimulationStatus.FINISHED, jobId1)).toObject()._id
      const simId2 = (await createNewSimulation(SimulationStatus.INQUEUE, jobId1)).toObject()._id
      const simId3 = (await createNewSimulation(SimulationStatus.FINISHED, jobId1)).toObject()._id
      const simId4 = (await createNewSimulation(SimulationStatus.FINISHED, jobId2)).toObject()._id
      const simId5 = (await createNewSimulation(SimulationStatus.RUNNING, jobId2)).toObject()._id

      const cursor = SimulationService.groupSimulationsByJob();

      const jobs = []
      for await (const job of cursor) {
        jobs.push(job);
      }

      expect(jobs).toHaveLength(2);
      expect(jobs[0]).toEqual({
        _id: jobId1,
        simulations: [
          {status: SimulationStatus.FINISHED, id: simId1},
          {status: SimulationStatus.INQUEUE, id: simId2},
          {status: SimulationStatus.FINISHED, id: simId3},
          ]});
      expect(jobs[1]).toEqual({
        _id: jobId2,
        simulations: [
          {status: SimulationStatus.FINISHED, id: simId4},
          {status: SimulationStatus.RUNNING, id: simId5},
        ]});
    });
  });

  const createNewSimulation = (simulationStatus, jobId = mockObjectId()) => {
    return new Simulation({job_id: jobId, status: simulationStatus}).save();
  }

  beforeAll(async () => await dbHandler.connect());
  afterEach(async () => await dbHandler.clearDatabase());
  afterAll(async () => await dbHandler.closeDatabase());
});