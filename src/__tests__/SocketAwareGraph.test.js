import React from 'react'
import SocketAwareGraph from '../SocketAwareGraph'
import {render, getByTestId, act} from '@testing-library/react'
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
    socket.emit("epidemicStats", JSON.stringify(hourStatistics))

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
        socket.emit("epidemicStats", JSON.stringify(hourStatistics))
    // })
    jest.runAllTimers();
    expect(mockDygraphfn).toHaveBeenCalledWith(expect.anything(), Array.of(Object.values(hourStatistics)), expect.anything())
})