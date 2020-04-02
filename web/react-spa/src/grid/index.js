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

import React, { useState, useEffect, useRef, useContext } from 'react';

import gridLayout from '../resources/grid-layout';
import GridLandmarks from './GridLandmarks';

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

    const CELL_DIMENSION = Math.floor((window.innerHeight - 20) / size),
        LINE_WIDTH = Math.floor(CELL_DIMENSION / 4) < 1 ? 0 : Math.floor(CELL_DIMENSION / 4),
        CANVAS_DIMENSION = (size * CELL_DIMENSION) + LINE_WIDTH;

    const gridCanvasAreas = useRef(null);
    const [areasCanvasContext, setAreasCanvasContext] = useState(null);

    useEffect(() => {
        if (!gridCanvasAreas)
            return

        setAreasCanvasContext(gridCanvasAreas.current.getContext("2d"));

    }, [gridCanvasAreas])

    useEffect(() => {
        if (!areasCanvasContext)
            return

        function updateAreaColor(areaDimensions, x, y) {
            let area;
            for (var i = 0; i < areaDimensions.length; i++) {
                if (isWithinArea(areaDimensions[i], x, y)) {
                    area = areaDimensions[i];
                    break;
                }
            }

            const color = area ? area.color : "#ccc";

            if (areasCanvasContext.fillStyle !== color) {
                areasCanvasContext.fillStyle = color;
            }
        }

        for (let x = 0; x < size; x++) {
            for (let y = 0; y < size; y++) {
                updateAreaColor(areaDimensions, x, y);
                areasCanvasContext.fillRect((x * CELL_DIMENSION) + LINE_WIDTH / 2, (y * CELL_DIMENSION) + LINE_WIDTH / 2, CELL_DIMENSION, CELL_DIMENSION)
            }
        }
    }, [areasCanvasContext, size, CELL_DIMENSION, LINE_WIDTH, areaDimensions])

    return (
        <div style={{ position: "relative" }}>
            <GridContext.Provider value={{
                cellDimension: CELL_DIMENSION,
                lineWidth: LINE_WIDTH,
                canvasDimension: CANVAS_DIMENSION,
                size: size
            }}>
                <canvas ref={gridCanvasAreas} id="grid-canvas" width={CANVAS_DIMENSION} height={CANVAS_DIMENSION} style={{ position: "absolute", zIndex: 1 }} />
                {/* <GridLines /> */}
                <GridLandmarks landmarksDimensions={landmarksDimensions} />
            </GridContext.Provider>
        </div>
    )
}

function isWithinArea(area, currentX, currentY) {

    return (currentX >= area.start_offset.x && currentY >= area.start_offset.y
        && currentX < area.end_offset.x && currentY < area.end_offset.y);
}