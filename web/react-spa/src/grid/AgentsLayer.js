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
import io from "socket.io-client";
import {useParams} from "react-router-dom"

export default function AgentPositionsWrapper() {
    const {id} = useParams();
    const [socket, setSocket] = useState(null);
    const [agentPositions, setAgentPositions] = useState(null);

    const [simulationStarted, setSimulationStarted] = useState(false);

    const [hasSocketDataStarted, setHasSocketDataStarted] = useState(false);
    const [socketDataExhausted, setSocketDataExhausted] = useState(false);

    const [currentDisplayIndex, setCurrentDisplayIndex] = useState(0);
    const [intervalId, setIntervalId] = useState(null);

    //starting socket
    useEffect(() => {

        if (agentPositions)
            return

        if (simulationStarted && currentDisplayIndex === 0) {
            setSocket(io('http://localhost:3000/grid-updates'));
        }
    }, [currentDisplayIndex, simulationStarted, agentPositions])

    //reading from socket
    useEffect(() => {
        if (!socket)
            return

        socket.emit('simulation_id', id);

        socket.on('gridData', function (messageRaw) {
            const message = messageRaw;

            console.log(message)

            let simulationEndedTemp = false
            if ("simulation_ended" in message) {
                simulationEndedTemp = true;
                socket.close();
            }
            else {
                message.citizen_states && setAgentPositions(pos => {
                    if (!pos)
                        return [message.citizen_states]

                    return [...pos, message.citizen_states]
                })
            }

            if (simulationEndedTemp) {
                setSocketDataExhausted(true)
            }
        });
    }, [socket])

    //displayed all the data
    useEffect(() => {
        if (!socket) return

        if (socket.connected) return

        const displayedAllHours = currentDisplayIndex >= agentPositions.length;

        if (displayedAllHours) {
            console.log("interval cleared bacause sim end", intervalId)
            clearInterval(intervalId)
            setIntervalId(null)
            return
        }
    }, [currentDisplayIndex, agentPositions, socketDataExhausted])

    //pause
    useEffect(() => {
        if (!simulationStarted && agentPositions) {
            console.log("interval cleared bacause Pause", intervalId)
            clearInterval(intervalId)
            setIntervalId(null)
        }

    }, [simulationStarted, agentPositions])

    //count++
    useEffect(() => {
        if (!simulationStarted)
            return

        if (!agentPositions || intervalId)
            return

        const interval = setInterval(() => setCurrentDisplayIndex(index => index + 1), 100)
        console.log("interval started", interval)
        setIntervalId(interval);

        return () => clearInterval(intervalId)
    }, [simulationStarted, agentPositions])

    const handleStart = () => {
        setSimulationStarted(true)
    }

    const handlePause = () => {
        setSimulationStarted(false)
    }

    const handleStop = () => {
        setCurrentDisplayIndex(0);
        setSimulationStarted(false)
    }

    const positionsToDisplay = agentPositions
        ? agentPositions[currentDisplayIndex]
        : []

    return (
        <div style={{ position: "relative" }}>
            <div style={{ position: "absolute", zIndex: 5, right: 0 }}>
                <h4>{`${currentDisplayIndex}/${agentPositions ? agentPositions.length : 0} hrs`}</h4>

                {simulationStarted
                    ? <button className="btn btn-primary" onClick={handlePause}>PAUSE</button>
                    : <button className="btn btn-success" onClick={handleStart}>{currentDisplayIndex === 0 ? 'START' : 'RESUME'}</button>
                }

                <button className="btn btn-danger" onClick={handleStop}>STOP</button>
            </div>
            <AgentsLayer agentPositionsPerHour={positionsToDisplay} />
        </div>
    )
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
        if (!agentsCanvasContext || !agentPositionsPerHour)
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
