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
import { render, fireEvent } from '@testing-library/react'
import ParametersForm from '../../simulation-configuration/ParamtersForm'
import renderer from 'react-test-renderer'

test('should render ParametersForm with defaults', () => {
    const component = renderer.create(<ParametersForm onDataSubmit={jest.fn()} />)
    let tree = component.toJSON()

    expect(tree).toMatchSnapshot()
});

test('invoke handler on form submit', () => {
    const handleSubmitData = jest.fn();
    const { getByTestId } = render(<ParametersForm onDataSubmit={handleSubmitData} />);

    const expectedData = {
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

    fireEvent.submit(getByTestId('simulationForm'))
    expect(handleSubmitData).toHaveBeenCalledWith(expectedData)
});

test("onSubmit form if grid-view checkbox is on data should contain `enable_citizen_state_messages` with value true", () => {
    const handleSubmitData = jest.fn();
    const { getByLabelText, getByTestId } = render(<ParametersForm onDataSubmit={handleSubmitData} />);

    fireEvent.change(getByLabelText("Grid Size"), { target: { value: 100 } });
    fireEvent.change(getByLabelText("Grid Visualization"), { target: { checked: true } });
    fireEvent.submit(getByTestId('simulationForm'));

    expect("enable_citizen_state_messages" in handleSubmitData.mock.calls[0][0]).toBe(true);
    expect(handleSubmitData.mock.calls[0][0].enable_citizen_state_messages).toBe(true);
});

test('invoke handler on form submit with only active interventions', () => {
    const handleSubmitData = jest.fn()
    const { getByTestId, getByLabelText } = render(<ParametersForm onDataSubmit={handleSubmitData} />)

    //click the lockdown intervention
    expect(getByLabelText('Lockdown').checked).toBe(true)
    fireEvent.click(getByLabelText('Lockdown'))
    expect(getByLabelText('Lockdown').checked).toBe(false)


    fireEvent.submit(getByTestId('simulationForm'))
    const capturedData = handleSubmitData.mock.calls[0][0];
    expect("essential_workers_population" in capturedData).toBe(false);
    expect("lockdown_at_number_of_infections" in capturedData).toBe(false);

    expect(capturedData["hospital_spread_rate_threshold"]).toBe(100);
    expect(capturedData["vaccinate_at"]).toBe(5000);
    expect(capturedData["vaccinate_percentage"]).toBe(0.2);
});

