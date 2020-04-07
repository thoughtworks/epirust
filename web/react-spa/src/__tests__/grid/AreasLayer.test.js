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
import AreasLayer from '../../grid/AreasLayer'
import { GridContext } from '../../grid/index'

test('should render AreasLayer component', () => {
    const cellDimension = 2
    const lineWidth = 1
    const canvasDimension = 20
    const size = 10

    const areaDimensions = {
        "housing_area": {
            "start_offset": {
                "x": 0,
                "y": 0
            },
            "end_offset": {
                "x": 4,
                "y": 10
            }
        },
        "work_area": {
            "start_offset": {
                "x": 5,
                "y": 0
            },
            "end_offset": {
                "x": 7,
                "y": 10
            }
        },
        "transport_area": {
            "start_offset": {
                "x": 4,
                "y": 0
            },
            "end_offset": {
                "x": 5,
                "y": 10
            }
        },
        "hospital_area": {
            "start_offset": {
                "x": 7,
                "y": 0
            },
            "end_offset": {
                "x": 8,
                "y": 10
            }
        },
    }

    const { getByTestId } = render(<GridContext.Provider value={{ cellDimension, lineWidth, canvasDimension, size }}><AreasLayer areaDimensions={areaDimensions} /></GridContext.Provider>)
    const context = getByTestId("grid-canvas").getContext("2d")
    expect(context.__getEvents()).toMatchSnapshot()
})