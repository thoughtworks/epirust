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
import {JobsContainer} from "../../jobs/JobsContainer";
import React from "react";
import {renderWithRouter} from "../helper/router-helper";
import {Router} from "react-router-dom";

import {get} from "../../common/apiCall";

jest.mock("../../common/apiCall");
get.mockImplementation((url) => {
  if (/jobs\/tags/.test(url))
    return Promise.resolve({json: jest.fn().mockResolvedValueOnce([])});

  return Promise.resolve({json: jest.fn().mockResolvedValueOnce({})});
});

describe('JobsContainer', () => {
  const testJobs = [
    {_id: "12344", status: "finished", config: {}},
    {_id: "12345", status: "finished", config: {}},
    {_id: "12346", status: "finished", config: {}}
  ];

  it('should redirect to first job in list if job id not present', () => {
    const {history} = renderWithRouter(
      <JobsContainer jobs={testJobs}/>, {initialEntries: ["/jobs/"]}
    );
    expect(history.location.pathname).toEqual(`/jobs/${testJobs[0]._id}/time-series`)
  });

  it('should redirect to time-series component if view is not present', () => {
    const {history} = renderWithRouter(
      <JobsContainer jobs={testJobs}/>, {initialEntries: ["/jobs/12346"]}
    );
    expect(history.location.pathname).toEqual(`/jobs/12346/time-series`);
  });

  it('should render jobs component if jobId and view are present', () => {
    const {asFragment} = renderWithRouter(
      <JobsContainer jobs={testJobs}/>, {initialEntries: ["/jobs/12346"]}
    );
    expect(asFragment()).toMatchSnapshot()
  });

  it('should show no jobs message if the fetched jobs list is empty', () => {
    const {asFragment} = renderWithRouter(
      <JobsContainer jobs={[]}/>, {initialEntries: ["/jobs/12346"]}
    );
    expect(asFragment()).toMatchSnapshot()
  });
});
