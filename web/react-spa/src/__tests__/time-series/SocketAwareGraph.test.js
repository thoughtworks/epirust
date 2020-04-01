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
import SocketAwareGraph from '../../time-series/SocketAwareGraph'
import {render } from '@testing-library/react'
import MockSocket from 'socket.io-mock'
import Dygraph from 'dygraphs'
import renderer from 'react-test-renderer'

jest.mock('dygraphs')
jest.useFakeTimers();


test('should render SocketAwareGraph', () => {
    const component = renderer.create(<SocketAwareGraph />)
    let tree = component.toJSON()

    expect(tree).toMatchSnapshot()
})

test('should recieve data sent on socket and parse & set graph to empty', () => {
    const mockDygraphfn = Dygraph.mockImplementation(()=>{})
    let socket = new MockSocket()
    const hourStatistics = {
        hour: 10,
        susceptible: 9,
        infected: 2,
        quarantined: 1,
        recovered: 0,
        deceased: 0
    }

    const {getByTestId} = render(<SocketAwareGraph socket={socket.socketClient}/>)
    socket.emit("epidemicStats", hourStatistics)

    expect(mockDygraphfn).toHaveBeenCalledTimes(0)
    jest.clearAllMocks()
})

test('should set dataBuffer and render graph and plot graph', () => {
    const mockDygraphfn = Dygraph.mockImplementation(()=>{})
    let socket = new MockSocket()
    const hourStatistics = {
        hour: 100,
        susceptible: 9,
        infected: 2,
        quarantined: 1,
        recovered: 0,
        deceased: 0
    }

    // act(() => { TODO: warning for act
        render(<SocketAwareGraph socket={socket.socketClient}/>)
        socket.emit("epidemicStats", hourStatistics)
    // })
    jest.runAllTimers();
    expect(mockDygraphfn).toHaveBeenCalledWith(expect.anything(), Array.of(Object.values(hourStatistics)), expect.anything())
    jest.clearAllMocks()
})


test('should set residue also into data buffer when simulation ended flag is true', () => {
    const updateSpyFn = jest.fn()
    const mockDygraphfn = Dygraph.mockImplementation(()=>({
        updateOptions : updateSpyFn
    }))
    let socket = new MockSocket()
    socket.socketClient.close = () => {}
    const hourStatistics = {
        hour: 100,
        susceptible: 9,
        infected: 2,
        quarantined: 1,
        recovered: 0,
        deceased: 0
    }
    const hourStatistics101 = {...hourStatistics, hour: 101}
    render(<SocketAwareGraph socket={socket.socketClient}/>)
    socket.emit("epidemicStats", hourStatistics)
    socket.emit("epidemicStats", hourStatistics101)
    socket.emit("epidemicStats", {"simulation_ended": true})
    jest.runAllTimers();

    expect(mockDygraphfn).toHaveBeenCalledTimes(1)
    expect(updateSpyFn).toHaveBeenCalledWith({file: [Object.values(hourStatistics), Object.values(hourStatistics101)]})
})

test("should enable export in graph if simulation has ended", () => {
    let socket = new MockSocket()
    socket.socketClient.close = () => {};
    const {container} = render(<SocketAwareGraph socket={socket.socketClient}/>)
    expect(container.querySelector(".graph-actions .btn-secondary")).toBeDisabled()

    socket.emit("epidemicStats", {"simulation_ended": true})
    jest.runAllTimers();

    expect(container.querySelector(".graph-actions .btn-secondary")).toBeEnabled()
})

test("should close the socket on receiving simulation ended message", () => {
    let socket = new MockSocket();
    let closeCall = 0;
    socket.socketClient.close = () => {closeCall += 1};
    render(<SocketAwareGraph socket={socket.socketClient}/>);

    socket.emit("epidemicStats", {"simulation_ended": true});
    jest.runAllTimers();

    expect(closeCall).toBe(1);
});
