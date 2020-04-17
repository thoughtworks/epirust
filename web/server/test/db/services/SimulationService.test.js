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

  describe('markSimulationEnd', function () {
    it('should set simulation status to ended', async function () {
      const testSimulationId = 1234;
      await new Simulation({simulation_id: testSimulationId, status: SimulationStatus.RUNNING}).save();

      await SimulationService.markSimulationEnd(testSimulationId);

      const s = await Simulation.findOne({simulation_id: testSimulationId}).exec();
      expect(s.toObject().status).toBe(SimulationStatus.FINISHED);
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
});