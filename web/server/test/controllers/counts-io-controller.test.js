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

const {handleRequest: handleCountsRequest} = require("../../controllers/counts-io-controller");
jest.mock("../../db/models/Simulation");
jest.mock("../../db/models/Count");
const {Simulation} = require("../../db/models/Simulation");
const Count = require("../../db/models/Count");

describe("Count controller", () => {
  let mockSocket;
  beforeEach(() => {
    jest.clearAllMocks();

    mockSocket = {
      on: jest.fn(),
      emit: jest.fn()
    };
  });

  function mockSimulationPromise(status) {
    return {
      then: async function (fn) {
        await fn({status, simulation_id: 'dummyId'})
      }
    }
  }

  it('should emit all counts if simulation has ended', (done) => {
    const mockPromise = mockSimulationPromise('finished');
    const mockExec = jest.fn().mockReturnValue(mockPromise);
    Simulation.findOne.mockReturnValue({'exec': mockExec});
    let mockCursor = jest.fn().mockReturnValueOnce([{dummyKey: 'dummyValue', hour: 1}]);
    let mockSkip = jest.fn().mockReturnValueOnce({cursor: mockCursor});
    Count.find.mockReturnValueOnce({skip: mockSkip});

    handleCountsRequest(mockSocket);
    expect(mockSocket.on).toHaveBeenCalledTimes(2);
    expect(mockSocket.on.mock.calls[0]).toHaveLength(2);
    expect(mockSocket.on.mock.calls[0][0]).toBe('simulation_id');
    let testSimId = "1234";
    mockSocket.on.mock.calls[0][1](testSimId);

    process.nextTick(() => {
      expect(mockSocket.emit).toHaveBeenCalledTimes(2);
      expect(mockSocket.emit.mock.calls[0]).toEqual([
        'epidemicStats',
        {dummyKey: 'dummyValue', hour: 1}
      ]);
      expect(mockSocket.emit.mock.calls[1]).toEqual(['epidemicStats', {"simulation_ended": true}]);
      expect(Simulation.findOne).toHaveBeenCalledTimes(1);
      expect(Simulation.findOne.mock.calls[0]).toEqual([{simulation_id: 1234}, {status: 1}]);
      expect(Count.find).toHaveBeenCalledTimes(1);
      expect(mockSkip).toHaveBeenCalledTimes(1);
      expect(mockSkip).toBeCalledWith(0);
      expect(Count.find.mock.calls[0]).toEqual([{simulation_id: 1234, hour: {$mod: [24, 1]}}, {}, {sort: {'hour': 1}}]);
      done();
    })
  });

  it('should keep emitting all counts until simulation has ended', (done) => {
    const docPromises = [
      mockSimulationPromise('unfinished'),
      mockSimulationPromise('finished')
    ];
    const cursors = [[{dummyKey: 'dummyValue', hour: 1}], [{dummyKey: 'dummyValue2', hour: 2}]];
    const mockExec = jest.fn(() => docPromises.shift());
    Simulation.findOne.mockReturnValue({'exec': mockExec});
    const mockCursor = jest.fn(() => cursors.shift());
    const mockSkip = jest.fn().mockReturnValue({cursor: mockCursor});
    Count.find.mockReturnValue({skip: mockSkip});

    handleCountsRequest(mockSocket);
    expect(mockSocket.on).toHaveBeenCalledTimes(2);
    expect(mockSocket.on.mock.calls[0]).toHaveLength(2);
    expect(mockSocket.on.mock.calls[0][0]).toBe('simulation_id');
    let testSimId = "1234";
    mockSocket.on.mock.calls[0][1](testSimId);

    process.nextTick(() => {
      expect(mockSocket.emit).toHaveBeenCalledTimes(3);
      expect(mockSocket.emit.mock.calls[0]).toEqual([
        'epidemicStats',
        {dummyKey: 'dummyValue', hour: 1}
      ]);
      expect(mockSocket.emit.mock.calls[1]).toEqual([
        'epidemicStats',
        {dummyKey: 'dummyValue2', hour: 2}
      ]);
      expect(mockSocket.emit.mock.calls[2]).toEqual(['epidemicStats', {"simulation_ended": true}]);
      expect(Simulation.findOne).toHaveBeenCalledTimes(2);
      expect(Simulation.findOne.mock.calls[0]).toEqual([{simulation_id: 1234}, {status: 1}]);
      expect(Simulation.findOne.mock.calls[1]).toEqual([{simulation_id: 1234}, {status: 1}]);
      expect(Count.find).toHaveBeenCalledTimes(2);
      expect(Count.find.mock.calls[0]).toEqual([
        {simulation_id: 1234, hour: {$mod: [24, 1]}},
        {},
        {sort: {'hour': 1}}
      ]);
      expect(Count.find.mock.calls[1]).toEqual([
        {simulation_id: 1234, hour: {$mod: [24, 1]}},
        {},
        {sort: {'hour': 1}}
      ]);
      expect(mockSkip).toHaveBeenCalledTimes(2);
      expect(mockSkip).toHaveBeenNthCalledWith(1, 0);
      expect(mockSkip).toHaveBeenNthCalledWith(2, 1);
      done();
    });
  });

  it('should send simulation ended if simulation has failed', (done) => {
    const mockPromise = mockSimulationPromise('failed');
    const mockExec = jest.fn().mockReturnValue(mockPromise);
    Simulation.findOne.mockReturnValue({'exec': mockExec});
    let mockCursor = jest.fn().mockReturnValueOnce([{dummyKey: 'dummyValue', hour: 1}]);
    let mockSkip = jest.fn().mockReturnValueOnce({cursor: mockCursor});
    Count.find.mockReturnValueOnce({skip: mockSkip});

    handleCountsRequest(mockSocket);
    expect(mockSocket.on).toHaveBeenCalledTimes(2);
    expect(mockSocket.on.mock.calls[0]).toHaveLength(2);
    expect(mockSocket.on.mock.calls[0][0]).toBe('simulation_id');
    let testSimId = "1234";
    mockSocket.on.mock.calls[0][1](testSimId);

    process.nextTick(() => {
      expect(mockSocket.emit).toHaveBeenCalledTimes(2);
      expect(mockSocket.emit.mock.calls[0]).toEqual([
        'epidemicStats',
        {dummyKey: 'dummyValue', hour: 1}
      ]);
      expect(mockSocket.emit.mock.calls[1]).toEqual(['epidemicStats', {"simulation_ended": true}]);
      expect(Simulation.findOne).toHaveBeenCalledTimes(1);
      expect(Simulation.findOne.mock.calls[0]).toEqual([{simulation_id: 1234}, {status: 1}]);
      expect(Count.find).toHaveBeenCalledTimes(1);
      expect(mockSkip).toHaveBeenCalledTimes(1);
      expect(mockSkip).toBeCalledWith(0);
      expect(Count.find.mock.calls[0]).toEqual([{simulation_id: 1234, hour: {$mod: [24, 1]}}, {}, {sort: {'hour': 1}}]);
      done();
    })
  });

  it('should console on connection closed', () => {
    const mockPromise = mockSimulationPromise('finished');
    const mockExec = jest.fn().mockReturnValue(mockPromise);
    Simulation.findOne.mockReturnValue({'exec': mockExec});
    let mockCursor = jest.fn().mockReturnValueOnce([]);
    const mockSkip = jest.fn().mockReturnValueOnce({cursor: mockCursor});
    Count.find.mockReturnValueOnce({skip: mockSkip});
    global.console = {
      log: jest.fn()
    };

    handleCountsRequest(mockSocket);

    expect(mockSocket.on).toHaveBeenCalledTimes(2);
    expect(mockSocket.on.mock.calls[1]).toHaveLength(2);
    expect(mockSocket.on.mock.calls[1][0]).toEqual('disconnect');
    const disconnectCallback = mockSocket.on.mock.calls[1][1];
    disconnectCallback("dummyReason");
    expect(console.log).toHaveBeenCalledTimes(1);
    expect(console.log.mock.calls[0]).toEqual(["Disconnect", "dummyReason"]);
  });
});