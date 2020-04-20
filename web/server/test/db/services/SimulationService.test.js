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

describe('Simulation Service', function () {
  beforeAll(async () => await dbHandler.connect());
  afterEach(async () => await dbHandler.clearDatabase());
  afterAll(async () => await dbHandler.closeDatabase());

  describe('updateSimulationStatus', function () {
    it('should set simulation status to ended', async function () {
      const testSimulationId = 1234;
      await new Simulation({simulation_id: testSimulationId, status: SimulationStatus.RUNNING}).save();

      await SimulationService.updateSimulationStatus(testSimulationId, SimulationStatus.FAILED);

      const s = await Simulation.findOne({simulation_id: testSimulationId}).exec();
      expect(s.toObject().status).toBe(SimulationStatus.FAILED);
    });
  });

  describe('markSimulationEnd', function () {
    it('should set simulation status to ended', async function () {
      const testSimulationId = 1234;
      await new Simulation({simulation_id: testSimulationId, status: SimulationStatus.RUNNING}).save();

      await SimulationService.markGridConsumptionFinished(testSimulationId);

      const s = await Simulation.findOne({simulation_id: testSimulationId}).exec();
      expect(s.grid_consumption_finished).toEqual(true);
    });
  });

  describe('fetchSimulationStatus', function () {
    it('should return a simulation with its status if available in db', async function () {
      const simulationId = randomId();
      await (new Simulation({simulation_id: simulationId, status: SimulationStatus.RUNNING})).save()

      await SimulationService.fetchSimulationStatus(simulationId);

      const receivedSimulation = (await Simulation.findOne({simulation_id: simulationId}).exec()).toObject();
      expect(receivedSimulation.status).toBe(SimulationStatus.RUNNING);
      expect(receivedSimulation.simulation_id).toBe(simulationId)
    });

    it('should throw error with error message if no simulation exists',  function (done) {
      const simulationId = randomId();
      const expectedError = `Simulation with id: ${simulationId} not found`

      SimulationService.fetchSimulationStatus(simulationId)
        .catch(err => {
          expect(err.message).toBe(expectedError);
          done();
        })
    });
  });

  describe('fetchSimulation', function () {
    it('should return a simulation with projection fields specified', async function (done) {
      const simulationId = randomId();
      await (new Simulation({
        simulation_id: simulationId,
        status: SimulationStatus.RUNNING,
        config: {
          dummyField1: 'dummyValue1',
          dummyField2: 'dummyValue2'
        }
      })).save()

      SimulationService.fetchSimulation(simulationId, ['simulation_id', 'status', 'config.dummyField1'])
          .then(receivedSimulation => {
            expect(receivedSimulation.status).toBe(SimulationStatus.RUNNING);
            expect(receivedSimulation.simulation_id).toBe(simulationId);
            expect(receivedSimulation.config.dummyField1).toBe('dummyValue1');
            expect(receivedSimulation.config).not.toHaveProperty('dummyField2');
            done();
          });
    });

    it('should throw error with error message if no simulation exists',  function () {
      const simulationId = randomId();
      const expectedError = `Simulation with id: ${simulationId} not found`

      expect(SimulationService.fetchSimulation(simulationId)).rejects.toEqual(new Error(expectedError));
    });
  });

  const randomId = () => Math.random()
});