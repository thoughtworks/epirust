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

const {handleRequest} = require("../../controllers/grid-io-controller");
jest.mock("../../db/models/Simulation");
jest.mock("../../db/models/Grid");
jest.mock("../../db/services/SimulationService");
jest.mock("../../db/services/GridService");
jest.mock("../../db/services/JobService");
const {fetchSimulation} = require("../../db/services/SimulationService");
const {findSortedById} = require("../../db/services/GridService");
const {mockObjectId} = require('../helpers');
const {fetchJob} = require('../../db/services/JobService')

describe("Grid controller", () => {
    let mockSocket;
    describe('when there is single simulation in the job', () => {
        beforeEach(() => {
            fetchJob.mockResolvedValueOnce({_id: mockObjectId(), simulations: [{_id: "1234"}]})
        });

        it('should emit all grid if simulation has ended', (done) => {
            const jobId = mockObjectId();
            fetchSimulation.mockResolvedValue(mockSimulationPromise(true, "", true))
            findSortedById.mockReturnValueOnce([{dummyKey: 'dummyValue', hour:1}]);

            handleRequest(mockSocket, jobId.toString());

            process.nextTick(() => {
                expect(mockSocket.emit).toHaveBeenCalledTimes(2);
                expect(mockSocket.emit.mock.calls[0]).toEqual([
                    'gridData',
                    {dummyKey: 'dummyValue', hour:1}
                ]);
                expect(mockSocket.emit.mock.calls[1]).toEqual(['gridData',  {"simulation_ended": true}]);
                expect(fetchSimulation).toHaveBeenCalledTimes(1);
                expect(fetchSimulation).toHaveBeenCalledWith(
                  "1234",
                  ["status", "grid_consumption_finished", "config.enable_citizen_state_messages"]
                );
                expect(findSortedById).toHaveBeenCalledTimes(1);
                expect(findSortedById).toHaveBeenCalledWith("1234", 0);
                done();
            })
        });

        it('should keep emit all grid until simulation has ended', (done) => {
            const jobId = mockObjectId();
            const docPromises = [
                mockSimulationPromise(false, "running", true),
                mockSimulationPromise(true, "running", true)
            ];
            fetchSimulation.mockImplementation(() => Promise.resolve(docPromises.shift()))
            const cursors = [[{dummyKey: 'dummyValue', _id:1}], [{dummyKey: 'dummyValue2', _id:2}]];
            findSortedById.mockImplementation(() => cursors.shift());

            handleRequest(mockSocket, jobId);

            process.nextTick(() => {
                expect(mockSocket.emit).toHaveBeenCalledTimes(3);
                expect(mockSocket.emit.mock.calls[0]).toEqual([
                    'gridData',
                    {dummyKey: 'dummyValue', _id:1}
                ]);
                expect(mockSocket.emit.mock.calls[1]).toEqual([
                    'gridData',
                    {dummyKey: 'dummyValue2', _id:2}
                ]);
                expect(mockSocket.emit.mock.calls[2]).toEqual(['gridData',  {"simulation_ended": true}]);
                expect(fetchSimulation).toHaveBeenCalledTimes(2);
                expect(fetchSimulation.mock.calls[0]).toEqual([
                    "1234",
                    ["status", "grid_consumption_finished", "config.enable_citizen_state_messages"]
                ]);
                expect(fetchSimulation.mock.calls[1]).toEqual([
                    "1234",
                    ["status", "grid_consumption_finished", "config.enable_citizen_state_messages"]
                ]);
                expect(findSortedById).toHaveBeenCalledTimes(2);
                expect(findSortedById.mock.calls[0]).toEqual(["1234", 0]);
                expect(findSortedById.mock.calls[1]).toEqual(["1234", 1]);
                done();
            });
        });

        it('should not send simulation ended if simulation status is failed', (done) => {
            const jobId = mockObjectId();
            fetchSimulation.mockResolvedValue(mockSimulationPromise(false, 'failed', true))
            findSortedById.mockReturnValueOnce([{dummyKey: 'dummyValue', hour:1}]);

            handleRequest(mockSocket, jobId);

            process.nextTick(() => {
                expect(mockSocket.emit).toHaveBeenCalledTimes(2);
                expect(mockSocket.emit.mock.calls[0]).toEqual([
                    'gridData',
                    {dummyKey: 'dummyValue', hour:1}
                ]);
                expect(mockSocket.emit.mock.calls[1]).toEqual(['gridData',  {"simulation_ended": true}]);
                expect(fetchSimulation).toHaveBeenCalledTimes(1);
                expect(fetchSimulation).toHaveBeenCalledWith(
                  "1234",
                  ["status", "grid_consumption_finished", "config.enable_citizen_state_messages"]
                );
                expect(findSortedById).toHaveBeenCalledTimes(1);
                expect(findSortedById).toHaveBeenCalledWith("1234", 0);
                done();
            })
        });

        it('should not emit anymore messages if socket is closed', (done) => {
            const jobId = mockObjectId();
            const docPromises = [
                mockSimulationPromise(false, "running", true),
                mockSimulationPromise(true, "running", true)
            ];
            const mockCursorClose = jest.fn()
            const cursors = [[{dummyKey: 'dummyValue', _id:1}], [{dummyKey: 'dummyValue2', _id:2}]];
            fetchSimulation.mockImplementation(() => Promise.resolve(docPromises.shift()))
            findSortedById.mockImplementation(() => {
                const d = cursors.shift();
                mockSocket.disconnected = d[0]._id!==1;
                d.close = mockCursorClose
                return d;
            });

            handleRequest(mockSocket, jobId);

            process.nextTick(() => {
                expect(mockSocket.emit).toHaveBeenCalledTimes(1);
                expect(mockSocket.emit.mock.calls[0]).toEqual([
                    'gridData',
                    {dummyKey: 'dummyValue', _id:1}
                ]);
                expect(fetchSimulation).toHaveBeenCalledTimes(1);
                expect(fetchSimulation).toHaveBeenCalledWith(
                  "1234",
                  ["status", "grid_consumption_finished", "config.enable_citizen_state_messages"]
                );
                expect(findSortedById).toHaveBeenCalledTimes(2);
                expect(findSortedById.mock.calls[0]).toEqual(["1234", 0]);
                expect(findSortedById.mock.calls[1]).toEqual(["1234", 1]);
                expect(mockCursorClose).toHaveBeenCalledTimes(1);
                done();
            });
        });
    });

    describe('when the job has multiple simulations', () => {
        it('should send back error saying that grid visualization is not available for multi-simulation', (done) => {
            const jobId = mockObjectId();
            fetchJob.mockResolvedValueOnce({simulations: [{_id: mockObjectId()}, {_id: mockObjectId()}]})

            handleRequest(mockSocket, jobId.toString());

            process.nextTick(() => {
                expect(fetchJob).toHaveBeenCalledTimes(1);
                expect(fetchJob).toHaveBeenCalledWith(jobId)

                expect(mockSocket.emit).toHaveBeenCalledTimes(1);
                expect(mockSocket.emit).toHaveBeenCalledWith(
                  'error',
                  {message: "Grid visualization is not supported for multi-simulation job"}
                )
                done();
            })
        });
    });

    describe('when the job has no associated simulations', () => {
        it('should send back error with message no simulations for provided jobId', (done) => {
            const jobId = mockObjectId();
            fetchJob.mockResolvedValueOnce({simulations: []})

            handleRequest(mockSocket, jobId.toString());

            process.nextTick(() => {
                expect(fetchJob).toHaveBeenCalledTimes(1);
                expect(fetchJob).toHaveBeenCalledWith(jobId)

                expect(mockSocket.emit).toHaveBeenCalledTimes(1);
                expect(mockSocket.emit).toHaveBeenCalledWith(
                  'error',
                  {message: "No simulation for provided jobId or invalid job-id provided"}
                )
                done();
            })
        });
    });

    beforeEach(() => {
        jest.clearAllMocks();

        mockSocket = {
            disconnected: false,
            emit: jest.fn()
        };
    });

    function mockSimulationPromise(grid_consumption_finished, status, is_grid_enabled) {
        return {
            grid_consumption_finished,
            simulation_id: 'dummyId',
            status: status,
            config: {
                enable_citizen_state_messages: is_grid_enabled
            }
        };
    }
});