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

import selectEvent from "react-select-event";
import {get} from "../../common/apiCall";
import {renderWithRouter} from "../helper/router-helper";

jest.mock("../../common/apiCall");

describe('JobsList', () => {
  let sarsTag = {id: "12", name: "SARS"}, covidTag = {id: "11", name: "covid-19"};
  let sarsJobId = "12345", covidJobId = "12344", sarsCovidJob = "12346", noTagsJob = '123';

  const testJobs = [
    {_id: covidJobId, status: "finished", config: {tags: [covidTag]}},
    {_id: sarsJobId, status: "finished", config: {tags: [sarsTag]}},
    {_id: sarsCovidJob, status: "finished", config: {tags: [covidTag, sarsTag]}},
    {_id: noTagsJob, status: "finished", config: {tags: []}}
  ];

  get.mockImplementation((url) => {
    if (/jobs\/tags/.test(url))
      return Promise.resolve({json: jest.fn().mockResolvedValueOnce([])});

    return Promise.resolve({json: jest.fn().mockResolvedValueOnce({})});
  });

  it('should render jobs with active job if valid active job', () => {
    const {asFragment} = render(
      <MemoryRouter>
        <JobsList jobs={testJobs} activeJob={testJobs[1]}/>
      </MemoryRouter>
    );
    expect(asFragment()).toMatchSnapshot()
  });

  it('should render jobs list with no active active job if it is invalid', () => {
    const {asFragment} = render(
      <MemoryRouter>
        <JobsList jobs={testJobs} activeJob={null}/>
      </MemoryRouter>
    );
    expect(asFragment()).toMatchSnapshot()
  });

  //TODO: should render jobs list with no active active job if it is invalid

  it('should filter elements by selected tags', async () => {

    get.mockImplementation((url) => {
      if (/jobs\/tags/.test(url))
        return Promise.resolve({json: jest.fn().mockResolvedValueOnce([covidTag, sarsTag])});

      return Promise.resolve({json: jest.fn().mockResolvedValueOnce({})});
    });

    const {getByTestId, getByLabelText, getByText} = render(
      <MemoryRouter>
        <JobsList jobs={testJobs} activeJob={testJobs[1]}/>
      </MemoryRouter>
    );

    //display all by default
    expect(getByTestId('job-list').children.length).toBe(4)

    //display for selected tag
    await selectEvent.select(getByLabelText('Filter'), [sarsTag.name]);
    expect(getByTestId('job-list').children.length).toBe(2)
    expect(getByText(sarsJobId)).toBeInTheDocument();
    expect(getByText(sarsCovidJob)).toBeInTheDocument();

    await selectEvent.select(getByLabelText('Filter'), [covidTag.name]);
    expect(getByTestId('job-list').children.length).toBe(3);
    expect(getByText(sarsJobId)).toBeInTheDocument();
    expect(getByText(sarsCovidJob)).toBeInTheDocument();
    expect(getByText(covidJobId)).toBeInTheDocument();

    //display all when cleared tags
    await selectEvent.clearAll(getByLabelText('Filter'));
    expect(getByTestId('job-list').children.length).toBe(4)
  });

  it('should filter elements by the tags present in the URL on load and later on user selection', async () => {

    get.mockImplementation((url) => {
      if (/jobs\/tags/.test(url))
        return Promise.resolve({json: jest.fn().mockResolvedValueOnce([covidTag, sarsTag])});

      return Promise.resolve({json: jest.fn().mockResolvedValueOnce({})});
    });

    const {getByTestId, getByLabelText, getByText} = renderWithRouter(
      <JobsList jobs={testJobs} activeJob={testJobs[1]}/>, {initialEntries: [`/jobs/123/time-series?filterTags=${sarsTag.id}`]}
    );

    expect(getByTestId('job-list').children.length).toBe(2)
    expect(getByText(sarsJobId)).toBeInTheDocument();
    expect(getByText(sarsCovidJob)).toBeInTheDocument();

    await selectEvent.select(getByLabelText('Filter'), [covidTag.name]);
    expect(getByTestId('job-list').children.length).toBe(3);
    expect(getByText(sarsJobId)).toBeInTheDocument();
    expect(getByText(sarsCovidJob)).toBeInTheDocument();
    expect(getByText(covidJobId)).toBeInTheDocument();
  });
});
