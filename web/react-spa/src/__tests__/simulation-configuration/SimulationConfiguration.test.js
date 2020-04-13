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

import React from 'react'
import {fireEvent, render} from '@testing-library/react'
import SimulationConfiguration from '../../simulation-configuration'

const mockHistoryPush = jest.fn();
jest.mock('react-router-dom', () => ({
    useHistory: () => ({
        push: mockHistoryPush,
    }),
}));

jest.useFakeTimers();

test('make API call form data to server on submit', () => {
        const mockToJson = jest.fn().mockResolvedValue({simulationId: 'dummyId'});
        jest.spyOn(global, 'fetch')
            .mockImplementation(() => Promise.resolve({json: mockToJson}));

        const expectedBody = {
            "death_rate": 0.2,
            "disease_name": "small_pox",
            "essential_workers_population": 0.1,
            "grid_size": 250,
            "high_transmission_rate": 0.5,
            "high_transmission_start_day": 16,
            "hospital_spread_rate_threshold": 100,
            "last_day": 22,
            "lockdown_at_number_of_infections": 100,
            "number_of_agents": 10000,
            "public_transport_percentage": 0.2,
            "regular_transmission_rate": 0.05,
            "regular_transmission_start_day": 10,
            "simulation_hrs": 10000,
            "vaccinate_at": 5000,
            "vaccinate_percentage": 0.2,
            "working_percentage": 0.7,
            "enable_citizen_state_messages": false
        };

        const { getByTestId } = render(<SimulationConfiguration />);

        fireEvent.submit(getByTestId('simulationForm'));

        expect(global.fetch).toHaveBeenCalledTimes(1);
        expect(global.fetch.mock.calls[0][0]).toBe("http://localhost:3000/simulation/init");
        expect(global.fetch.mock.calls[0][1].method).toBe("POST");
        expect(JSON.parse(global.fetch.mock.calls[0][1].body)).toStrictEqual(expectedBody);
        process.nextTick(() => {
            expect(mockHistoryPush).toHaveBeenCalledTimes(1);
            expect(mockHistoryPush).toHaveBeenCalledWith('/jobs/dummyId')
        });
});

