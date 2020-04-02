import React, { useState, useEffect, useRef } from 'react';

import gridLayout from '../resources/grid-layout';

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

export function CanvasGrid({ size, areaDimensions, landmarksDimensions }) {

    const CELL_DIMENSION = Math.floor((window.innerHeight - 20) / size),
        LINE_WIDTH = Math.floor(CELL_DIMENSION / 4) < 1 ? 0 : Math.floor(CELL_DIMENSION / 4),
        CANVAS_DIMENSION = (size * CELL_DIMENSION) + LINE_WIDTH;

    const gridCanvasAreas = useRef(null);
    // const gridCanvasLines = useRef(null);
    const gridCanvasLandmarks = useRef(null);

    const [areasCanvasContext, setAreasCanvasContext] = useState(null);
    // const [lineCanvasContext, setLineCanvasContext] = useState(null);
    const [landmarksCanvasContext, setLandmarksCanvasContext] = useState(null);

    useEffect(() => {
        if (!gridCanvasAreas)
            return

        setAreasCanvasContext(gridCanvasAreas.current.getContext("2d"));

    }, [gridCanvasAreas])

    // useEffect(() => {
    //     if (!gridCanvasLines)
    //         return

    //     setLineCanvasContext(gridCanvasLines.current.getContext("2d"));

    // }, [gridCanvasLines])

    useEffect(() => {
        if (!gridCanvasLandmarks)
            return

        setLandmarksCanvasContext(gridCanvasLandmarks.current.getContext("2d"));

    }, [gridCanvasLandmarks])


    useEffect(() => {
        if (!landmarksCanvasContext)
            return

        landmarksCanvasContext.lineWidth = LINE_WIDTH;

        landmarksCanvasContext.strokeStyle = "#f1f1f1";
        landmarksDimensions.housesDimensions.forEach(element => {
            const startX = element.start_offset.x;
            const startY = element.start_offset.y;

            const width = element.end_offset.x - startX
            const height = element.end_offset.y - startY

            landmarksCanvasContext.strokeRect((startX * CELL_DIMENSION) + LINE_WIDTH / 2, (startY * CELL_DIMENSION) + LINE_WIDTH / 2, width * CELL_DIMENSION, height * CELL_DIMENSION);
        });


        landmarksCanvasContext.strokeStyle = "#e83e8c";
        landmarksDimensions.officesDimensions.forEach(element => {
            const startX = element.start_offset.x;
            const startY = element.start_offset.y;

            const width = element.end_offset.x - startX
            const height = element.end_offset.y - startY

            landmarksCanvasContext.strokeRect((startX * CELL_DIMENSION) + LINE_WIDTH / 2, (startY * CELL_DIMENSION) + LINE_WIDTH / 2, width * CELL_DIMENSION, height * CELL_DIMENSION);
        });

    }, [landmarksCanvasContext, CELL_DIMENSION, LINE_WIDTH, landmarksDimensions])

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

    // useEffect(() => {
    //     if (!lineCanvasContext || LINE_WIDTH === 0)
    //         return

    //     lineCanvasContext.strokeStyle = "#f1f1f1";
    //     lineCanvasContext.lineWidth = LINE_WIDTH;

    //     for (let x = 0; x < size; x++) {
    //         for (let y = 0; y < size; y++) {
    //             lineCanvasContext.strokeRect((x * CELL_DIMENSION) + LINE_WIDTH / 2, (y * CELL_DIMENSION) + LINE_WIDTH / 2, CELL_DIMENSION, CELL_DIMENSION);
    //         }
    //     }

    // }, [lineCanvasContext, size, CELL_DIMENSION, LINE_WIDTH])

    return (
        <div style={{ position: "relative" }}>
            <canvas ref={gridCanvasAreas} id="grid-canvas" width={CANVAS_DIMENSION} height={CANVAS_DIMENSION} style={{ position: "absolute", zIndex: 1 }} />
            {/* <canvas ref={gridCanvasLines} id="grid-canvas-bg" width={CANVAS_DIMENSION} height={CANVAS_DIMENSION} style={{ border: "1px solid #000000", position: "absolute", zIndex: 2 }} /> */}
            <canvas ref={gridCanvasLandmarks} id="grid-canvas-landmarks" width={CANVAS_DIMENSION} height={CANVAS_DIMENSION} style={{ position: "absolute", zIndex: 3 }} />
        </div>
    )
}

function isWithinArea(area, currentX, currentY) {

    return (currentX >= area.start_offset.x && currentY >= area.start_offset.y
        && currentX < area.end_offset.x && currentY < area.end_offset.y);
}