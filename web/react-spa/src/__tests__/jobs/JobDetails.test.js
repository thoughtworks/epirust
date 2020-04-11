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

import { JobDetails } from "../../jobs/JobDetails";
import { render } from "@testing-library/react";
import React from "react";
import { MemoryRouter } from "react-router-dom";

describe('Job Details', () => {
    it('should render the time series if the view is time-series', () => {
        const component = render(<MemoryRouter initialEntries={['/jobs/123/time-series']}>
            <JobDetails simulationId={123} details={{ config: {} }} />
        </MemoryRouter>);

        expect(component.container).toMatchSnapshot()
    });

    it('should render the config if the config is given', () => {
        const component = render(<MemoryRouter initialEntries={['/jobs/123/config']}>
            <JobDetails simulationId={123} details={{ config: { configKey: 'configValue' } }} />
        </MemoryRouter>);

        expect(component.container).toMatchSnapshot()
    });

    it('should not render the config if the config is not given', () => {
        const component = render(<MemoryRouter initialEntries={['/jobs/123/config']}>
            <JobDetails simulationId={123} />
        </MemoryRouter>);

        expect(component.container).toMatchSnapshot()
    });

    it('should render time-series if grid is disabled in config', () => {
        const simulationDetails = { config: { enable_citizen_state_messages: false } };
        const component = render(<MemoryRouter initialEntries={['/jobs/123/grid']}>
            <JobDetails simulationId={123} details={simulationDetails} />
        </MemoryRouter>);

        expect(component.container).toMatchSnapshot()
    });

    it('should render grid if grid is enabled in config', () => {
        const simulationDetails = { config: { enable_citizen_state_messages: true } };
        const component = render(<MemoryRouter initialEntries={['/jobs/123/grid']}>
            <JobDetails simulationId={123} details={simulationDetails} />
        </MemoryRouter>);

        expect(component.container).toMatchSnapshot()
    });

    it('should render time series mean comparison graph', () => {
        const component = render(<MemoryRouter initialEntries={['/jobs/123/time-series-deviation']}>
            <JobDetails simulationId={123} details={{ config: {} }} />
        </MemoryRouter>);

        expect(component.container).toMatchSnapshot()
    })
});