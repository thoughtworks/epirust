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
import { BaseColors } from './constants';
export default function LinesLayer() {

    const { cellDimension, lineWidth, canvasDimension, size } = useContext(GridContext);

    const linesLayerCanvas = useRef(null);
    const [lineCanvasContext, setLineCanvasContext] = useState(null);

    useEffect(() => {
        if (!linesLayerCanvas)
            return

        setLineCanvasContext(linesLayerCanvas.current.getContext("2d"));

    }, [linesLayerCanvas])

    useEffect(() => {
        if (!lineCanvasContext || lineWidth === 0)
            return

        lineCanvasContext.strokeStyle = BaseColors.GRAY_LIGHTER;
        lineCanvasContext.lineWidth = lineWidth;

        for (let x = 0; x < size; x++) {
            for (let y = 0; y < size; y++) {
                lineCanvasContext.strokeRect((x * cellDimension) + lineWidth / 2, (y * cellDimension) + lineWidth / 2, cellDimension, cellDimension);
            }
        }

    }, [lineCanvasContext, size, cellDimension, lineWidth])

    return (
        <canvas ref={linesLayerCanvas} data-testid="grid-canvas-bg" id="grid-canvas-bg" width={canvasDimension} height={canvasDimension} style={{ position: "absolute", zIndex: 2 }} />
    )
}