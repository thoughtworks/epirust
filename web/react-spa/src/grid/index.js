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

import React from 'react';

import gridLayout from '../resources/grid-layout';
import GridLandmarks from './GridLandmarks';
import GridAreas from './GridAreas';

export default function GridPage() {
    const { housing_area, work_area, transport_area, hospital_area } = gridLayout
    const areaDimensions = [
        { ...housing_area, color: "#fcce44" },
        { ...work_area, color: "#3498db" },
        { ...transport_area, color: "#9b59b6" },
        { ...hospital_area, color: "#e74c3c" }
    ]

    const housesDimensions = gridLayout.houses,
        officesDimensions = gridLayout.offices

    return (
        <CanvasGrid size={gridLayout.grid_size} areaDimensions={areaDimensions} landmarksDimensions={{ housesDimensions, officesDimensions }} />
    )
}

export const GridContext = React.createContext(null);

export function CanvasGrid({ size, areaDimensions, landmarksDimensions }) {

    const cellDimension = Math.floor((window.innerHeight - 20) / size),
        lineWidth = Math.floor(cellDimension / 4) < 1 ? 0 : Math.floor(cellDimension / 4),
        canvasDimension = (size * cellDimension) + lineWidth;

    return (
        <div style={{ position: "relative" }}>
            <GridContext.Provider value={{
                cellDimension: cellDimension,
                lineWidth: lineWidth,
                canvasDimension: canvasDimension,
                size: size
            }}>
                <GridAreas areaDimensions={areaDimensions} />
                {/* <GridLines /> */}
                < GridLandmarks landmarksDimensions={landmarksDimensions} />
            </GridContext.Provider>
        </div>
    )
}