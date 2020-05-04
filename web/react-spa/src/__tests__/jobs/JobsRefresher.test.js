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

import {JobsRefresher} from "../../jobs/JobsRefresher";
import {get} from "../../common/apiCall";
import {reduceStatus} from "../../jobs/JobTransformer";
import {LOADING_STATES} from "../../common/constants";

jest.mock("../../common/apiCall")
jest.mock("../../jobs/JobTransformer")
jest.useFakeTimers();

describe('Jobs Refresher', function () {
  let jobsRefresher, updateJobs, updateLoadingState;
  beforeEach(() => {
    updateJobs = jest.fn();
    updateLoadingState = jest.fn();
    jobsRefresher = new JobsRefresher(updateJobs, updateLoadingState)

    reduceStatus.mockImplementation(x => x)
  });

  it('should set loading state to be failed if the api call fails', async function () {
    get.mockRejectedValueOnce()

    jobsRefresher.start()
    await flushPromises();

    expect(get).toHaveBeenCalledTimes(1);
    expect(get).toHaveBeenCalledWith('/jobs');

    expect(updateLoadingState).toHaveBeenCalledTimes(1);
    expect(updateLoadingState).toHaveBeenCalledWith(LOADING_STATES.FAILED);
  });

  it('should update the jobs and not make another api call if all jobs have finished', async function () {
    const jobs = [{_id: 'id1', status: 'finished'}, {_id: 'id2', status: 'finished'}];
    const jobsToRender = [{_id: 'id2', status: 'finished'}, {_id: 'id1', status: 'finished'}];
    get.mockResolvedValueOnce({json: jest.fn().mockResolvedValue(jobs)})

    jobsRefresher.start()
    await flushPromises();
    jest.advanceTimersByTime(15000);
    await flushPromises();

    expect(get).toHaveBeenCalledTimes(1);
    expect(get).toHaveBeenCalledWith('/jobs');

    expect(updateJobs).toHaveBeenCalledTimes(1)
    expect(updateJobs).toHaveBeenCalledWith(jobsToRender)
    expect(updateLoadingState).toHaveBeenCalledTimes(1);
    expect(updateLoadingState).toHaveBeenCalledWith(LOADING_STATES.FINISHED);
  });

  it('should update the jobs and make api calls till all jobs have finished', async function () {
    const jobs = [{_id: 'id1', status: 'finished'}, {_id: 'id2', status: 'running'}, {_id: 'id3', status: 'running'}];
    const jobsToRender = [{_id: 'id3', status: 'running'}, {_id: 'id2', status: 'running'}, {_id: 'id1', status: 'finished'}];
    get.mockResolvedValueOnce({json: jest.fn().mockResolvedValue(jobs)})

    jobsRefresher.start()
    await flushPromises();

    expect(get).toHaveBeenCalledTimes(1);
    expect(get).toHaveBeenLastCalledWith('/jobs');

    expect(updateJobs).toHaveBeenCalledTimes(1)
    expect(updateJobs).toHaveBeenLastCalledWith(jobsToRender)
    expect(updateLoadingState).toHaveBeenCalledTimes(1);
    expect(updateLoadingState).toHaveBeenLastCalledWith(LOADING_STATES.FINISHED);

    const jobs2 = [{_id: 'id2', status: 'finished'}, {_id: 'id3', status: 'running'}];
    const jobsToRender2 = [{_id: 'id3', status: 'running'}, {_id: 'id2', status: 'finished'}, {_id: 'id1', status: 'finished'}];
    get.mockResolvedValueOnce({json: jest.fn().mockResolvedValue(jobs2)})

    jest.advanceTimersByTime(15000);
    await flushPromises();

    expect(get).toHaveBeenCalledTimes(2);
    expect(get).toHaveBeenLastCalledWith('/jobs?jobIds=id3,id2');

    expect(updateJobs).toHaveBeenCalledTimes(2)
    expect(updateJobs.mock.calls[1][0](jobsToRender)).toEqual(jobsToRender2)

    const jobs3 = [{_id: 'id3', status: 'finished'}];
    const jobsToRender3 = [{_id: 'id3', status: 'finished'}, {_id: 'id2', status: 'finished'}, {_id: 'id1', status: 'finished'}];
    get.mockResolvedValueOnce({json: jest.fn().mockResolvedValue(jobs3)})

    jest.advanceTimersByTime(15000);
    await flushPromises();

    expect(get).toHaveBeenCalledTimes(3);
    expect(get).toHaveBeenLastCalledWith('/jobs?jobIds=id3');

    expect(updateJobs).toHaveBeenCalledTimes(3)
    expect(updateJobs.mock.calls[2][0](jobsToRender2)).toEqual(jobsToRender3)

    jest.advanceTimersByTime(15000);
    expect(get).toHaveBeenCalledTimes(3);
  });

  afterEach(() => {
    jest.clearAllTimers();
    jest.clearAllMocks();
  });

  const flushPromises = () => new Promise(setImmediate)
});