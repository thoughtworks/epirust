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
import { LandmarkColors } from './constants';

export default function LandmarksLayer({ landmarksDimensions }) {
    const { cellDimension, lineWidth, canvasDimension, size } = useContext(GridContext);

    const landmarksLayerCanvas = useRef(null);
    const [landmarksCanvasContext, setLandmarksCanvasContext] = useState(null);

    useEffect(() => {
        if (!landmarksDimensions)
            return

        if (!landmarksLayerCanvas)
            return

        setLandmarksCanvasContext(landmarksLayerCanvas.current.getContext("2d"));

    }, [landmarksLayerCanvas, landmarksDimensions])

    useEffect(() => {
        if (!landmarksDimensions)
            return

        if (!landmarksCanvasContext)
            return

        landmarksCanvasContext.lineWidth = lineWidth;

        [
            { landmark: landmarksDimensions.housesDimensions, color: LandmarkColors.HOUSES },
            { landmark: landmarksDimensions.officesDimensions, color: LandmarkColors.OFFICES }
        ].forEach((landmarkObject) => {
            landmarksCanvasContext.strokeStyle = landmarkObject.color

            landmarkObject.landmark.forEach((element) => {

                const startX = element.start_offset.x;
                const startY = element.start_offset.y;

                const width = 1 + element.end_offset.x - startX
                const height = 1 + element.end_offset.y - startY

                /**
                 *   ____________
                 *  | 0,0 | 0, 1|
                 *  |_____|_____|
                 *  | 0,0 | 1, 1|
                 *  |_____|_____|
                 * e.g. for a landmark, start_offset {x: 0, y: 0} & end_offset {x: 1, y: 1}
                 * Hence, width and height of the landmark is 2 cells.
                 */

                landmarksCanvasContext.strokeRect((startX * cellDimension) + lineWidth / 2, (startY * cellDimension) + lineWidth / 2, width * cellDimension, height * cellDimension);
            });
        });
    }, [landmarksCanvasContext, cellDimension, lineWidth, landmarksDimensions])

    return (
        <canvas ref={landmarksLayerCanvas} data-testid="grid-canvas-landmarks" id="grid-canvas-landmarks" width={canvasDimension} height={canvasDimension} style={{ position: "absolute", zIndex: 3 }} />
    )
}