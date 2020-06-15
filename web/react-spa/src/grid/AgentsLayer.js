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
import React, { useContext, useEffect, useRef, useState } from 'react';
import { GridContext } from './index'
import { AgentStateMapper, AgentStateToColor } from './constants';
import GridLegend from "./GridLegend";
import {faPauseCircle, faPlayCircle, faRedo, faStepForward, faStepBackward} from "@fortawesome/free-solid-svg-icons";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";

export default function AgentPositionsWrapper({ agentPositions, simulationEnded }) {
    const [displayIndexIncrementPaused, setDisplayIndexIncrementPaused] = useState(true);
    const [currentDisplayIndex, setCurrentDisplayIndex] = useState(0);
    const [intervalId, setIntervalId] = useState(null);
    const [clickedPause, setClickedPause] = useState(false);

    const stopIncrement = () => {
        clearInterval(intervalId);
        setIntervalId(null);
    };

    const displayedAll = () => {
        return simulationEnded && (currentDisplayIndex >= agentPositions.length - 1)
    };

    //displayed all the data
    useEffect(() => {
        if (displayedAll()) {
            stopIncrement()
        }
    }, [currentDisplayIndex, agentPositions]);

    //pause
    useEffect(() => {
        if (displayIndexIncrementPaused && agentPositions) {
            stopIncrement();
        }
    }, [displayIndexIncrementPaused, agentPositions]);

    //race condition
    useEffect(() => {
        if (!agentPositions || simulationEnded) {
            return
        }
        if (currentDisplayIndex >= agentPositions.length - 1) {
            setDisplayIndexIncrementPaused(true);
            return
        }

        if (displayIndexIncrementPaused && !clickedPause) {
            setDisplayIndexIncrementPaused(false)
        }

    }, [currentDisplayIndex, agentPositions, simulationEnded, clickedPause]);

    //count++
    useEffect(() => {
        if (displayIndexIncrementPaused)
            return;

        if (!agentPositions || intervalId)
            return;

        const interval = setInterval(() => setCurrentDisplayIndex(index => index + 1), 100);
        console.log("interval started", interval);
        setIntervalId(interval);

        return () => clearInterval(intervalId)
    }, [displayIndexIncrementPaused, agentPositions]);

    const handleResume = () => {
        setClickedPause(false);
        setDisplayIndexIncrementPaused(false)
    };

    const handlePause = () => {
        setClickedPause(true);
        setDisplayIndexIncrementPaused(true)
    };

    const handleReset = () => {
        setCurrentDisplayIndex(0);
        setClickedPause(true);
        setDisplayIndexIncrementPaused(true)
    };

    const positionsToDisplay = agentPositions
        ? agentPositions[currentDisplayIndex]
        : [];

    const getCounts = (citizens) => {
        const counts = {
            'susceptible': 0,
            'exposed': 0,
            'infected': 0,
            'recovered': 0,
            'deceased': 0
        };
        citizens && citizens.forEach(a => counts[AgentStateMapper[a.state]] += 1);
        return counts

    };

    const handleAddHour = () => {
        if (currentDisplayIndex + 1 >= agentPositions.length)
            return

        setCurrentDisplayIndex(idx => idx + 1)
    }

    const handleSubtractHour = () => {
        if (currentDisplayIndex <= 0)
            return

        setCurrentDisplayIndex(idx => idx - 1)
    }

    const dayAndHour = (index) => {
        let day = Math.floor((index + 1) / 24);
        let hour = (index + 1) % 24;
        return `Day ${day}, hour: ${hour}`;
    }

    return (
        <div style={{ position: "relative" }}>
            <div className="card p-4 grid-control" style={{ position: "absolute", zIndex: 5, right: 0, width: 190 }}>
                <h4 style={{ textAlign: 'center' }} className="pb-2" data-testid="counter">{`${currentDisplayIndex + 1}/${agentPositions ? agentPositions.length : 0} hrs`}</h4>

                <div style={{ textAlign: 'center' }} className="pb-2" >{ dayAndHour(currentDisplayIndex) }</div>

                <div className="btn-group mb-2 justify-content-center" role="group" aria-label="Buttons to pause/play and reset">
                    {clickedPause
                        ? <FontAwesomeIcon onClick={handleResume} data-testid="resume" disabled={displayedAll()} icon={faPlayCircle} className="icon resume"/>
                        : <FontAwesomeIcon data-testid="pause" icon={faPauseCircle} onClick={handlePause} disabled={displayedAll()} className="icon pause"/>
                    }

                    <FontAwesomeIcon onClick={handleReset} data-testid="reset" icon={faRedo} className="icon reset"/>
                </div>

                {(clickedPause || displayedAll()) && (
                    <div data-testid="hour-step-controls" className="btn-group my-0 mx-auto hour-step-control" role="group" aria-label="Buttons to increase/decrease current displayed hour">
                        <FontAwesomeIcon icon={faStepBackward} data-testid="decrement-hour" name="decrement hour" onClick={handleSubtractHour} className="icon"/>
                        <FontAwesomeIcon icon={faStepForward} data-testid="increment-hour" name="increment hour" onClick={handleAddHour} className="icon" />
                    </div>
                )}

            </div>
            <AgentsLayer agentPositionsPerHour={positionsToDisplay} />

            <GridLegend {...getCounts(positionsToDisplay)} />
        </div>
    )
}

function AgentsLayer({ agentPositionsPerHour }) {
    const { cellDimension, lineWidth, canvasDimension } = useContext(GridContext);

    const agentsLayerCanvas = useRef(null);
    const [agentsCanvasContext, setAgentsCanvasContext] = useState(null);

    useEffect(() => {
        if (!agentsLayerCanvas)
            return;

        setAgentsCanvasContext(agentsLayerCanvas.current.getContext("2d"));

    }, [agentsLayerCanvas]);

    useEffect(() => {
        if (!agentsCanvasContext || !agentPositionsPerHour)
            return;

        agentsCanvasContext.clearRect(0, 0, canvasDimension, canvasDimension);

        agentPositionsPerHour.forEach((agent) => {
            const { x, y } = agent.location;

            agentsCanvasContext.fillStyle = AgentStateToColor[agent.state];

            agentsCanvasContext.beginPath();
            const agentCircleRadius = Math.floor(cellDimension / 2);
            const startAngle = 0, endAngle = 2 * Math.PI;
            agentsCanvasContext.arc(calculateCoordinate(x), calculateCoordinate(y), agentCircleRadius, startAngle, endAngle);
            agentsCanvasContext.fill();
        })
    }, [agentsCanvasContext, agentPositionsPerHour]);

    function calculateCoordinate(x) {
        return Math.floor(x * cellDimension + (0.5 * cellDimension) + lineWidth)
    }

    return (
        <canvas ref={agentsLayerCanvas} data-testid="grid-canvas-agents" id="grid-canvas-agents" width={canvasDimension}
            height={canvasDimension} style={{ position: "absolute", zIndex: 4 }} />
    )
}
