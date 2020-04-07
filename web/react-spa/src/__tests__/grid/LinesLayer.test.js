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
import LinesLayer from '../../grid/LinesLayer'
import { GridContext } from '../../grid/index'

test('should render Lines Layer correctly', () => {
    const cellDimension = 2
    const lineWidth = 1
    const canvasDimension = 20
    const size = 10

    const { getByTestId } = render(<GridContext.Provider value={{ cellDimension, lineWidth, canvasDimension, size }}><LinesLayer /></GridContext.Provider>)
    const context = getByTestId("grid-canvas-bg").getContext("2d")
    expect(context.__getEvents()).toMatchSnapshot()
})

test('should not draw lines if lineWidth is 0', () => {
    const cellDimension = 2
    const lineWidth = 0
    const canvasDimension = 20
    const size = 10

    const { getByTestId } = render(<GridContext.Provider value={{ cellDimension, lineWidth, canvasDimension, size }}><LinesLayer /></GridContext.Provider>)
    const context = getByTestId("grid-canvas-bg").getContext("2d")
    expect(context.__getEvents()).toMatchSnapshot()
})
