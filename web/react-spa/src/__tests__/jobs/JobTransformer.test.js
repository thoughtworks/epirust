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

import {reduceStatus} from "../../jobs/JobTransformer";

describe('reduceStatus', () => {
  it('should reduce job status to in-queue if all the statuses are in-queue', () => {
    const jobId = '458ihbnmk';
    const data = {
      _id: jobId,
      simulations: [
        {id: "4567", status: "in-queue"},
        {id: "567876", status: "in-queue"},
        {id: "87654", status: "in-queue"}
      ]
    }
    const reducedStatus = reduceStatus(data);

    expect(reducedStatus).toEqual({...data, status: 'in-queue'})
  });

  it('should reduce job status to running if one of the jobs is running', () => {
    const jobId = '458ihbnmk';
    const data = {
      _id: jobId,
      simulations: [
        {id: "4567", status: "in-queue"},
        {id: "567876", status: "running"},
        {id: "87654", status: "in-queue"}
      ]
    }
    const reducedStatus = reduceStatus(data);

    expect(reducedStatus).toEqual({...data, status: 'running'})
  });

  it('should reduce job status to failed if one of the jobs has failed', () => {
    const jobId = '458ihbnmk';
    const data = {
      _id: jobId,
      simulations: [
        {id: "4567", status: "in-queue"},
        {id: "567876", status: "running"},
        {id: "87654", status: "failed"}
      ]
    }
    const reducedStatus = reduceStatus(data);

    expect(reducedStatus).toEqual({...data, status: 'failed'})
  });

  it('should reduce job status to finished if all of the jobs are finished', () => {
    const jobId = '458ihbnmk';
    const data = {
      _id: jobId,
      simulations: [
        {id: "4567", status: "finished"},
        {id: "567876", status: "finished"},
        {id: "87654", status: "finished"}
      ]
    }
    const reducedStatus = reduceStatus(data);

    expect(reducedStatus).toEqual({...data, status: 'finished'})
  });

  it('should maintain the state machine property of statuses (in-queue -> running -> failed -> finished)', () => {
    const jobId = '458ihbnmk';
    const data = {
      _id: jobId,
      simulations: [
        {id: "4567", status: "finished"},
        {id: "567876", status: "in-queue"},
        {id: "87654", status: "in-queue"}
      ]
    }
    const reducedStatus = reduceStatus(data);

    expect(reducedStatus).toEqual({...data, status: 'running'})
  });
});
