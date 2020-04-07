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
import { render } from '@testing-library/react'
import LandmarksLayer from '../../grid/LandmarksLayer'
import { GridContext } from '../../grid/index'

test('should render LandmarksLayer component', () => {
    const cellDimension = 2
    const lineWidth = 1
    const canvasDimension = 20
    const size = 10

    const landmarksDimensions = {
        "housesDimensions": [
            {
                "start_offset": {
                    "x": 0,
                    "y": 0
                },
                "end_offset": {
                    "x": 2,
                    "y": 2
                },
            },
            {
                "start_offset": {
                    "x": 2,
                    "y": 0
                },
                "end_offset": {
                    "x": 4,
                    "y": 2
                },
            }],
        "officesDimensions": [
            {
                "start_offset": {
                    "x": 5,
                    "y": 0
                },
                "end_offset": {
                    "x": 6,
                    "y": 1
                },
            },
            {
                "start_offset": {
                    "x": 6,
                    "y": 0
                },
                "end_offset": {
                    "x": 7,
                    "y": 1
                },
            }]
    }

    const { getByTestId } = render(
        <GridContext.Provider value={{ cellDimension, lineWidth, canvasDimension, size }}>
            <LandmarksLayer landmarksDimensions={landmarksDimensions} />
        </GridContext.Provider>)
    const context = getByTestId("grid-canvas-landmarks").getContext("2d")
    expect(context.__getEvents()).toMatchSnapshot()
})