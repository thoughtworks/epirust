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
const {fetchSimulation} = require('../../db/services/SimulationService')
const {fetchCountsInSimulation, aggregateSimulations} = require("../../db/services/CountService")
const {mockObjectId} = require('../helpers');
jest.mock("../../db/services/JobService");
const {fetchJob} = require("../../db/services/JobService")
jest.useFakeTimers();

describe("Count controller", () => {
  let mockSocket;
  const placeHolderData = {
    infected_std: 0,
    susceptible_std: 0,
    hospitalized_std: 0,
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

  function mockSimulation(status, id = 'dummyId') {
    return {status, _id: id}
  }

  function MockMongoDocument(data) {
    this._doc = data;
    this.toObject = function () {
      return this._doc
    }
  }

  describe('when job contains single simulation', () => {

    it('should emit all counts if simulation has ended', (done) => {
      const testSimId = 1234;
      const jobId = mockObjectId()
      fetchJob.mockResolvedValueOnce({_id: jobId, simulations: [mockSimulation('finished', testSimId)]})
      fetchSimulation.mockResolvedValue(mockSimulation('finished'));
      const job = {dummyKey: 'dummyValue', hour: 1};
      fetchCountsInSimulation.mockReturnValueOnce([new MockMongoDocument(job)])

      handleRequest(mockSocket, jobId.toString());

      process.nextTick(() => {
        expect(fetchJob).toHaveBeenCalledTimes(1)
        expect(fetchJob).toHaveBeenCalledWith(jobId)

        expect(mockSocket.emit).toHaveBeenCalledTimes(2);
        expect(mockSocket.emit.mock.calls[0]).toEqual([
          'epidemicStats',
          {dummyKey: 'dummyValue', hour: 1, ...placeHolderData}
        ]);
        expect(mockSocket.emit.mock.calls[1]).toEqual(['epidemicStats', {"simulation_ended": true}]);
        expect(fetchSimulation).toHaveBeenCalledTimes(1);
        expect(fetchSimulation).toHaveBeenCalledWith(testSimId);
        expect(fetchCountsInSimulation).toHaveBeenCalledTimes(1);
        expect(fetchCountsInSimulation).toHaveBeenCalledWith(testSimId, 0);
        done();
      })
    });

    it('should keep emitting all counts until simulation has ended', (done) => {
      const testSimId = 1234;
      const jobId = mockObjectId()

      fetchJob.mockResolvedValueOnce({_id: jobId, simulations: [mockSimulation('finished', testSimId)]})

      const countsData = [{dummyKey: 'dummyValue', hour: 1}, {dummyKey: 'dummyValue2', hour: 2}];
      fetchCountsInSimulation.mockImplementation(() => [new MockMongoDocument(countsData.shift())])

      fetchSimulation
        .mockResolvedValueOnce(mockSimulation('running'))
        .mockResolvedValueOnce(mockSimulation('finished'))

      handleRequest(mockSocket, jobId.toString());

      process.nextTick(async () => {
        await flushPromises()
        expect(fetchJob).toHaveBeenCalledTimes(1)
        expect(fetchJob).toHaveBeenCalledWith(jobId)

        expect(mockSocket.emit).toHaveBeenCalledTimes(3);
        expect(mockSocket.emit.mock.calls[0]).toEqual(['epidemicStats', {
          ...placeHolderData,
          dummyKey: 'dummyValue',
          hour: 1,
        }]);
        expect(mockSocket.emit.mock.calls[1]).toEqual(['epidemicStats', {
          ...placeHolderData,
          dummyKey: 'dummyValue2',
          hour: 2
        }]);
        expect(mockSocket.emit.mock.calls[2]).toEqual(['epidemicStats', {simulation_ended: true}]);

        expect(fetchSimulation).toHaveBeenCalledTimes(2);
        expect(fetchSimulation.mock.calls).toEqual([[testSimId], [testSimId]]);

        expect(fetchCountsInSimulation).toHaveBeenCalledTimes(2);
        expect(fetchCountsInSimulation.mock.calls).toEqual([
          [testSimId, 0],
          [testSimId, 1]
        ]);
        done();
      });
    });

    it('should send simulation ended if simulation has failed', (done) => {
      const testSimId = 1234;
      const jobId = mockObjectId()

      fetchSimulation.mockResolvedValue(mockSimulation('failed'));
      fetchJob.mockResolvedValueOnce({_id: jobId, simulations: [mockSimulation('finished', testSimId)]})
      fetchCountsInSimulation.mockReturnValueOnce([new MockMongoDocument({dummyKey: 'dummyValue', hour: 1})])

      handleRequest(mockSocket, jobId.toString());

      process.nextTick(() => {
        expect(fetchJob).toHaveBeenCalledTimes(1)
        expect(fetchJob).toHaveBeenCalledWith(jobId)

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
      fetchJob.mockResolvedValue({
        _id: jobId,
        simulations: [mockSimulation('finished', simId1), mockSimulation('finished', simId2)]
      });

      aggregateSimulations.mockReturnValueOnce([1, 2])

      handleRequest(mockSocket, jobId.toString());

      process.nextTick(() => {
        expect(fetchJob).toHaveBeenCalledTimes(2)
        expect(fetchJob).toHaveBeenNthCalledWith(1, jobId)
        expect(fetchJob).toHaveBeenNthCalledWith(2, jobId)

        expect(aggregateSimulations).toHaveBeenCalledTimes(1);
        expect(aggregateSimulations).toHaveBeenCalledWith([simId1, simId2])

        expect(mockSocket.emit).toHaveBeenCalledTimes(3)
        expect(mockSocket.emit).toHaveBeenNthCalledWith(1, 'epidemicStats', 1)
        expect(mockSocket.emit).toHaveBeenNthCalledWith(2, 'epidemicStats', 2)
        expect(mockSocket.emit).toHaveBeenNthCalledWith(3, 'epidemicStats', {simulation_ended: true})
        done();
      })


    });

    it('should retry until all the simulations are finished', (done) => {
      const simId1 = "3457634"
      const simId2 = "76543e4"
      const jobId = mockObjectId();
      fetchJob
        .mockResolvedValueOnce({
          _id: jobId, simulations: [
            mockSimulation('in-progress', simId1),
            mockSimulation('in-queue', simId2)
          ]
        })
        .mockResolvedValueOnce({
          _id: jobId, simulations: [
            mockSimulation('finished', simId1),
            mockSimulation('in-progress', simId2)
          ]
        })
        .mockResolvedValueOnce({
          _id: jobId, simulations: [
            mockSimulation('finished', simId1),
            mockSimulation('finished', simId2)
          ]
        });

      aggregateSimulations.mockReturnValueOnce([1, 2])

      handleRequest(mockSocket, jobId.toString());

      process.nextTick(async () => {
        jest.runAllTimers();
        await flushPromises();
        expect(fetchJob).toHaveBeenCalledTimes(3)
        expect(fetchJob).toHaveBeenNthCalledWith(1, jobId)
        expect(fetchJob).toHaveBeenNthCalledWith(2, jobId)
        expect(fetchJob).toHaveBeenNthCalledWith(3, jobId)

        expect(aggregateSimulations).toHaveBeenCalledTimes(1);
        expect(aggregateSimulations).toHaveBeenCalledWith([simId1, simId2])
        done();
      })
    });

    it('should send back error if any of the job fails', (done) => {
      const simId1 = "3457634"
      const simId2 = "76543e4"
      const jobId = mockObjectId();

      fetchJob
        .mockResolvedValueOnce({_id: jobId, simulations: [
          mockSimulation('in-progress', simId1),
          mockSimulation('in-queue', simId2)
        ]})
        .mockResolvedValueOnce({_id: jobId, simulations: [
          mockSimulation('finished', simId1),
          mockSimulation('failed', simId2)
        ]})

      aggregateSimulations.mockReturnValueOnce([1, 2])

      handleRequest(mockSocket, jobId.toString());

      process.nextTick(() => {
        expect(mockSocket.emit).toHaveBeenCalledTimes(1)
        expect(mockSocket.emit).toHaveBeenCalledWith('error', {"message": "One of the simulations failed to finish"})
        done();
      })
    });
  });

  const flushPromises = () => new Promise(setImmediate);
});