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

import config from '../../config'
import GraphUpdater from "../../jobsCompare/GraphUpdater";
import MockSocket from "socket.io-mock";
import io from "socket.io-client";

jest.mock('../../config');
jest.mock('socket.io-client')
jest.setTimeout(15000)

describe('Graph Updater', () => {

    let mockSocket1, mockSocket2, closeSpy;
    beforeEach(() => {
        mockSocket1 = new MockSocket();
        mockSocket2 = new MockSocket();
        io.mockReturnValueOnce(mockSocket1.socketClient).mockReturnValueOnce(mockSocket2.socketClient);
        closeSpy = jest.fn();
        mockSocket1.socketClient.close = closeSpy;
        mockSocket2.socketClient.close = closeSpy;

        config.RENDER_COUNT = 10
    });

    afterEach(() => {
        jest.clearAllMocks()
    })

    it('should not update till both the jobs have same number of messages', () => {
        const jobId1 = 'jobId1';
        const jobId2 = 'jobId2';
        const mockUpdateBuffer = jest.fn();
        const graphUpdater = new GraphUpdater(mockUpdateBuffer, jobId1, jobId2)
        mockSocket1.on('get', (message) => {
            for(let i = 0; i< 10; i++) {
                mockSocket1.socketClient.emit('epidemicStats', {hour: i+1})
            }
            expect(message).toEqual({jobId: jobId1})
        });
        mockSocket2.on('get', (message) => {
            mockSocket2.socketClient.emit('epidemicStats', {hour: 1})
            expect(message).toEqual({jobId: jobId2})
        });


        graphUpdater.start()

        expect(io).toHaveBeenCalledTimes(2);
        expect(io).toHaveBeenNthCalledWith(1, 'http://localhost:3000/counts');
        expect(io).toHaveBeenNthCalledWith(2, 'http://localhost:3000/counts');
        expect(mockUpdateBuffer).not.toHaveBeenCalled()
    });

    it('should update when both the jobs have gone beyond required number of messages', () => {
        const jobId1 = 'jobId1';
        const jobId2 = 'jobId2';
        const mockUpdateBuffer = jest.fn();
        const graphUpdater = new GraphUpdater(mockUpdateBuffer, jobId1, jobId2)
        const updatedDataBuffer =  [...Array(10).keys()].map(i => ({job1: {hour: i+1}, job2: {hour: i+1}, hour: i+1}))
        mockSocket1.on('get', (message) => {
            for(let i = 0; i< 10; i++) {
                mockSocket1.emit('epidemicStats', {hour: i+1})
            }
            expect(message).toEqual({jobId: jobId1})
        });
        mockSocket2.on('get', (message) => {
            for(let i = 0; i< 10; i++) {
                mockSocket2.emit('epidemicStats', {hour: i+1})
            }
            expect(message).toEqual({jobId: jobId2})
        });


        graphUpdater.start()

        expect(io).toHaveBeenCalledTimes(2);
        expect(io).toHaveBeenNthCalledWith(1, 'http://localhost:3000/counts');
        expect(io).toHaveBeenNthCalledWith(2, 'http://localhost:3000/counts');
        expect(mockUpdateBuffer).toHaveBeenCalledTimes(1);
        expect(mockUpdateBuffer).toHaveBeenCalledWith(updatedDataBuffer);
    });

    it('should update when both the jobs till the last message has been received', () => {
        const jobId1 = 'jobId1';
        const jobId2 = 'jobId2';
        const mockUpdateBuffer = jest.fn();
        const graphUpdater = new GraphUpdater(mockUpdateBuffer, jobId1, jobId2)
        const updatedDataBuffer1 =  [...Array(10).keys()].map(i => ({job1: {hour: i+1}, job2: {hour: i+1}, hour: i+1}))
        const updatedDataBuffer2 =  [...Array(10).keys()].map(i => ({job1: {hour: i+11}, job2: {hour: i+11}, hour: i+11}))
        const updatedDataBuffer3 =  [{job2: {hour: 21}, hour: 21}, {job2: {hour: 22}, hour: 22}]
        mockSocket1.on('get', (message) => {
            for(let i = 0; i< 20; i++) {
                mockSocket1.emit('epidemicStats', {hour: i+1})
            }
            mockSocket1.emit('epidemicStats', {simulation_ended: true})
            expect(message).toEqual({jobId: jobId1})
        });
        mockSocket2.on('get', (message) => {
            for(let i = 0; i< 22; i++) {
                mockSocket2.emit('epidemicStats', {hour: i+1})
            }
            mockSocket2.emit('epidemicStats', {simulation_ended: true})
            expect(message).toEqual({jobId: jobId2})
        });


        graphUpdater.start()

        expect(io).toHaveBeenCalledTimes(2);
        expect(io).toHaveBeenNthCalledWith(1, 'http://localhost:3000/counts');
        expect(io).toHaveBeenNthCalledWith(2, 'http://localhost:3000/counts');
        expect(mockUpdateBuffer).toHaveBeenCalledTimes(3);
        expect(mockUpdateBuffer).toHaveBeenNthCalledWith(1, updatedDataBuffer1);
        expect(mockUpdateBuffer).toHaveBeenNthCalledWith(2, updatedDataBuffer2);
        expect(mockUpdateBuffer).toHaveBeenNthCalledWith(3, updatedDataBuffer3);
        expect(closeSpy).toHaveBeenCalledTimes(2)
    });
});