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

jest.mock('../../db/services/JobService');
const { handleRequest: handleJobsRequest } = require('../../controllers/job-status-io-controller')
const { SimulationStatus } = require('../../db/models/Simulation');
const JobService = require('../../db/services/JobService')

jest.useFakeTimers();

describe('Jobs Status controller', () => {

    afterEach(() => {
        jest.clearAllMocks();
        jest.runOnlyPendingTimers();
    });
    const flushPromises = () => new Promise(setImmediate);
    
    it('should return job status on every 15th second', async () => {
        const jobStatus0 = [{ simulation_id: 0, status: SimulationStatus.INQUEUE }]
        const jobStatus1 = [{ simulation_id: 1, status: SimulationStatus.INQUEUE }]
        const jobStatus2 = [{ simulation_id: 12, status: SimulationStatus.RUNNING }]
        const jobStatus3 = [{ simulation_id: 123, status: SimulationStatus.FINISHED }]

        //TODO: implement async iterator for the returned value from cursor.
        //this implementation restricts testing close method
        JobService.fetchJobs
            .mockReturnValueOnce(jobStatus0)
            .mockReturnValueOnce(jobStatus1)
            .mockReturnValueOnce(jobStatus2)
            .mockReturnValueOnce(jobStatus3)

        const emitSpy = jest.fn(), onHookSpy = jest.fn();
        const mockSocket = { emit: emitSpy, disconnected: false, on: onHookSpy }

        await handleJobsRequest(mockSocket);

        expect(setInterval).toHaveBeenCalledTimes(1)
        expect(setInterval.mock.calls[0][1]).toBe(15000)

        //1
        jest.advanceTimersByTime(15000);
        expect(JobService.fetchJobs).toHaveBeenCalledTimes(2)

        await flushPromises()
        expect(emitSpy).toHaveBeenCalledTimes(2)

        //2
        jest.advanceTimersByTime(15000);
        expect(JobService.fetchJobs).toHaveBeenCalledTimes(3)

        await flushPromises()
        expect(emitSpy).toHaveBeenCalledTimes(3)

        //3
        jest.advanceTimersByTime(15000);
        // mockSocket.disconnected = true
        expect(JobService.fetchJobs).toHaveBeenCalledTimes(4)

        await flushPromises()
        expect(emitSpy).toHaveBeenCalledTimes(4)

        expect(emitSpy.mock.calls).toEqual([
            ["jobStatus", [{ "simulation_id": 0, "status": "in-queue" }]],
            ["jobStatus", [{ "simulation_id": 1, "status": "in-queue" }]],
            ["jobStatus", [{ "simulation_id": 12, "status": "running" }]],
            ["jobStatus", [{ "simulation_id": 123, "status": "finished" }]]
        ])

        expect(onHookSpy.mock.calls[0][0]).toBe('disconnect');
    });

});