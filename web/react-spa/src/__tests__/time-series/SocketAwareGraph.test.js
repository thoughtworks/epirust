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
import { render } from '@testing-library/react'
import MockSocket from 'socket.io-mock'
import Dygraph from 'dygraphs'
import { act } from 'react-dom/test-utils'

jest.mock('dygraphs')
jest.useFakeTimers();

const simulationId = 1231231231

function emitNMessages(socket, n, hourStatistics) {
    for (let index = 0; index < n; index++) {
        socket.emit("epidemicStats", hourStatistics)
    }
}

const hourStatisticsFor100thHour = {
    hour: 100,
    susceptible: 9,
    infected: 2,
    quarantined: 1,
    recovered: 0,
    deceased: 0
}

test('should render SocketAwareGraph', () => {
    const { asFragment } = render(<SocketAwareGraph simulationId={simulationId} socket={null} />)
    expect(asFragment()).toMatchSnapshot()
})

test('should recieve data sent on socket and parse & set graph to empty', () => {
    const mockDygraphfn = Dygraph.mockImplementation(() => { })
    let socket = new MockSocket()
    const closeSpy = jest.fn()
    socket.socketClient.close = closeSpy

    const hourStatistics = { ...hourStatisticsFor100thHour, hour: 10 }

    render(<SocketAwareGraph socket={socket.socketClient} simulationId={simulationId} />)

    act(() => {
        socket.emit("epidemicStats", hourStatistics)
    })
    expect(mockDygraphfn).toHaveBeenCalledTimes(0)
    jest.clearAllMocks()
})

test('should set dataBuffer and render graph and plot graph', () => {
    const mockDygraphfn = Dygraph.mockImplementation(() => { })
    let socket = new MockSocket()
    const closeSpy = jest.fn()
    socket.socketClient.close = closeSpy

    const hourStatistics = { ...hourStatisticsFor100thHour, hour: 0 }

    render(<SocketAwareGraph socket={socket.socketClient} simulationId={simulationId} />)

    act(() => {
        emitNMessages(socket, 10, hourStatistics)
        jest.runAllTimers();
    })

    expect(mockDygraphfn).toHaveBeenCalledTimes(1)
    expect(mockDygraphfn.mock.calls[0][1].length).toBe(10);
    jest.clearAllMocks()
})


test('should set residue also into data buffer when simulation ended flag is true', () => {
    const updateSpyFn = jest.fn()
    const mockDygraphfn = Dygraph.mockImplementation(() => ({
        updateOptions: updateSpyFn
    }))
    let socket = new MockSocket()

    const closeSpy = jest.fn()
    socket.socketClient.close = closeSpy
    const hourStatistics = { ...hourStatisticsFor100thHour }
    const hourStatistics101 = { ...hourStatistics, hour: 101 }

    render(<SocketAwareGraph socket={socket.socketClient} simulationId={simulationId} />)

    act(() => {
        socket.emit("epidemicStats", hourStatistics)
        socket.emit("epidemicStats", hourStatistics101)
        socket.emit("epidemicStats", { "simulation_ended": true })
        jest.runAllTimers();
    })

    expect(mockDygraphfn).toHaveBeenCalledTimes(1)
    expect(updateSpyFn).toHaveBeenCalledWith({ file: [Object.values(hourStatistics), Object.values(hourStatistics101)] })
})

test("should enable export in graph if simulation has ended", () => {
    let socket = new MockSocket()
    const closeSpy = jest.fn()
    socket.socketClient.close = closeSpy
    const { container } = render(<SocketAwareGraph socket={socket.socketClient} simulationId={simulationId} />)
    expect(container.querySelector(".graph-actions .btn-secondary")).toBeDisabled()

    act(() => {
        socket.emit("epidemicStats", { "simulation_ended": true })
        jest.runAllTimers();
    })

    expect(container.querySelector(".graph-actions .btn-secondary")).toBeEnabled()
})

test("should close the socket on receiving simulation ended message", () => {
    let socket = new MockSocket();
    const closeSpy = jest.fn()
    socket.socketClient.close = closeSpy;

    render(<SocketAwareGraph socket={socket.socketClient} simulationId={simulationId} />);

    act(() => {
        socket.emit("epidemicStats", { "simulation_ended": true });
        jest.runAllTimers();
    })

    expect(closeSpy).toHaveBeenCalledTimes(1)
});

test("should render the annotations for lockdown applied intervention ", () => {
    const hourStatistics = { ...hourStatisticsFor100thHour }

    let socket = new MockSocket();
    socket.socketClient.close = jest.fn();

    const setAnnotationSpy = jest.fn()

    Dygraph.mockImplementation(() => ({
        setAnnotations: setAnnotationSpy,
        updateOptions: jest.fn()
    }))

    render(<SocketAwareGraph socket={socket.socketClient} simulationId={simulationId} />);

    act(() => {
        emitNMessages(socket, 10, hourStatistics)

        socket.emit("epidemicStats", {
            ...hourStatistics, interventions: [{
                intervention: "lockdown",
                data: { status: "locked_down" }
            }]
        });
        jest.runAllTimers();
    })
    expect(setAnnotationSpy).toHaveBeenCalledTimes(1)
    expect(setAnnotationSpy).toHaveBeenCalledWith([{
        "attachAtBottom": true,
        "cssClass": "annotation lockdown",
        "series": "susceptible",
        "shortText": "Lockdown start",
        "text": "Lockdown start at 100",
        "tickHeight": 40,
        "x": 100
    }])
});

test("should render the annotations for lockdown revoked intervention ", () => {
    const hourStatistics = { ...hourStatisticsFor100thHour }

    let socket = new MockSocket();
    socket.socketClient.close = jest.fn();

    const setAnnotationSpy = jest.fn()

    Dygraph.mockImplementation(() => ({
        setAnnotations: setAnnotationSpy,
        updateOptions: jest.fn()
    }))

    render(<SocketAwareGraph socket={socket.socketClient} simulationId={simulationId} />);

    act(() => {
        emitNMessages(socket, 10, hourStatistics)

        socket.emit("epidemicStats", {
            ...hourStatistics, interventions: [{
                intervention: "lockdown",
                data: { status: "lockdown_revoked" }
            }]
        });
        jest.runAllTimers();
    })
    expect(setAnnotationSpy).toHaveBeenCalledTimes(1)
    expect(setAnnotationSpy).toHaveBeenCalledWith([{
        "attachAtBottom": true,
        "cssClass": "annotation lockdown",
        "series": "susceptible",
        "shortText": "Lockdown end",
        "text": "Lockdown end at 100",
        "tickHeight": 40,
        "x": 100
    }])
});

test("should render the annotations for interventions for BuildNewHospital", () => {
    const hourStatistics = { ...hourStatisticsFor100thHour }

    let socket = new MockSocket();
    socket.socketClient.close = jest.fn;

    const setAnnotationSpy = jest.fn()

    Dygraph.mockImplementation(() => ({
        setAnnotations: setAnnotationSpy,
        updateOptions: jest.fn()
    }))

    render(<SocketAwareGraph socket={socket.socketClient} simulationId={simulationId} />);

    act(() => {
        emitNMessages(socket, 10, hourStatistics)
        socket.emit("epidemicStats", {
            ...hourStatistics, interventions: [{
                intervention: "build_new_hospital",
                data: {}
            }]
        });
        jest.runAllTimers();
    })
    expect(setAnnotationSpy).toHaveBeenCalledTimes(1)
    expect(setAnnotationSpy).toHaveBeenCalledWith([{
        "attachAtBottom": true,
        "cssClass": "annotation hospital",
        "series": "susceptible",
        "shortText": "Build Hospitals",
        "text": "Build Hospitals at 100",
        "tickHeight": 40,
        "x": 100
    }])
});

test("should render the annotations for interventions for Vaccination", () => {
    let socket = new MockSocket();
    socket.socketClient.close = jest.fn;

    const setAnnotationSpy = jest.fn()

    Dygraph.mockImplementation(() => ({
        setAnnotations: setAnnotationSpy,
        updateOptions: jest.fn()
    }))

    render(<SocketAwareGraph socket={socket.socketClient} simulationId={simulationId} />);

    act(() => {
        emitNMessages(socket, 10, hourStatisticsFor100thHour)
        socket.emit("epidemicStats", {
            ...hourStatisticsFor100thHour, interventions: [{
                intervention: "vaccination",
                data: {}
            }]
        });
        jest.runAllTimers();
    })
    expect(setAnnotationSpy).toHaveBeenCalledTimes(1)
    expect(setAnnotationSpy).toHaveBeenCalledWith([{
        "attachAtBottom": true,
        "cssClass": "annotation vaccination",
        "series": "susceptible",
        "shortText": "Vaccination",
        "text": "Vaccination at 100",
        "tickHeight": 40,
        "x": 100
    }])
});


test("should render the annotations for interventions and apply height to the tick alternatively", () => {
    let socket = new MockSocket();
    socket.socketClient.close = jest.fn;

    const setAnnotationSpy = jest.fn()

    Dygraph.mockImplementation(() => ({
        setAnnotations: setAnnotationSpy,
        updateOptions: jest.fn()
    }))

    render(<SocketAwareGraph socket={socket.socketClient} simulationId={simulationId} />);

    act(() => {
        emitNMessages(socket, 10, hourStatisticsFor100thHour)
        socket.emit("epidemicStats", {
            ...hourStatisticsFor100thHour, interventions: [{
                intervention: "vaccination",
                data: {}
            }]
        });

        socket.emit("epidemicStats", {
            ...hourStatisticsFor100thHour, interventions: [{
                intervention: "lockdown",
                data: { status: "locked_down" }
            }]
        });

        socket.emit("epidemicStats", {
            ...hourStatisticsFor100thHour, interventions: [{
                intervention: "lockdown",
                data: { status: "lockdown_revoked" }
            }]
        });
        jest.runAllTimers();
    })
    expect(setAnnotationSpy).toHaveBeenCalledTimes(1)
    expect(setAnnotationSpy).toHaveBeenCalledWith([{
        "attachAtBottom": true,
        "cssClass": "annotation vaccination",
        "series": "susceptible",
        "shortText": "Vaccination",
        "text": "Vaccination at 100",
        "tickHeight": 40,
        "x": 100
    },
    {
        "attachAtBottom": true,
        "cssClass": "annotation lockdown",
        "series": "susceptible",
        "shortText": "Lockdown start",
        "text": "Lockdown start at 100",
        "tickHeight": 80,
        "x": 100
    },
    {
        "attachAtBottom": true,
        "cssClass": "annotation lockdown",
        "series": "susceptible",
        "shortText": "Lockdown end",
        "text": "Lockdown end at 100",
        "tickHeight": 40,
        "x": 100
    }])
});
