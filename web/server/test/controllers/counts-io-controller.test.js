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
jest.mock("../../db/models/Count");
jest.mock("../../db/services/SimulationService");
const {Count} = require("../../db/models/Count");
const {fetchSimulation} = require('../../db/services/SimulationService')

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
    return {status, simulation_id: 'dummyId'}
  }

  it('should emit all counts if simulation has ended', (done) => {
    fetchSimulation.mockResolvedValue(mockSimulationPromise('finished'));
    let mockCursor = jest.fn().mockReturnValueOnce([{dummyKey: 'dummyValue', hour: 1}]);
    let mockSkip = jest.fn().mockReturnValueOnce({cursor: mockCursor});
    Count.find.mockReturnValueOnce({skip: mockSkip});

    handleCountsRequest(mockSocket);
    expect(mockSocket.on).toHaveBeenCalledTimes(2);
    expect(mockSocket.on.mock.calls[0]).toHaveLength(2);
    expect(mockSocket.on.mock.calls[0][0]).toBe('simulation_id');
    const testSimId = "1234";
    mockSocket.on.mock.calls[0][1](testSimId);

    process.nextTick(() => {
      expect(mockSocket.emit).toHaveBeenCalledTimes(2);
      expect(mockSocket.emit.mock.calls[0]).toEqual([
        'epidemicStats',
        {dummyKey: 'dummyValue', hour: 1}
      ]);
      expect(mockSocket.emit.mock.calls[1]).toEqual(['epidemicStats', {"simulation_ended": true}]);
      expect(fetchSimulation).toHaveBeenCalledTimes(1);
      expect(fetchSimulation).toHaveBeenCalledWith(1234, ['status']);
      expect(Count.find).toHaveBeenCalledTimes(1);
      expect(mockSkip).toHaveBeenCalledTimes(1);
      expect(mockSkip).toBeCalledWith(0);
      expect(Count.find.mock.calls[0][0]).toEqual({"simulation_id": 1234}, {}, {"$sort": 1})
      done();
    })
  });

  it('should keep emitting all counts until simulation has ended', (done) => {
    const docPromises = [
      mockSimulationPromise('unfinished'),
      mockSimulationPromise('finished')
    ];
    const cursors = [[{dummyKey: 'dummyValue', hour: 1}], [{dummyKey: 'dummyValue2', hour: 2}]];
    fetchSimulation.mockImplementation(() => Promise.resolve(docPromises.shift()))
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
      expect(fetchSimulation).toHaveBeenCalledTimes(2);
      expect(fetchSimulation.mock.calls[0]).toEqual([1234, ['status']]);
      expect(fetchSimulation.mock.calls[1]).toEqual([1234, ['status']]);
      expect(Count.find).toHaveBeenCalledTimes(2);
      expect(Count.find.mock.calls[0]).toEqual([{"simulation_id": 1234},{},{"$sort": 1}]);
      expect(Count.find.mock.calls[1]).toEqual([{"simulation_id": 1234},{},{"$sort": 1}]);
      expect(mockSkip).toHaveBeenCalledTimes(2);
      expect(mockSkip).toHaveBeenNthCalledWith(1, 0);
      expect(mockSkip).toHaveBeenNthCalledWith(2, 1);
      done();
    });
  });

  it('should send simulation ended if simulation has failed', (done) => {
    fetchSimulation.mockResolvedValue(mockSimulationPromise('failed'));
    const mockCursor = jest.fn().mockReturnValueOnce([{dummyKey: 'dummyValue', hour: 1}]);
    const mockSkip = jest.fn().mockReturnValueOnce({cursor: mockCursor});
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
      done();
    })
  });

  it('should console on connection closed', () => {
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