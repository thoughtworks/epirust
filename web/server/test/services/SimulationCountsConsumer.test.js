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

const {SimulationCountsConsumer} = require('../../services/SimulationCountsConsumer');
jest.mock("../../services/kafka");
jest.mock("../../db/models/Simulation");
jest.mock("../../db/models/Count");
jest.mock('../../db/services/SimulationService');
jest.mock('../../db/services/CountService');
const {KafkaGroupConsumer} = require('../../services/kafka');
const {Simulation} = require('../../db/models/Simulation');
const {Count} = require('../../db/models/Count');
const SimulationService = require('../../db/services/SimulationService');
const CountService = require('../../db/services/CountService');

describe('Simulation Counts Consumer', () => {
  beforeEach(() => {
    jest.clearAllMocks()
  });

  it('should initiate the kafka group consumer with with correct config', () => {
    new SimulationCountsConsumer();
    let KafkaGroupConsumerConfig = ['localhost:9092', 'counts_updated', 'dev_server_consumer'];

    expect(KafkaGroupConsumer).toHaveBeenCalledTimes(1);
    expect(KafkaGroupConsumer.mock.calls[0]).toEqual(KafkaGroupConsumerConfig);
  });

  it('should updated the status of simulation to be finished', async () => {
    KafkaGroupConsumer.mockReturnValueOnce({consumerStream: [{value: '{"simulation_ended":true}', key: "123"}]});
    const simulationCountsConsumer = new SimulationCountsConsumer();

    await simulationCountsConsumer.start();

    expect(SimulationService.updateSimulationStatus).toHaveBeenCalledTimes(1);
    expect(SimulationService.updateSimulationStatus).toHaveBeenCalledWith(123);
  });

  it('should store counts if not ended message', async () => {
    const countMessage = {"infected":78, "hour": 12};
    KafkaGroupConsumer.mockReturnValueOnce({
      consumerStream: [{value: JSON.stringify(countMessage), key: "123"}]
    });
    const simulationCountsConsumer = new SimulationCountsConsumer();

    await simulationCountsConsumer.start();

    expect(CountService.upsertCount).toHaveBeenCalledTimes(1);
    expect(CountService.upsertCount).toHaveBeenCalledWith(123, countMessage)
  });

  it('should update the status of simulation as running when the first count is recieved', async (done) => {
    KafkaGroupConsumer.mockReturnValueOnce({
      consumerStream: [{
        value: '{"dummyKey":"dummyValue", "hour":1}',
        key: "123"
      }]
    });
    const execMock = jest.fn();
    Simulation.updateOne.mockReturnValueOnce({exec: execMock});
    const countExecMock = jest.fn();
    Count.updateOne.mockReturnValueOnce({exec: countExecMock});
    const simulationCountsConsumer = new SimulationCountsConsumer();

    await simulationCountsConsumer.start();

    expect(Simulation.updateOne).toHaveBeenCalledTimes(1);
    expect(Simulation.updateOne.mock.calls[0]).toEqual([
      {simulation_id: 123},
      {status: "running"},
      {"upsert": true},
    ]);
    process.nextTick(() => {
      expect(execMock).toHaveBeenCalledTimes(1);
      done();
    });
  });

  it('should write intervention in db', async () => {
    const intervention = {"hour": 12, "intervention": "lockdown", "data": {"status": "locked_down"}}
    KafkaGroupConsumer.mockReturnValueOnce({
      consumerStream: [{value: JSON.stringify(intervention), key: "123"}]
    });
    const simulationCountsConsumer = new SimulationCountsConsumer();

    await simulationCountsConsumer.start();

    expect(CountService.addIntervention).toHaveBeenCalledTimes(1);
    expect(CountService.addIntervention).toHaveBeenCalledWith(123, intervention);
  })
});



