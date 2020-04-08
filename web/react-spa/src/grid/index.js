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
import LandmarksLayer from './LandmarksLayer';
import LinesLayer from './LinesLayer';
import AreasLayer from './AreasLayer';
import AgentsLayer from './AgentsLayer';
import { AreaColors, AgentStateToColor } from './constants';
import agentMovement from '../resources/agent-movement';


export default function GridPage() {
    const { housing_area, work_area, transport_area, hospital_area } = gridLayout
    const areaDimensions = [
        { ...housing_area, color: AreaColors.HOUSING },
        { ...work_area, color: AreaColors.WORK },
        { ...transport_area, color: AreaColors.TRANSPORT },
        { ...hospital_area, color: AreaColors.HOSPITAL }
    ]

    const housesDimensions = gridLayout.houses,
        officesDimensions = gridLayout.offices

    return (
        <div className="grid-wrap">
            <CanvasGrid size={gridLayout.grid_size} areaDimensions={areaDimensions} landmarksDimensions={{ housesDimensions, officesDimensions }} />
            <GridLegends />
        </div>
    )
}

function GridLegends() {

    const areasLegends = [
        { backgroundColor: AreaColors.HOUSING, text: "Housing" },
        { backgroundColor: AreaColors.WORK, text: "Work" },
        { backgroundColor: AreaColors.TRANSPORT, text: "Transport" },
        { backgroundColor: AreaColors.HOSPITAL, text: "Hospital" },
        { backgroundColor: AreaColors.OTHER, text: "Other" }
    ]

    const agentsLegends = [
        { backgroundColor: AgentStateToColor.s, text: "Susceptible" },
        { backgroundColor: AgentStateToColor.i, text: "Infected" },
        { backgroundColor: AgentStateToColor.r, text: "Recovered" },
        { backgroundColor: AgentStateToColor.d, text: "Deceased" }
    ]
    return (
        <div className="legends-wrap">
            <h6>Areas</h6>
            <ul className="legends areas">{
                areasLegends.map(({ backgroundColor, text }) => (
                    <li>
                        <div className="legend-item">
                            <span className="swatch" style={{ backgroundColor }}></span>
                            <span className="text">{text}</span>
                        </div>
                    </li>
                ))}
            </ul>

            <h6>Agent States</h6>
            <ul className="legends agents">{
                agentsLegends.map(({ backgroundColor, text }) => (
                    <li>
                        <div className="legend-item">
                            <span className="swatch" style={{ backgroundColor }}></span>
                            <span className="text">{text}</span>
                        </div>
                    </li>
                ))}
            </ul>
        </div>
    )
}

export const GridContext = React.createContext(null);

export function CanvasGrid({ size, areaDimensions, landmarksDimensions }) {

    const cellDimension = Math.floor((window.innerHeight - 165) / size),
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
                <AreasLayer areaDimensions={areaDimensions} />
                <LinesLayer />
                <LandmarksLayer landmarksDimensions={landmarksDimensions} />
                <AgentsLayer agentPositions={agentMovement.agentsPerTick} />
            </GridContext.Provider>
        </div>
    )
}