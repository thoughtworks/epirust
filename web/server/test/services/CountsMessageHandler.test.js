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

const {CountsMessageHandler} = require('../../services/CountsMessageHandler');
jest.mock("../../db/models/Simulation");
jest.mock("../../db/models/Count");
jest.mock('../../db/services/SimulationService');
jest.mock('../../db/services/CountService');
const {SimulationStatus} = require('../../db/models/Simulation');
const SimulationService = require('../../db/services/SimulationService');
const CountService = require('../../db/services/CountService');
const {mockObjectId} = require('../helpers');

describe('Simulation Counts Consumer', () => {
  beforeEach(() => {
    jest.clearAllMocks()
  });

  it('should updated the status of simulation to be finished', async () => {
    const simulationId = mockObjectId();
    const data = {value: '{"simulation_ended":true}', key: simulationId.toString()};
    const simulationCountsConsumer = new CountsMessageHandler();

    await simulationCountsConsumer.handleMessage(data);

    expect(SimulationService.updateSimulationStatus).toHaveBeenCalledTimes(1);
    expect(SimulationService.updateSimulationStatus).toHaveBeenCalledWith(simulationId, SimulationStatus.FINISHED);
  });

  it('should store counts if not ended message', async () => {
    const countMessage = {"infected":78, "hour": 12};
    const simulationId = mockObjectId();
    const data = {value: JSON.stringify(countMessage), key: simulationId.toString()};
    const simulationCountsConsumer = new CountsMessageHandler();

    await simulationCountsConsumer.handleMessage(data);

    expect(CountService.upsertCount).toHaveBeenCalledTimes(1);
    expect(CountService.upsertCount).toHaveBeenCalledWith(simulationId, countMessage)
  });

  it('should update the status of simulation as running when the first count is recieved', async (done) => {
    const simulationId = mockObjectId();
    const data = {
      value: '{"dummyKey":"dummyValue", "hour":1}',
      key: simulationId.toString()
    };
    const simulationCountsConsumer = new CountsMessageHandler();

    await simulationCountsConsumer.handleMessage(data);

    process.nextTick(() => {
      expect(SimulationService.updateSimulationStatus).toHaveBeenCalledTimes(1);
      expect(SimulationService.updateSimulationStatus).toHaveBeenCalledWith(simulationId, SimulationStatus.RUNNING);
      done();
    });
  });

  it('should write intervention in db', async () => {
    const intervention = {"hour": 12, "intervention": "lockdown", "data": {"status": "locked_down"}}
    const simulationId = mockObjectId();
    const data = {value: JSON.stringify(intervention), key: simulationId.toString()};
    const simulationCountsConsumer = new CountsMessageHandler();

    await simulationCountsConsumer.handleMessage(data)

    expect(CountService.addIntervention).toHaveBeenCalledTimes(1);
    expect(CountService.addIntervention).toHaveBeenCalledWith(simulationId, intervention);
  })
});



