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

import React, { useState } from 'react'
import Graph from './LineGraph';

export default function RestfulGraph({ simulationId, apiPath, transformFn }) {
    const [data, setData] = useState([])

    fetch(`${apiPath}`, {
        method: 'GET',
        headers: {
            'Content-Type': 'application/json'
        }
    })
        .then(res => res.json())
        .then(hours => hours.map((hour) => transformFn(hour)))
        .then(hours => {
            const labels = ["hour", "susceptible", "infected", "quarantined", "recovered", "deceased",
                "susceptible_mean", "infected_mean", "quarantined_mean", "recovered_mean", "deceased_mean"]
            hours.unshift(labels)
            setData(hours.join("\n"))
        })

    return <Graph dataBuffer={data} enableExport={true} errorBars={true} />
}