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

const {handleRequest: handleGridRequest} = require("../../controllers/grid-io-controller");
jest.mock("../../db/models/Simulation");
jest.mock("../../db/models/Grid");
jest.mock("../../db/services/SimulationService");
const {Grid} = require("../../db/models/Grid");
const {fetchSimulation} = require("../../db/services/SimulationService");

describe("Grid controller", () => {
    let mockSocket;
    beforeEach(() => {
        jest.clearAllMocks();

        mockSocket = {
            disconnected: false,
            on: jest.fn(),
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

    it('should emit all grid if simulation has ended', (done) => {
        fetchSimulation.mockResolvedValue(mockSimulationPromise(true, "", true))
        const mockCursor = jest.fn().mockReturnValueOnce([{dummyKey: 'dummyValue', hour:1}]);
        const mockSkip = jest.fn().mockReturnValueOnce({cursor: mockCursor});
        Grid.find.mockReturnValueOnce({skip: mockSkip});

        handleGridRequest(mockSocket);
        expect(mockSocket.on).toHaveBeenCalledTimes(2);
        expect(mockSocket.on.mock.calls[0]).toHaveLength(2);
        expect(mockSocket.on.mock.calls[0][0]).toBe('simulation_id');
        const testSimId = "1234";
        mockSocket.on.mock.calls[0][1](testSimId);

        process.nextTick(() => {
            expect(mockSocket.emit).toHaveBeenCalledTimes(2);
            expect(mockSocket.emit.mock.calls[0]).toEqual([
                'gridData',
                {dummyKey: 'dummyValue', hour:1}
            ]);
            expect(mockSocket.emit.mock.calls[1]).toEqual(['gridData',  {"simulation_ended": true}]);
            expect(fetchSimulation).toHaveBeenCalledTimes(1);
            expect(fetchSimulation).toHaveBeenCalledWith(
                1234,
                ["status", "grid_consumption_finished", "config.enable_citizen_state_messages"]
            );
            expect(Grid.find).toHaveBeenCalledTimes(1);
            expect(Grid.find.mock.calls[0]).toEqual([
                {simulation_id: 1234},
                {},
                {sort: {_id: 1}}
            ]);
            expect(mockSkip).toHaveBeenCalledTimes(1);
            expect(mockSkip).toHaveBeenCalledWith(0);

            done();
        })
    });

    it('should keep emit all grid until simulation has ended', (done) => {
        const docPromises = [
            mockSimulationPromise(false, "running", true),
            mockSimulationPromise(true, "running", true)
        ];
        fetchSimulation.mockImplementation(() => Promise.resolve(docPromises.shift()))
        const cursors = [[{dummyKey: 'dummyValue', _id:1}], [{dummyKey: 'dummyValue2', _id:2}]];
        const mockCursor = jest.fn(() => cursors.shift());
        const mockSkip = jest.fn().mockReturnValue({cursor: mockCursor});
        Grid.find.mockReturnValue({skip: mockSkip});

        handleGridRequest(mockSocket);
        expect(mockSocket.on).toHaveBeenCalledTimes(2);
        expect(mockSocket.on.mock.calls[0]).toHaveLength(2);
        expect(mockSocket.on.mock.calls[0][0]).toBe('simulation_id');
        const testSimId = "1234";
        mockSocket.on.mock.calls[0][1](testSimId);

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
                1234,
                ["status", "grid_consumption_finished", "config.enable_citizen_state_messages"]
            ]);
            expect(fetchSimulation.mock.calls[1]).toEqual([
                1234,
                ["status", "grid_consumption_finished", "config.enable_citizen_state_messages"]
            ]);
            expect(Grid.find).toHaveBeenCalledTimes(2);
            expect(Grid.find.mock.calls[0]).toEqual([
                {simulation_id: 1234},
                {},
                {sort: {'_id': 1}}
            ]);
            expect(Grid.find.mock.calls[1]).toEqual([
                {simulation_id: 1234},
                {},
                {sort: {'_id': 1}}
            ]);
            expect(mockSkip).toHaveBeenCalledTimes(2);
            expect(mockSkip).toHaveBeenNthCalledWith(1, 0);
            expect(mockSkip).toHaveBeenNthCalledWith(2, 1);
            done();
        });
    });

    it('should not send simulation ended if simulation status is failed', (done) => {
        fetchSimulation.mockResolvedValue(mockSimulationPromise(false, 'failed', true))
        const mockCursor = jest.fn().mockReturnValueOnce([{dummyKey: 'dummyValue', hour:1}]);
        const mockSkip = jest.fn().mockReturnValueOnce({cursor: mockCursor});
        Grid.find.mockReturnValueOnce({skip: mockSkip});

        handleGridRequest(mockSocket);
        expect(mockSocket.on).toHaveBeenCalledTimes(2);
        expect(mockSocket.on.mock.calls[0]).toHaveLength(2);
        expect(mockSocket.on.mock.calls[0][0]).toBe('simulation_id');
        let testSimId = "1234";
        mockSocket.on.mock.calls[0][1](testSimId);

        process.nextTick(() => {
            expect(mockSocket.emit).toHaveBeenCalledTimes(2);
            expect(mockSocket.emit.mock.calls[0]).toEqual([
                'gridData',
                {dummyKey: 'dummyValue', hour:1}
            ]);
            expect(mockSocket.emit.mock.calls[1]).toEqual(['gridData',  {"simulation_ended": true}]);
            expect(fetchSimulation).toHaveBeenCalledTimes(1);
            expect(fetchSimulation).toHaveBeenCalledWith(
                1234,
                ["status", "grid_consumption_finished", "config.enable_citizen_state_messages"]
            );
            expect(Grid.find).toHaveBeenCalledTimes(1);
            expect(Grid.find.mock.calls[0]).toEqual([
                {simulation_id: 1234},
                {},
                {sort: {_id: 1}}
            ]);
            expect(mockSkip).toHaveBeenCalledTimes(1);
            expect(mockSkip).toHaveBeenCalledWith(0);

            done();
        })
    });

    it('should not emit anymore messages if socket is closed', (done) => {
        const docPromises = [
            mockSimulationPromise(false, "running", true),
            mockSimulationPromise(true, "running", true)
        ];
        const mockCursorClose = jest.fn()
        const cursors = [[{dummyKey: 'dummyValue', _id:1}], [{dummyKey: 'dummyValue2', _id:2}]];
        fetchSimulation.mockImplementation(() => Promise.resolve(docPromises.shift()))
        const mockCursor = jest.fn(() => {
            const d = cursors.shift();
            mockSocket.disconnected = d[0]._id!==1;
            d.close = mockCursorClose
            return d;
        });
        const mockSkip = jest.fn().mockReturnValue({cursor: mockCursor});
        Grid.find.mockReturnValue({skip: mockSkip});

        handleGridRequest(mockSocket);
        expect(mockSocket.on).toHaveBeenCalledTimes(2);
        expect(mockSocket.on.mock.calls[0]).toHaveLength(2);
        expect(mockSocket.on.mock.calls[0][0]).toBe('simulation_id');
        let testSimId = "1234";
        mockSocket.on.mock.calls[0][1](testSimId);

        process.nextTick(() => {
            expect(mockSocket.emit).toHaveBeenCalledTimes(1);
            expect(mockSocket.emit.mock.calls[0]).toEqual([
                'gridData',
                {dummyKey: 'dummyValue', _id:1}
            ]);
            expect(fetchSimulation).toHaveBeenCalledTimes(1);
            expect(fetchSimulation).toHaveBeenCalledWith(
                1234,
                ["status", "grid_consumption_finished", "config.enable_citizen_state_messages"]
            );
            expect(Grid.find).toHaveBeenCalledTimes(2);
            expect(Grid.find.mock.calls[0]).toEqual([
                {simulation_id: 1234},
                {},
                {sort: {'_id': 1}}
            ]);
            expect(Grid.find.mock.calls[1]).toEqual([
                {simulation_id: 1234},
                {},
                {sort: {'_id': 1}}
            ]);
            expect(mockSkip).toHaveBeenCalledTimes(2);
            expect(mockSkip).toHaveBeenNthCalledWith(1, 0);
            expect(mockSkip).toHaveBeenNthCalledWith(2, 1);
            expect(mockCursorClose).toHaveBeenCalledTimes(1);
            done();
        });
    });

    it('should console on connection closed', () => {
        const mockCursor = jest.fn().mockReturnValueOnce([]);
        const mockSkip = jest.fn().mockReturnValueOnce({cursor: mockCursor});
        Grid.find.mockReturnValueOnce({skip: mockSkip});
        global.console = {
            log: jest.fn()
        };

        handleGridRequest(mockSocket);

        expect(mockSocket.on).toHaveBeenCalledTimes(2);
        const mockOnDisconnect = mockSocket.on.mock.calls[1];
        expect(mockOnDisconnect).toHaveLength(2);
        expect(mockOnDisconnect[0]).toEqual('disconnect');
        const disconnectCallback = mockOnDisconnect[1];
        disconnectCallback("dummyReason");
        expect(console.log).toHaveBeenCalledTimes(1);
        expect(console.log.mock.calls[0]).toEqual(["Disconnect", "dummyReason"]);
    });
});