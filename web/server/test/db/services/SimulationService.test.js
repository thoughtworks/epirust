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

describe('Simulation Service', function () {
  describe('updateSimulationStatus', function () {
    it('should set simulation status to ended', async function () {
      const {_id: simulationId} = await createNewSimulation(SimulationStatus.RUNNING)

      await SimulationService.updateSimulationStatus(simulationId, SimulationStatus.FAILED);

      const s = await Simulation.findOne({_id: simulationId}).exec();
      expect(s.toObject().status).toBe(SimulationStatus.FAILED);
    });
  });

  describe('markGridConsumptionFinished', function () {
    it('should set grid consumption finished to true', async function () {
      const {_id: simulationId} = await createNewSimulation(SimulationStatus.RUNNING)

      await SimulationService.markGridConsumptionFinished(simulationId);

      const s = await Simulation.findOne({_id: simulationId}).exec();
      expect(s.grid_consumption_finished).toEqual(true);
    });
  });

  describe('fetchSimulation', function () {
    it('should return a simulation with projection fields specified', async function () {
      const {_id: simulationId} = await createNewSimulation(SimulationStatus.RUNNING)

      const receivedSimulation = await SimulationService.fetchSimulation(simulationId, ['_id', 'status']);

      expect(receivedSimulation.status).toBe(SimulationStatus.RUNNING);
      expect(receivedSimulation._id).toEqual(simulationId);
      expect(receivedSimulation).not.toHaveProperty('job_id');
    });

    it('should return a simulation with all fields if not specified projection fields', async function () {
      const {_id: simulationId, job_id} = await createNewSimulation(SimulationStatus.RUNNING)

      const receivedSimulation = await SimulationService.fetchSimulation(simulationId);

      expect(receivedSimulation.status).toBe(SimulationStatus.RUNNING);
      expect(receivedSimulation._id).toEqual(simulationId);
      expect(receivedSimulation.job_id).toEqual(job_id)
    });

    it('should throw error with error message if no simulation exists',  function () {
      const simulationId = mockObjectId();
      const expectedError = `Simulation with id: ${simulationId} not found`;

      expect(SimulationService.fetchSimulation(simulationId)).rejects.toEqual(new Error(expectedError));
    });
  });

  describe('saveSimulation', function () {
    it('should save simulation in database', async function () {
      const jobId = mockObjectId();
      const simulation = {status: SimulationStatus.RUNNING, job_id: jobId};

      await SimulationService.saveSimulation(simulation);

      const savedSimulation = await Simulation.findOne({}, {__v: 0, _id: 0}).exec();
      expect(savedSimulation.toObject()).toEqual(simulation);
    });
  });

  const createNewSimulation = (simulationStatus) => {
    const jobId = mockObjectId();
    return new Simulation({job_id: jobId, status: simulationStatus}).save();
  }

  beforeAll(async () => await dbHandler.connect());
  afterEach(async () => await dbHandler.clearDatabase());
  afterAll(async () => await dbHandler.closeDatabase());
});