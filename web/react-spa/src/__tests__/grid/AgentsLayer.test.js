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
import { render, findAllByTestId, fireEvent, getByText } from '@testing-library/react'
import AgentsLayer from '../../grid/AgentsLayer'
import { GridContext } from '../../grid/index'

import io from "socket.io-client";
jest.mock("socket.io-client");
import MockSocket from 'socket.io-mock'
import AgentPositionsWrapper from '../../grid/AgentsLayer';
jest.mock('react-router-dom', () => ({
    ...jest.requireActual('react-router-dom'), // use actual for all non-hook parts
    useParams: () => ({
      id: 1542319876,
    }),
  }));

jest.useFakeTimers()

afterEach(() => {
    jest.clearAllTimers()
    jest.clearAllMocks()
})

test('should render AgentsLayer when grid visualization starts', () => {
    const cellDimension = 2
    const lineWidth = 1
    const canvasDimension = 20
    const size = 10

    const message = {
        "hr": 1,
        "citizen_states": [{
            "id": 595, "state": "s",
            "location": { "x": 0, "y": 0 }
        },
        { "id": 238, "state": "s", "location": { "x": 37, "y": 66 } },
        {
            "id": 981, "state": "s", "location": { "x": 31, "y": 1 }
        }]
    }
    const mockSocket = new MockSocket()
    io.mockImplementation(() => mockSocket)
    mockSocket.close = jest.fn()
    const { getByTestId, getByText } = render(
        <GridContext.Provider value={{ cellDimension, lineWidth, canvasDimension, size }}>
            <AgentPositionsWrapper />
        </GridContext.Provider>)

    fireEvent.click(getByText("START"))
    mockSocket.socketClient.emit("gridData", message)
    mockSocket.socketClient.emit("gridData", message)
    mockSocket.socketClient.emit("gridData", { "simulation_ended": true })

    jest.advanceTimersByTime(1000)

    expect(setInterval).toHaveBeenCalledTimes(1)
    expect(getByTestId("grid-canvas-agents").getContext("2d").__getEvents()).toMatchSnapshot()
})

test('should render AgentsLayer when grid visualization pauses', () => {
    const cellDimension = 2
    const lineWidth = 1
    const canvasDimension = 20
    const size = 10

    const message = {
        "hr": 1,
        "citizen_states": [{
            "id": 595, "state": "s",
            "location": { "x": 0, "y": 0 }
        },
        { "id": 238, "state": "s", "location": { "x": 37, "y": 66 } },
        {
            "id": 981, "state": "s", "location": { "x": 31, "y": 1 }
        }]
    }
    const mockSocket = new MockSocket()
    io.mockImplementation(() => mockSocket)
    mockSocket.close = jest.fn()
    const { getByTestId, getByText } = render(
        <GridContext.Provider value={{ cellDimension, lineWidth, canvasDimension, size }}>
            <AgentPositionsWrapper />
        </GridContext.Provider>)

    fireEvent.click(getByText("START"))
    mockSocket.socketClient.emit("gridData", message)
    mockSocket.socketClient.emit("gridData", message)

    jest.advanceTimersByTime(200)
    fireEvent.click(getByText("PAUSE"))
    mockSocket.socketClient.emit("gridData", { "simulation_ended": true })
    
    expect(getByTestId("grid-canvas-agents").getContext("2d").__getEvents()).toMatchSnapshot()
})



