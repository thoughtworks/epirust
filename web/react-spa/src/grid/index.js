import React, { useState, useEffect, useRef } from 'react';

import gridLayout from '../resources/grid-layout';

export default function GridPage() {
    const { housing_area, work_area, transport_area, hospital_area } = gridLayout
    const areaDimensions = [
        { ...housing_area, color: "#f1c40f" },
        { ...work_area, color: "#3498db" },
        { ...transport_area, color: "#9b59b6" },
        { ...hospital_area, color: "#e74c3c" }
    ]

    return (
        <CanvasGrid size={gridLayout.grid_size} areaDimensions={areaDimensions} />
    )
}

export function CanvasGrid({ size, areaDimensions }) {

    const CELL_DIMENSION = Math.floor((window.innerHeight - 20) / size),
        LINE_WIDTH = 1;

    const gridCanvasAreas = useRef(null);
    const gridCanvasLines = useRef(null);

    const [areasCanvasContext, setAreasCanvasContext] = useState(null);
    const [lineCanvasContext, setLineCanvasContext] = useState(null);

    useEffect(() => {
        if (!gridCanvasAreas)
            return

        setAreasCanvasContext(gridCanvasAreas.current.getContext("2d"));

    }, [gridCanvasAreas])

    useEffect(() => {
        if (!gridCanvasLines)
            return

        setLineCanvasContext(gridCanvasLines.current.getContext("2d"));

    }, [gridCanvasLines])

    useEffect(() => {
        if (!areasCanvasContext)
            return

        for (let x = 0; x < size; x++) {
            for (let y = 0; y < size; y++) {
                areasCanvasContext.fillStyle = getCellColor(areaDimensions, x, y);
                areasCanvasContext.fillRect((x * CELL_DIMENSION) + LINE_WIDTH / 2, (y * CELL_DIMENSION) + LINE_WIDTH / 2, CELL_DIMENSION, CELL_DIMENSION)
            }
        }
    }, [areasCanvasContext, size, CELL_DIMENSION, LINE_WIDTH])

    useEffect(() => {
        if (!lineCanvasContext || LINE_WIDTH === 0)
            return

        lineCanvasContext.strokeStyle = "#fff";
        lineCanvasContext.lineWidth = LINE_WIDTH;

        for (let x = 0; x < size; x++) {
            for (let y = 0; y < size; y++) {
                lineCanvasContext.strokeRect((x * CELL_DIMENSION) + LINE_WIDTH / 2, (y * CELL_DIMENSION) + LINE_WIDTH / 2, CELL_DIMENSION, CELL_DIMENSION);
            }
        }

    }, [lineCanvasContext, size, CELL_DIMENSION, LINE_WIDTH])

    return (
        <div style={{ position: "relative" }}>
            <canvas ref={gridCanvasAreas} id="grid-canvas" width={(size * CELL_DIMENSION) + (LINE_WIDTH)} height={(size * CELL_DIMENSION) + (LINE_WIDTH)} style={{ position: "absolute", zIndex: 1 }} />
            {<canvas ref={gridCanvasLines} id="grid-canvas-bg" width={(size * CELL_DIMENSION) + (LINE_WIDTH)} height={(size * CELL_DIMENSION) + (LINE_WIDTH)} style={{ border: "1px solid #000000", position: "absolute", zIndex: 2 }} />}
        </div>
    )
}

function getCellColor(areaDimensions, x, y) {
    const area = areaDimensions.filter(area => isWithinArea(area, x, y));
    
    return area.length > 0 ? area[0].color : "#ccc";
}

function isWithinArea(area, currentX, currentY) {

    return (currentX >= area.start_offset.x && currentY >= area.start_offset.y
        && currentX < area.end_offset.x && currentY < area.end_offset.y);
}
