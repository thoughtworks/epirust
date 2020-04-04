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
const {Simulation} = require("../../db/models/Simulation");
const {Grid} = require("../../db/models/Grid");

describe("Count controller", () => {
    let mockSocket;
    beforeEach(() => {
        jest.clearAllMocks();

        mockSocket = {
            on: jest.fn(),
            emit: jest.fn()
        };
    });

    function mockSimulationPromise(grid_consumption_finished) {
        return {
            then: async function (fn) {
                await fn({grid_consumption_finished, simulation_id: 'dummyId'})
            }
        }
    }

    it('should emit all counts if simulation has ended', (done) => {
        const mockPromise = mockSimulationPromise(true);
        const mockExec = jest.fn().mockReturnValue(mockPromise);
        Simulation.findOne.mockReturnValue({'exec': mockExec});
        const mockCursor = jest.fn().mockReturnValueOnce([{dummyKey: 'dummyValue', hour:1}]);
        const mockSkip = jest.fn().mockReturnValueOnce({cursor: mockCursor});
        Grid.find.mockReturnValueOnce({skip: mockSkip});

        handleGridRequest(mockSocket);

        expect(Simulation.findOne).toHaveBeenCalledTimes(1);
        expect(Simulation.findOne.mock.calls[0]).toEqual([{}, {simulation_id: 1}, {sort: {'_id': -1}}]);

        process.nextTick(() => {
            expect(mockSocket.emit).toHaveBeenCalledTimes(2);
            expect(mockSocket.emit.mock.calls[0]).toEqual([
                'gridData',
                {dummyKey: 'dummyValue', hour:1}
            ]);
            expect(mockSocket.emit.mock.calls[1]).toEqual(['gridData',  {"simulation_ended": true}]);
            expect(Simulation.findOne).toHaveBeenCalledTimes(2);
            expect(Simulation.findOne.mock.calls[0]).toEqual([{}, {simulation_id: 1}, {sort: {'_id': -1}}]);
            expect(Simulation.findOne.mock.calls[1]).toEqual([{}, {grid_consumption_finished: 1}, {sort: {'_id': -1}}]);
            expect(Grid.find).toHaveBeenCalledTimes(1);
            expect(Grid.find.mock.calls[0]).toEqual([
                {simulation_id: 'dummyId'},
                {},
                {sort: {_id: 1}}
            ]);
            expect(mockSkip).toHaveBeenCalledTimes(1);
            expect(mockSkip).toHaveBeenCalledWith(0);

            done();
        })
    });

    it('should keep emit all counts until simulation has ended', (done) => {
        const docPromises = [
            mockSimulationPromise(false),
            mockSimulationPromise(false),
            mockSimulationPromise(true),
            mockSimulationPromise(true)
        ];
        const cursors = [[{dummyKey: 'dummyValue', _id:1}], [{dummyKey: 'dummyValue2', _id:2}]];
        const mockExec = jest.fn(() => docPromises.shift());
        Simulation.findOne.mockReturnValue({'exec': mockExec});
        const mockCursor = jest.fn(() => cursors.shift());
        const mockSkip = jest.fn().mockReturnValue({cursor: mockCursor});
        Grid.find.mockReturnValue({skip: mockSkip});

        handleGridRequest(mockSocket);

        expect(Simulation.findOne).toHaveBeenCalledTimes(1);
        expect(Simulation.findOne.mock.calls[0]).toEqual([{}, {simulation_id: 1}, {sort: {'_id': -1}}]);

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
            expect(Simulation.findOne).toHaveBeenCalledTimes(4);
            expect(Simulation.findOne.mock.calls[0]).toEqual([{}, {simulation_id: 1}, {sort: {'_id': -1}}]);
            expect(Simulation.findOne.mock.calls[1]).toEqual([{}, {grid_consumption_finished: 1}, {sort: {'_id': -1}}]);
            expect(Simulation.findOne.mock.calls[2]).toEqual([{}, {simulation_id: 1}, {sort: {'_id': -1}}]);
            expect(Simulation.findOne.mock.calls[3]).toEqual([{}, {grid_consumption_finished: 1}, {sort: {'_id': -1}}]);
            expect(Grid.find).toHaveBeenCalledTimes(2);
            expect(Grid.find.mock.calls[0]).toEqual([
                {simulation_id: 'dummyId'},
                {},
                {sort: {'_id': 1}}
            ]);
            expect(Grid.find.mock.calls[1]).toEqual([
                {simulation_id: 'dummyId'},
                {},
                {sort: {'_id': 1}}
            ]);
            expect(mockSkip).toHaveBeenCalledTimes(2);
            expect(mockSkip).toHaveBeenNthCalledWith(1, 0);
            expect(mockSkip).toHaveBeenNthCalledWith(2, 1);
            done();
        });
    });

    it('should console on connection closed', () => {
        const mockPromise = mockSimulationPromise('finished');
        const mockExec = jest.fn().mockReturnValue(mockPromise);
        Simulation.findOne.mockReturnValue({'exec': mockExec});
        let mockCursor = jest.fn().mockReturnValueOnce([]);
        const mockSkip = jest.fn().mockReturnValueOnce({cursor: mockCursor});
        Grid.find.mockReturnValueOnce({skip: mockSkip});
        global.console = {
            log: jest.fn()
        };

        handleGridRequest(mockSocket);

        expect(mockSocket.on).toHaveBeenCalledTimes(1);
        expect(mockSocket.on.mock.calls[0]).toHaveLength(2);
        expect(mockSocket.on.mock.calls[0][0]).toEqual('disconnect');
        const disconnectCallback = mockSocket.on.mock.calls[0][1];
        disconnectCallback("dummyReason");
        expect(console.log).toHaveBeenCalledTimes(1);
        expect(console.log.mock.calls[0]).toEqual(["Disconnect", "dummyReason"]);
    });
});