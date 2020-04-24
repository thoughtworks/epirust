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

const {handleRequest} = require("../../controllers/counts-io-controller");
jest.mock("../../db/services/SimulationService");
jest.mock("../../db/services/CountService");
const {fetchSimulation, fetchSimulationsWithJobId} = require('../../db/services/SimulationService')
const {fetchCountsInSimulation, aggregateSimulations} = require("../../db/services/CountService")
const {mockObjectId} = require('../helpers');
jest.useFakeTimers();

describe("Count controller", () => {
  let mockSocket;
  const placeHolderData = {
    infected_std: 0,
    susceptible_std: 0,
    quarantined_std: 0,
    recovered_std: 0,
    deceased_std: 0,
  }
  beforeEach(() => {
    jest.clearAllMocks();

    mockSocket = {
      on: jest.fn(),
      emit: jest.fn()
    };
  });

  function mockSimulationPromise(status, id = 'dummyId') {
    return {status, _id: id}
  }

  describe('when job contains single simulation', () => {

    it('should emit all counts if simulation has ended', (done) => {
      const testSimId = 1234;
      const jobId = mockObjectId()
      fetchSimulationsWithJobId.mockResolvedValueOnce([mockSimulationPromise('finished', testSimId)])
      fetchSimulation.mockResolvedValue(mockSimulationPromise('finished'));
      fetchCountsInSimulation.mockReturnValueOnce([{dummyKey: 'dummyValue', hour: 1}])

      handleRequest(mockSocket);
      expect(mockSocket.on).toHaveBeenCalledTimes(2);
      expect(mockSocket.on.mock.calls[0]).toHaveLength(2);
      expect(mockSocket.on.mock.calls[0][0]).toBe('get');
      mockSocket.on.mock.calls[0][1]({jobId: jobId.toString()});

      process.nextTick(() => {
        expect(fetchSimulationsWithJobId).toHaveBeenCalledTimes(1)
        expect(fetchSimulationsWithJobId).toHaveBeenCalledWith(jobId)

        expect(mockSocket.emit).toHaveBeenCalledTimes(2);
        expect(mockSocket.emit.mock.calls[0]).toEqual([
          'epidemicStats',
          {dummyKey: 'dummyValue', hour: 1, ...placeHolderData}
        ]);
        expect(mockSocket.emit.mock.calls[1]).toEqual(['epidemicStats', {"simulation_ended": true}]);
        expect(fetchSimulation).toHaveBeenCalledTimes(1);
        expect(fetchSimulation).toHaveBeenCalledWith(testSimId, ['status']);
        expect(fetchCountsInSimulation).toHaveBeenCalledTimes(1);
        expect(fetchCountsInSimulation).toHaveBeenCalledWith(testSimId, 0);
        done();
      })
    });

    it('should keep emitting all counts until simulation has ended', (done) => {
      const docPromises = [
        mockSimulationPromise('unfinished'),
        mockSimulationPromise('finished')
      ];
      const testSimId = 1234;
      const jobId = mockObjectId()

      const cursors = [[{dummyKey: 'dummyValue', hour: 1}], [{dummyKey: 'dummyValue2', hour: 2}]];
      fetchSimulationsWithJobId.mockResolvedValueOnce([mockSimulationPromise('finished', testSimId)])
      fetchSimulation.mockImplementation(() => Promise.resolve(docPromises.shift()))
      fetchCountsInSimulation.mockImplementation(() => cursors.shift())

      handleRequest(mockSocket);
      expect(mockSocket.on).toHaveBeenCalledTimes(2);
      expect(mockSocket.on.mock.calls[0]).toHaveLength(2);
      expect(mockSocket.on.mock.calls[0][0]).toBe('get');
      mockSocket.on.mock.calls[0][1]({jobId: jobId.toString()});

      process.nextTick(() => {
        expect(fetchSimulationsWithJobId).toHaveBeenCalledTimes(1)
        expect(fetchSimulationsWithJobId).toHaveBeenCalledWith(jobId)

        expect(mockSocket.emit).toHaveBeenCalledTimes(3);
        expect(mockSocket.emit.mock.calls[0]).toEqual([
          'epidemicStats',
          {dummyKey: 'dummyValue', hour: 1, ...placeHolderData}
        ]);
        expect(mockSocket.emit.mock.calls[1]).toEqual([
          'epidemicStats',
          {dummyKey: 'dummyValue2', hour: 2, ...placeHolderData}
        ]);
        expect(mockSocket.emit.mock.calls[2]).toEqual(['epidemicStats', {"simulation_ended": true}]);
        expect(fetchSimulation).toHaveBeenCalledTimes(2);
        expect(fetchSimulation.mock.calls[0]).toEqual([testSimId, ['status']]);
        expect(fetchSimulation.mock.calls[1]).toEqual([testSimId, ['status']]);
        expect(fetchCountsInSimulation).toHaveBeenCalledTimes(2);
        expect(fetchCountsInSimulation.mock.calls[0]).toEqual([testSimId, 0]);
        expect(fetchCountsInSimulation.mock.calls[1]).toEqual([testSimId, 1]);
        done();
      });
    });

    it('should send simulation ended if simulation has failed', (done) => {
      const testSimId = 1234;
      const jobId = mockObjectId()

      fetchSimulation.mockResolvedValue(mockSimulationPromise('failed'));
      fetchSimulationsWithJobId.mockResolvedValueOnce([mockSimulationPromise('finished', testSimId)])
      fetchCountsInSimulation.mockReturnValueOnce([{dummyKey: 'dummyValue', hour: 1}])

      handleRequest(mockSocket);
      expect(mockSocket.on).toHaveBeenCalledTimes(2);
      expect(mockSocket.on.mock.calls[0]).toHaveLength(2);
      expect(mockSocket.on.mock.calls[0][0]).toBe('get');
      mockSocket.on.mock.calls[0][1]({jobId: jobId.toString()});


      process.nextTick(() => {
        expect(fetchSimulationsWithJobId).toHaveBeenCalledTimes(1)
        expect(fetchSimulationsWithJobId).toHaveBeenCalledWith(jobId)

        expect(mockSocket.emit).toHaveBeenCalledTimes(2);
        expect(mockSocket.emit.mock.calls[0]).toEqual([
          'epidemicStats',
          {dummyKey: 'dummyValue', hour: 1, ...placeHolderData}
        ]);
        expect(mockSocket.emit.mock.calls[1]).toEqual(['epidemicStats', {"simulation_ended": true}]);
        done();
      })
    });
  });

  describe('when job contains multiple simulations', () => {
    it('should emit all the aggregated counts if all the simulations in the job has ended', (done) => {
      const simId1 = "3457634"
      const simId2 = "76543e4"
      const jobId = mockObjectId();
      fetchSimulationsWithJobId.mockResolvedValue([
        mockSimulationPromise('finished', simId1),
        mockSimulationPromise('finished', simId2)
      ]);

      aggregateSimulations.mockReturnValueOnce([1, 2])

      handleRequest(mockSocket);
      expect(mockSocket.on).toHaveBeenCalledTimes(2);
      expect(mockSocket.on.mock.calls[0]).toHaveLength(2);
      expect(mockSocket.on.mock.calls[0][0]).toBe('get');
      mockSocket.on.mock.calls[0][1]({jobId});

      process.nextTick(() => {
        expect(fetchSimulationsWithJobId).toHaveBeenCalledTimes(2)
        expect(fetchSimulationsWithJobId).toHaveBeenNthCalledWith(1, jobId)
        expect(fetchSimulationsWithJobId).toHaveBeenNthCalledWith(2, jobId)

        expect(aggregateSimulations).toHaveBeenCalledTimes(1);
        expect(aggregateSimulations).toHaveBeenCalledWith([simId1, simId2])

        expect(mockSocket.emit).toHaveBeenCalledTimes(3)
        expect(mockSocket.emit).toHaveBeenNthCalledWith(1, 'epidemicStats', 1)
        expect(mockSocket.emit).toHaveBeenNthCalledWith(2, 'epidemicStats', 2)
        expect(mockSocket.emit).toHaveBeenNthCalledWith(3, 'epidemicStats', {simulation_ended: true})
        done();
      })


    });

    it('should retry until all the simulations are finished',  (done) => {
      const simId1 = "3457634"
      const simId2 = "76543e4"
      const jobId = mockObjectId();
      fetchSimulationsWithJobId
        .mockResolvedValueOnce([
          mockSimulationPromise('in-progress', simId1),
          mockSimulationPromise('in-queue', simId2)
        ])
        .mockResolvedValueOnce([
          mockSimulationPromise('finished', simId1),
          mockSimulationPromise('in-progress', simId2)
        ])
        .mockResolvedValueOnce([
          mockSimulationPromise('finished', simId1),
          mockSimulationPromise('finished', simId2)
        ]);

      aggregateSimulations.mockReturnValueOnce([1, 2])

      handleRequest(mockSocket);
      mockSocket.on.mock.calls[0][1]({jobId});

      process.nextTick(async () => {
        jest.runAllTimers();
        await flushPromises();
        expect(fetchSimulationsWithJobId).toHaveBeenCalledTimes(3)
        expect(fetchSimulationsWithJobId).toHaveBeenNthCalledWith(1, jobId)
        expect(fetchSimulationsWithJobId).toHaveBeenNthCalledWith(2, jobId)
        expect(fetchSimulationsWithJobId).toHaveBeenNthCalledWith(3, jobId)

        expect(aggregateSimulations).toHaveBeenCalledTimes(1);
        expect(aggregateSimulations).toHaveBeenCalledWith([simId1, simId2])
        done();
      })
    });

    it('should send back error if any of the job fails', (done) => {
      const simId1 = "3457634"
      const simId2 = "76543e4"
      const jobId = mockObjectId();
      fetchSimulationsWithJobId
        .mockResolvedValueOnce([
          mockSimulationPromise('in-progress', simId1),
          mockSimulationPromise('in-queue', simId2)
        ])
        .mockResolvedValueOnce([
          mockSimulationPromise('finished', simId1),
          mockSimulationPromise('failed', simId2)
        ])

      aggregateSimulations.mockReturnValueOnce([1, 2])

      handleRequest(mockSocket);
      mockSocket.on.mock.calls[0][1]({jobId});

      process.nextTick( () => {
        expect(mockSocket.emit).toHaveBeenCalledTimes(1)
        expect(mockSocket.emit).toHaveBeenCalledWith('error', {"message": "One of the simulations failed to finish"})
        done();
      })
    });
  });

  it('should console on connection closed', () => {
    global.console = {
      log: jest.fn()
    };

    handleRequest(mockSocket);

    expect(mockSocket.on).toHaveBeenCalledTimes(2);
    expect(mockSocket.on.mock.calls[1]).toHaveLength(2);
    expect(mockSocket.on.mock.calls[1][0]).toEqual('disconnect');
    const disconnectCallback = mockSocket.on.mock.calls[1][1];

    disconnectCallback("dummyReason");
    expect(console.log).toHaveBeenCalledTimes(1);
    expect(console.log.mock.calls[0]).toEqual(["Disconnect", "dummyReason"]);
  });

  const flushPromises = () => new Promise(setImmediate);
});