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
import { GridContext } from '../../grid/index'

import AgentPositionsWrapper from '../../grid/AgentsLayer';


jest.useFakeTimers()

afterEach(() => {
    jest.clearAllTimers()
    jest.clearAllMocks()
})

const gridContextData = {
    cellDimension: 2,
    lineWidth: 1,
    canvasDimension: 20,
    size: 10
}
const agentPositions = [[
    { "id": 595, "state": "s", "location": { "x": 0, "y": 0 } },
    { "id": 238, "state": "s", "location": { "x": 37, "y": 66 } },
    { "id": 981, "state": "s", "location": { "x": 31, "y": 1 } }
]];

test('should render AgentsLayer when grid visualization starts', () => {

    const { getByTestId, getByText } = render(
        <GridContext.Provider value={gridContextData}>
            <AgentPositionsWrapper agentPositions={agentPositions} simulationEnded={false} />
        </GridContext.Provider>)

    fireEvent.click(getByText("START"))
    jest.advanceTimersByTime(1000)

    expect(setInterval).toHaveBeenCalledTimes(1)
    expect(getByTestId("grid-canvas-agents").getContext("2d").__getEvents()).toMatchSnapshot()
})

test('should render AgentsLayer when grid visualization pauses', () => {
    const { getByTestId, getByText, rerender } = render(
        <GridContext.Provider value={gridContextData}>
            <AgentPositionsWrapper agentPositions={agentPositions} simulationEnded={false} />
        </GridContext.Provider>)

    fireEvent.click(getByText("START"))

    jest.advanceTimersByTime(100)

    fireEvent.click(getByText("PAUSE"))

    rerender(
        <GridContext.Provider value={gridContextData}>
            <AgentPositionsWrapper agentPositions={[agentPositions[0], agentPositions[0]]} simulationEnded={false} />
        </GridContext.Provider>
    )

    fireEvent.click(getByText("RESUME"))

    expect(getByTestId("grid-canvas-agents").getContext("2d").__getEvents()).toMatchSnapshot()
})



