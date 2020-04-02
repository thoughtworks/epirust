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

 import React, { useContext, useRef, useState, useEffect } from 'react';
import { GridContext } from './index'
export default function GridLines() {

    const { cellDimension, lineWidth, canvasDimension, size } = useContext(GridContext);

    const gridCanvasLines = useRef(null);
    const [lineCanvasContext, setLineCanvasContext] = useState(null);

    useEffect(() => {
        if (!gridCanvasLines)
            return

        setLineCanvasContext(gridCanvasLines.current.getContext("2d"));

    }, [gridCanvasLines])

    useEffect(() => {
        if (!lineCanvasContext || lineWidth === 0)
            return

        lineCanvasContext.strokeStyle = "#f1f1f1";
        lineCanvasContext.lineWidth = lineWidth;

        for (let x = 0; x < size; x++) {
            for (let y = 0; y < size; y++) {
                lineCanvasContext.strokeRect((x * cellDimension) + lineWidth / 2, (y * cellDimension) + lineWidth / 2, cellDimension, cellDimension);
            }
        }

    }, [lineCanvasContext, size, cellDimension, lineWidth])

    return (
        <canvas ref={gridCanvasLines} id="grid-canvas-bg" width={canvasDimension} height={canvasDimension} style={{ border: "1px solid #000000", position: "absolute", zIndex: 2 }} />
    )
}