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
import Graph from '../../time-series/LineGraph'
import { render } from '@testing-library/react'
import Dygraph from 'dygraphs'
import renderer from 'react-test-renderer'

jest.mock('dygraphs')

test("should render Graph", () => {
    const component = renderer.create(<Graph dataBuffer={[]} />)
    let tree = component.toJSON()

    expect(tree).toMatchSnapshot()
})

test('should render an empty Graph without calling dygraphs for empty data buffer', () => {
    const dygraphMockFn = Dygraph.mockImplementation(() => { graph: "mockGraph" })
    const { getByTestId } = render(<Graph dataBuffer={[]} />)

    expect(getByTestId("visualization")).toBeInTheDocument()
    expect(dygraphMockFn).toHaveBeenCalledTimes(0)
})

test('should invoke dygraph when data buffer length is not 0 and graph is null', () => {
    const simulationData = [
        [1, 10, 5, 0, 0, 0],
        [2, 15, 6, 0, 0, 0],
        [3, 20, 7, 0, 0, 1]]
    const labels = ["hour", "susceptible", "infected", "quarantined", "recovered", "deceased"]

    const expectedOptions = {
        "labels": labels,
        legend: 'always',
        animatedZooms: true,
        title: 'Time Series Graph',
        ylabel: 'Number of Agents',
        xlabel: 'Hours',
        showRoller: true,
        errorBars: false,
        height: 700,
        width: 1300
    }
    const dygraphMockFn = Dygraph.mockImplementationOnce(() => { graph: "mockGraph" })
    render(<Graph dataBuffer={simulationData} labels={labels} />)

    expect(dygraphMockFn).toHaveBeenCalledTimes(1)
    expect(dygraphMockFn).toHaveBeenCalledWith(expect.anything(), simulationData, expectedOptions)
    jest.clearAllMocks()
})

test('should update dygraph chart when data buffer is not 0 and graph is not null', () => {
    const simulationData1 = [
        [1, 10, 5, 0, 0, 0],
        [2, 15, 6, 0, 0, 0],
        [3, 20, 7, 0, 0, 1]]
    const simulationData2 = [[4, 30, 8, 0, 0, 2]]
    const updateSpyFn = jest.fn()
    const dygraphMockFn = Dygraph.mockImplementation(() => ({ updateOptions: updateSpyFn }))
    const { rerender } = render(<Graph dataBuffer={simulationData1} />)
    expect(dygraphMockFn).toHaveBeenCalled()
    expect(updateSpyFn).toHaveBeenNthCalledWith(1, { file: simulationData1 }) //TODO: stop update if new Dygraph has already been generated

    rerender(<Graph dataBuffer={simulationData2} />)
    expect(updateSpyFn).toHaveBeenNthCalledWith(2, { file: simulationData2 })
})
