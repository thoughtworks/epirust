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


export default function AgentPositionsWrapper({ agentPositions }) {
    const [agentPositionsPerHour, setAgentPositionsPerHour] = useState([])

    useEffect(() => {

        agentPositions.forEach(({ citizen_states }, index) => {
            setTimeout(() => {
                setAgentPositionsPerHour(citizen_states)
            }, (index + 1) * 500)
        })

    }, [agentPositions])

    return (<AgentsLayer agentPositionsPerHour={agentPositionsPerHour} />)
}


function AgentsLayer({ agentPositionsPerHour }) {
    const { cellDimension, lineWidth, canvasDimension } = useContext(GridContext);

    const agentsLayerCanvas = useRef(null);
    const [agentsCanvasContext, setAgentsCanvasContext] = useState(null);

    useEffect(() => {
        if (!agentsLayerCanvas)
            return

        setAgentsCanvasContext(agentsLayerCanvas.current.getContext("2d"));

    }, [agentsLayerCanvas])

    useEffect(() => {
        if (!agentsCanvasContext)
            return

        agentsCanvasContext.clearRect(0, 0, canvasDimension, canvasDimension);
        agentsCanvasContext.fillStyle = BaseColors.BLACK
        agentPositionsPerHour.forEach((agent) => {
            const { x, y } = agent.location
            agentsCanvasContext.beginPath();
            const agentCircleRadius = Math.floor(cellDimension / 4)
            const startAngle = 0, endAngle = 2 * Math.PI
            agentsCanvasContext.arc(calculateCoordinate(x), calculateCoordinate(y), agentCircleRadius, startAngle, endAngle);
            agentsCanvasContext.fill();
        })
    }, [agentsCanvasContext, agentPositionsPerHour])

    function calculateCoordinate(x) {
        return Math.floor(x * cellDimension + (0.5 * cellDimension) + lineWidth)
    }

    return (
        <canvas ref={agentsLayerCanvas} id="grid-canvas-agents" width={canvasDimension} height={canvasDimension} style={{ position: "absolute", zIndex: 4 }} />
    )
}
