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
import { act, fireEvent, render } from '@testing-library/react'
import { GridContext } from '../../grid/index'

import AgentPositionsWrapper from '../../grid/AgentsLayer';
import { element } from 'prop-types';

jest.useFakeTimers();

afterEach(() => {
    jest.clearAllTimers();
    jest.clearAllMocks();
});

const gridContextData = {
    cellDimension: 2,
    lineWidth: 1,
    canvasDimension: 20,
    size: 10
};
const agentPositions = [[
    { "id": 595, "state": "s", "location": { "x": 0, "y": 0 } },
    { "id": 238, "state": "s", "location": { "x": 37, "y": 66 } },
    { "id": 981, "state": "s", "location": { "x": 31, "y": 1 } }
]];

test('should pause without clicking when displayed all hours are sent by socket and resume when new data is received', () => {
    const { getByTestId, rerender } = render(
        <GridContext.Provider value={gridContextData}>
            <AgentPositionsWrapper agentPositions={[agentPositions[0], agentPositions[0]]} simulationEnded={false} />
        </GridContext.Provider>);

    act(() => {
        jest.advanceTimersByTime(500)
    });

    expect(getByTestId("counter").textContent).toBe("2/2 hrs");
    expect(clearInterval).toHaveBeenCalled();

    rerender(
        <GridContext.Provider value={gridContextData}>
            <AgentPositionsWrapper agentPositions={[agentPositions[0], agentPositions[0], agentPositions[0]]} simulationEnded={false} />
        </GridContext.Provider>);

    expect(setInterval).toHaveBeenCalled();

    act(() => {
        jest.advanceTimersByTime(100)
    });

    expect(getByTestId("counter").textContent).toBe("3/3 hrs");
    expect(getByTestId("grid-canvas-agents").getContext("2d").__getEvents()).toMatchSnapshot()
});

test('should render AgentsLayer when grid visualization resets', () => {

    const { getByTestId} = render(
        <GridContext.Provider value={gridContextData}>
            <AgentPositionsWrapper agentPositions={agentPositions} simulationEnded={false} />
        </GridContext.Provider>);
    act(() => {
        jest.advanceTimersByTime(100)
    });

    expect(getByTestId("counter").textContent).toBe("1/1 hrs");

    act(() => {
        fireEvent.click(getByTestId("reset"));
    });

    expect(getByTestId("resume")).toBeInTheDocument();
    expect(getByTestId("counter").textContent).toBe("1/1 hrs");
    expect(getByTestId("grid-canvas-agents").getContext("2d").__getEvents()).toMatchSnapshot()
});

test('should render AgentsLayer when grid visualization pauses and resume', () => {

    const { getByTestId, rerender } = render(
        <GridContext.Provider value={gridContextData}>
            <AgentPositionsWrapper agentPositions={[agentPositions[0], agentPositions[0], agentPositions[0]]} simulationEnded={false} />
        </GridContext.Provider>);

    act(() => {
        jest.advanceTimersByTime(100);
        fireEvent.click(getByTestId("pause"))
    });

    expect(getByTestId("counter").textContent).toBe("2/3 hrs");

    act(() => {
        fireEvent.click(getByTestId("resume"))
    });

    rerender(
        <GridContext.Provider value={gridContextData}>
            <AgentPositionsWrapper agentPositions={[agentPositions[0], agentPositions[0], agentPositions[0]]} simulationEnded={true} />
        </GridContext.Provider>
    );

    act(() => {
        jest.advanceTimersByTime(100)
    });

    expect(getByTestId("counter").textContent).toBe("3/3 hrs");
    expect(getByTestId("grid-canvas-agents").getContext("2d").__getEvents()).toMatchSnapshot()
});

test('should display hour step controls i.e. buttons to increment/decrement on pausing the simulation', () => {

    const { getByTestId, queryByTestId} = render(
        <GridContext.Provider value={gridContextData}>
            <AgentPositionsWrapper agentPositions={[agentPositions[0], agentPositions[0], agentPositions[0]]} simulationEnded={false} />
        </GridContext.Provider>);

    expect(queryByTestId("hour-step-controls")).not.toBeInTheDocument()

    act(() => {
        jest.advanceTimersByTime(100);
        fireEvent.click(getByTestId("pause"))
    });

    expect(getByTestId("hour-step-controls")).toBeInTheDocument()

    act(() => {
        fireEvent.click(getByTestId("resume"))
    });

    expect(queryByTestId("hour-step-controls")).not.toBeInTheDocument()
});

test('should display hour step controls i.e. buttons to increment/decrement when displayed all hours', () => {

    const { getByTestId, queryByTestId, rerender } = render(
        <GridContext.Provider value={gridContextData}>
            <AgentPositionsWrapper agentPositions={[agentPositions[0], agentPositions[0], agentPositions[0]]} simulationEnded={false} />
        </GridContext.Provider>);

    act(() => {
        jest.advanceTimersByTime(100);
    });

    expect(queryByTestId("hour-step-controls")).not.toBeInTheDocument();
    expect(getByTestId("counter").textContent).toBe("2/3 hrs");

    act(() => {
        jest.advanceTimersByTime(100);
    });

    rerender(
        <GridContext.Provider value={gridContextData}>
            <AgentPositionsWrapper agentPositions={[agentPositions[0], agentPositions[0], agentPositions[0]]} simulationEnded={true} />
        </GridContext.Provider>
    );

    expect(getByTestId("hour-step-controls")).toBeInTheDocument()
    expect(getByTestId("counter").textContent).toBe("3/3 hrs");
});

test('should increment currently displayed hour value on clicking + button', () => {

    const { getByTestId} = render(
        <GridContext.Provider value={gridContextData}>
            <AgentPositionsWrapper agentPositions={[agentPositions[0], agentPositions[0], agentPositions[0]]} simulationEnded={false} />
        </GridContext.Provider>);

    act(() => {
        jest.advanceTimersByTime(100);
        fireEvent.click(getByTestId("pause"))
    });
    expect(getByTestId("counter").textContent).toBe("2/3 hrs");

    act(() => { fireEvent.click(getByTestId("decrement-hour")) });
    expect(getByTestId("counter").textContent).toBe("1/3 hrs");
    const canvasEventsHour1 = getByTestId("grid-canvas-agents").getContext("2d").__getEvents()

    act(() => { fireEvent.click(getByTestId("increment-hour")) });
    expect(getByTestId("counter").textContent).toBe("2/3 hrs");

    act(() => { fireEvent.click(getByTestId("decrement-hour")) });
    expect(getByTestId("counter").textContent).toBe("1/3 hrs");
    const canvasEventsHour1Rerendered = getByTestId("grid-canvas-agents").getContext("2d").__getEvents()

    expect(canvasEventsHour1).toEqual(canvasEventsHour1Rerendered)
});
