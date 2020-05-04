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
import ShallowRenderer from 'react-test-renderer/shallow';
import {JobsContainer} from "../../jobs/JobsContainer";
import React from "react";
import {renderWithRouter} from "../helper";

describe('JobsContainer', () => {
  const testJobs = [
    {_id: "12344", status: "finished"},
    {_id: "12345", status: "finished"},
    {_id: "12346", status: "finished"}
  ]
  it('should redirect to first job in list if job id not present', () => {
    const {history} = renderWithRouter(
      <JobsContainer jobs={testJobs} activeJobId={null} currentView={"time-series"}/>
    )

    expect(history.location.pathname).toEqual("/jobs/12344/time-series")
  });

  it('should redirect to time-series component if view is not present', () => {
    const {history} = renderWithRouter(
      <JobsContainer jobs={testJobs} activeJobId={"12346"} currentView={null}/>
    )

    expect(history.location.pathname).toEqual("/jobs/12346/time-series")
  });

  it('should render jobs component if jobId and view are present', () => {
    const renderer = new ShallowRenderer()
    renderer.render(<JobsContainer jobs={testJobs} activeJobId={"12345"} currentView={"time-series"}/>)

    expect(renderer.getRenderOutput()).toMatchSnapshot()
  });
});
