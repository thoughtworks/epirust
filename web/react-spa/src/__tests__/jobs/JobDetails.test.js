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

import {JobDetails} from "../../jobs/JobDetails";
import {render} from "@testing-library/react";
import React from "react";
import {MemoryRouter} from "react-router-dom";
import {act} from "react-dom/test-utils";

jest.mock("../../common/apiCall")
import {get} from "../../common/apiCall";

function mockApiResponse(isGridEnabled) {
    const config = {config: {test: "testValue", enable_citizen_state_messages: isGridEnabled}};
    const response = {json: () => config}
    get.mockResolvedValueOnce(response)
}

describe('Job Details', () => {
    it('should render the time series if the view is time-series', async () => {
        mockApiResponse(false);
        const component = render(<MemoryRouter initialEntries={['/jobs/123/time-series']}>
            <JobDetails jobId={'123'}/>
        </MemoryRouter>);

        await act(async () => {
            await flushPromises();
        })

        //TODO: Find a better way to assert. Changes irrelevant to the test would fail this test
        expect(component.container).toMatchSnapshot()
    });

    it('should render the config when config is fetched from api', async () => {
        mockApiResponse(false);
        const component = render(<MemoryRouter initialEntries={['/jobs/123/config']}>
            <JobDetails jobId={'123'}/>
        </MemoryRouter>);

        await act(async () => {
            await flushPromises();
        })

        expect(component.container).toMatchSnapshot()
    });

    it('should render error message in config tab if fetch job details fails', async () => {
        get.mockRejectedValueOnce({})
        const component = render(<MemoryRouter initialEntries={['/jobs/123/config']}>
            <JobDetails jobId={'123'} />
        </MemoryRouter>);

        await act(async () => {
            await flushPromises();
        })

        expect(component.container).toMatchSnapshot()
    });

    it('should render loader initially when no info received from socket', async () => {
        mockApiResponse(false);
        const { asFragment } = render(<MemoryRouter initialEntries={['/jobs/123/grid']}>
            <JobDetails jobId={'123'} />
        </MemoryRouter>);

        await act(async () => {
            await flushPromises();
        })

        expect(asFragment()).toMatchSnapshot()
    });

    it('should render grid if grid is enabled in config', async () => {
        mockApiResponse(true);
        const component = render(<MemoryRouter initialEntries={['/jobs/123/grid']}>
            <JobDetails jobId={'123'}/>
        </MemoryRouter>);

        await act(async () => {
            await flushPromises();
        })

        expect(component.container).toMatchSnapshot()
    });

    beforeEach(() => {
        jest.clearAllMocks();
    });

    const flushPromises = () => new Promise(setImmediate);
});