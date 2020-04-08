import React from 'react';
import { AreaColors, AgentStateToColor } from './constants';
export default function GridLegend() {

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