import React from 'react'
import Graph from './LineGraph'
import { render, getByTestId, rerender } from '@testing-library/react'
import Dygraph from 'dygraphs'

jest.mock('dygraphs')

test('should render an empty Graph without calling dygraphs for empty data buffer', () => {
    const dygraphMockFn = Dygraph.mockImplementation(() => {graph: "mockGraph"})
    const { getByTestId } = render(<Graph dataBuffer="" />)

    expect(getByTestId("visualization")).toBeInTheDocument()
    expect(dygraphMockFn).toHaveBeenCalledTimes(0)
})

test('should invoke dygraph when data buffer length is not 0 and graph is null', () => {
    const simulationData = `
    1,10,5,0,0,0
    2,15,6,0,0,0
    3,20,7,0,0,1`
    const expectedLabels = {
        "labels": ["hour", "susceptible", "infected", "quarantined", "recovered", "deceased"]
    } 
    const dygraphMockFn = Dygraph.mockImplementationOnce(() => {graph: "mockGraph"})
    render(<Graph dataBuffer={simulationData}/>)
    
    expect(dygraphMockFn).toHaveBeenCalledTimes(1)
    expect(dygraphMockFn).toHaveBeenCalledWith(expect.anything(), simulationData, expectedLabels)
    jest.clearAllMocks()
})

test('should update dygraph chart when data buffer is not 0 and graph is not null', () => {
    const simulationData1 = `
    1,10,5,0,0,0
    2,15,6,0,0,0
    3,20,7,0,0,1`
    const simulationData2 = "4,30,8,0,0,2"
    const dygraphMockFn = Dygraph.mockImplementation(() => {})
    const {rerender} = render(<Graph dataBuffer={simulationData1}/>)
    expect(dygraphMockFn).toHaveBeenCalledTimes(1)
    
    rerender(<Graph dataBuffer={simulationData2}/>)
    //TODO: test if update method was called with arguments expected
})