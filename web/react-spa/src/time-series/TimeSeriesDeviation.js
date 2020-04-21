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

import React, { useState, useEffect } from 'react'
import Graph from './LineGraph';
import config from "../config";
import Loader from "../common/Loader"

export function transformTimeSeriesDeviationMessages(message) {

    const { hour, susceptible, infected, quarantined, recovered, deceased,
        infected_mean, susceptible_mean, quarantined_mean, recovered_mean, deceased_mean,
        infected_std, susceptible_std, quarantined_std, recovered_std, deceased_std } = message;

    const perHourStats = [
        hour,
        susceptible, 0,
        infected, 0,
        quarantined, 0,
        recovered, 0,
        deceased, 0,
        susceptible_mean, susceptible_std,
        infected_mean, infected_std,
        quarantined_mean, quarantined_std,
        recovered_mean,
        recovered_std,
        deceased_mean, deceased_std
    ];

    return perHourStats;
}

export function TimeSeriesDeviation({ simulationId }) {
    const [data, setData] = useState([]);
    const [isLoading, setIsLoading] = useState(false);

    useEffect(() => {
        setIsLoading(true);

        fetch(`${config.API_HOST}/simulation/${simulationId}/time-series-deviation`)
            .then(res => res.json())
            .then(hours => hours.map((hour) => transformTimeSeriesDeviationMessages(hour)))
            .then(hours => {
                const labels = [
                    "hour",
                    "susceptible", "susceptible_mean",
                    "infected", "infected_mean",
                    "quarantined", "quarantined_mean",
                    "recovered", "recovered_mean",
                    "deceased", "deceased_mean"
                ]

                setIsLoading(false)

                if (hours.length) {
                    setData([labels, ...hours])
                }
            })
    }, [simulationId])

    if (isLoading)
        return <Loader />

    if (!data.length)
        return "Run multiple simulations for COVID-19"

    const csvFormattedData = data.join("\n");
    return <Graph dataBuffer={csvFormattedData} enableExport={true} errorBars={true} />
}