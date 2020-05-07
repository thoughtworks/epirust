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

import {JobsList} from "../../jobs/JobsList";
import React from "react";
import {render} from "@testing-library/react";
import {MemoryRouter} from "react-router-dom";

describe('JobsList', () => {
  let sarsTag = {id: "12", name: "SARS"}, covidTag = {id: "11", name: "covid-19"};
  let sarsJobId = "12345", covidJobId = "12344", sarsCovidJob = "12346", noTagsJob = '123';

  const testJobs = [
    {_id: covidJobId, status: "finished", config: {tags: [covidTag]}},
    {_id: sarsJobId, status: "finished", config: {tags: [sarsTag]}},
    {_id: sarsCovidJob, status: "finished", config: {tags: [covidTag, sarsTag]}},
    {_id: noTagsJob, status: "finished", config: {tags: []}}
  ];

  it('should render jobs with active job if valid active job', () => {
    const {asFragment} = render(<MemoryRouter><JobsList jobs={testJobs} activeJob={testJobs[1]}/></MemoryRouter>);
    expect(asFragment()).toMatchSnapshot()
  });

  it('should render jobs list with no active active job if it is invalid', () => {
    const {asFragment} = render(<MemoryRouter><JobsList jobs={testJobs} activeJob={null}/></MemoryRouter>);
    expect(asFragment()).toMatchSnapshot()
  });
});
