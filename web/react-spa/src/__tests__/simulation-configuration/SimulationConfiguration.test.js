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
import {act, fireEvent, render, prettyDOM} from '@testing-library/react'
import {MemoryRouter} from "react-router-dom";
import SimulationConfiguration from "../../simulation-configuration";

import {get} from '../../common/apiCall';

jest.mock("../../common/apiCall");

get.mockImplementation((url) => {
  if (/jobs\/tags/.test(url))
    return Promise.resolve({json: jest.fn().mockResolvedValueOnce([{id: 'tag-id', name: "tag-name"}])});

  return Promise.resolve({json: jest.fn().mockResolvedValueOnce({})});
});

jest.mock("react-select", () => ({options}) => {
  return (<select data-testid="select" name="tags" id="tags">
    {options.map(({label, value}) => (<option key={value} value={value}>
      {label}
    </option>))}
  </select>);
});

const {post} = require("../../common/apiCall");

const mockHistoryPush = jest.fn();
jest.mock('react-router-dom', () => ({
  ...jest.requireActual('react-router-dom'),
  useHistory: () => ({
    push: mockHistoryPush,
  }),
}));


const flushPromises = () => new Promise(setImmediate);

describe('Simulation Configuration', function () {
  beforeEach(() => {
    jest.clearAllMocks();

    const mockToJson = jest.fn().mockResolvedValue({ jobId: 'dummyId' });
    post.mockResolvedValueOnce({ json: mockToJson });
  });

  function getComponent() {
    return <MemoryRouter><SimulationConfiguration /></MemoryRouter>;
  }

  it('should render with defaults', () => {
    const {asFragment} = render(getComponent());
    expect(asFragment()).toMatchSnapshot()
  });

  it('should invoke fetch on form submit', () => {
    const {getByTestId} = render(getComponent());

    fireEvent.submit(getByTestId('simulationForm'));
    expect(post).toHaveBeenCalledTimes(1);
    expect(post.mock.calls[0][0]).toBe("/jobs/init");
    expect(post.mock.calls[0][1]).toMatchSnapshot()
  });

  it("onSubmit form if grid-view checkbox is on data should contain `enable_citizen_state_messages` with value true", () => {
    const {getByLabelText, getByTestId} = render(getComponent());

    fireEvent.change(getByLabelText("Grid Size"), {target: {value: 100}});
    fireEvent.change(getByLabelText("Grid Visualization"), {target: {checked: true}});
    fireEvent.submit(getByTestId('simulationForm'));

    const body = post.mock.calls[0][1];

    expect(body).toHaveProperty('enable_citizen_state_messages');
    expect(body.enable_citizen_state_messages).toBe(true);
  });

  it('invoke handler on form submit with only active interventions', () => {
    const {getByTestId, getByLabelText} = render(getComponent());

    //click the lockdown intervention
    expect(getByLabelText('Lockdown').checked).toBe(true);
    fireEvent.click(getByLabelText('Lockdown'));
    expect(getByLabelText('Lockdown').checked).toBe(false);

    fireEvent.submit(getByTestId('simulationForm'));

    const requestBody = post.mock.calls[0][1];

    expect(requestBody).not.toHaveProperty("essential_workers_population");
    expect(requestBody).not.toHaveProperty("lockdown_at_number_of_infections");

    expect(requestBody["hospital_spread_rate_threshold"]).toBe(100);
    expect(requestBody["vaccinate_at"]).toBe(5000);
    expect(requestBody["vaccinate_percentage"]).toBe(0.2);
  });

  it('should disable submit button and show loading animation to button', () => {
    const {getByTestId, container} = render(getComponent());

    fireEvent.submit(getByTestId('simulationForm'));
    const button = container.querySelector('button');
    expect(button.disabled).toBe(true);
    expect(container).toMatchSnapshot()
  });

  it('should redirect to Jobs page on successful job creation', async function () {
    const { getByTestId } = render(getComponent());

    fireEvent.submit(getByTestId('simulationForm'));

    await act(async () => {
      await flushPromises()
    })

    expect(mockHistoryPush).toHaveBeenCalledTimes(1);
    expect(mockHistoryPush).toHaveBeenCalledWith('/jobs/dummyId')
  });

  it('should show error message if submit job fails and should enable start button', async function () {
    post.mockReset();
    post.mockRejectedValueOnce(new Error("Intentional error message. Ignore"));

    let container_a;
    act(() => {
      const { container, getByTestId } = render(getComponent());
      container_a = container;
      fireEvent.submit(getByTestId('simulationForm'))
    });

    await act(async () => {
      await flushPromises()
    })

    const button = container_a.querySelector('button');
    expect(button.disabled).toBe(false);
    expect(button.textContent).toBe("Submit");

    const errorMessage = container_a.querySelector('.error-message');
    expect(errorMessage.textContent).toBe("Error Occurred please try again!");
  })
});
