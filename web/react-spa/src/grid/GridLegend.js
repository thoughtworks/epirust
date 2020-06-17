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
import { AgentStateToColor, AreaColors } from './constants';
import PropTypes from 'prop-types'

export default function GridLegend({ susceptible, exposed, infected, recovered, deceased }) {

    const areasLegends = [
        { backgroundColor: AreaColors.HOUSING, text: "Housing" },
        { backgroundColor: AreaColors.WORK, text: "Work" },
        { backgroundColor: AreaColors.TRANSPORT, text: "Transport" },
        { backgroundColor: AreaColors.HOSPITAL, text: "Hospital" },
        { backgroundColor: AreaColors.OTHER, text: "Other" }
    ];

    const agentsLegends = [
        { backgroundColor: AgentStateToColor.s, text: "Susceptible", count: susceptible },
        { backgroundColor: AgentStateToColor.e, text: "Exposed", count: exposed },
        { backgroundColor: AgentStateToColor.i, text: "Infected", count: infected },
        { backgroundColor: AgentStateToColor.r, text: "Recovered", count: recovered },
        { backgroundColor: AgentStateToColor.d, text: "Deceased", count: deceased }
    ];
    return (
        <div className="legends-wrap px-2 py-2">
            <h6>Areas</h6>
            <ul className="legends areas">{
                areasLegends.map(({ backgroundColor, text }) => (
                    <li key={text}>
                        <div className="legend-item">
                            <span className="swatch" style={{ backgroundColor }} />
                            <span className="text">{text}</span>
                        </div>
                    </li>
                ))}
            </ul>

            <h6>Agent States</h6>
            <ul className="legends agents">{
                agentsLegends.map(({ backgroundColor, text, count }) => (
                    <li key={text}>
                        <div className="legend-item">
                            <span className="swatch" style={{ backgroundColor }} />
                            <span className="text count">{count}</span>
                            <span className="text">{text}</span>
                        </div>
                    </li>
                ))}
            </ul>
        </div>
    )
}

GridLegend.propTypes = {
    susceptible: PropTypes.number,
    exposed: PropTypes.number,
    infected: PropTypes.number,
    recovered: PropTypes.number,
    deceased: PropTypes.number
};
