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

import React, {useEffect, useState} from "react";
import ComparerDropdowns from "./ComparerDropdowns";
import {get} from "../common/apiCall";
import {LoadingComponent} from "../common/LoadingComponent";
import {LOADING_STATES} from "../common/constants";
import Graph from "../time-series/LineGraph";
import GraphUpdater from "./GraphUpdater";
import {reduceStatus} from "../jobs/JobTransformer";
import Select from "react-select";

const epiCurves = ["susceptible", "infected", "hospitalized", "recovered", "deceased"]

export default function JobsCompare() {
  const [jobs, updateJobs] = useState([])
  const [loadingState, updateLoadingState] = useState(LOADING_STATES.LOADING)
  const [graphData, updateGraphData] = useState([])
  const [selectedCurves, updateSelectedCurves] = useState([])

  useEffect(() => {
    get('/jobs')
      .then((res) => res.json())
      .then((jobsResponse) => {
        updateJobs(jobsResponse.map(reduceStatus).filter(j => j.status === 'finished'))
        updateLoadingState(LOADING_STATES.FINISHED)
      })
      .catch(() => updateLoadingState(LOADING_STATES.FAILED))
  }, [])

  const updateBuffer = (buffer) => {
    updateGraphData(prevData => prevData.concat(buffer))
  }

  const onCompare = (selectedJobs) => {
    updateGraphData([])
    new GraphUpdater(updateBuffer, selectedJobs.job1, selectedJobs.job2).start()
  }

  const dygraphOptions = () => (
    {
      colors: ["#0A1045", "#00C2D1", "#F9E900", "#F6AF65", "#ED33B9",
        "#8588a2", "#80e1e8", "#fcf480", "#fbd7b2", "#f699dc"],
      labels: ["hour",
        ...epiCurves.map(e => `${e}_job1`),
        ...epiCurves.map(e => `${e}_job2`),
      ]
    })

  return <div>
    <LoadingComponent loadingState={loadingState}>
      <div className='row justify-content-md-center'>
        <div className='col-7'>
          <ComparerDropdowns jobs={jobs} onCompare={onCompare}/>
        </div>
      </div>
      {graphData.length > 0 && (
        <div className="row">
          <div className="col-2">
            <ComparerConfig updateSelectedCurves={updateSelectedCurves}/>
          </div>
          <div className="col-10">
            <div className="jobs-compare-chart">
              <Graph
                dataBuffer={makeCSV(graphData)}
                dygraphsOptions={dygraphOptions()}
                visibility={visibility(selectedCurves)}
              />
            </div>
          </div>
        </div>
      )}
    </LoadingComponent>
  </div>
}

const visibility = (curvesSelected) => {
  const curves = [...epiCurves.map(e => `${e}_job1`), ...epiCurves.map(e => `${e}_job2`)];
  if (curvesSelected.length > 0) {
    return curves.map(l => curvesSelected.some(cs => l.includes(cs)))
  }
  return curves.map(_ => true)
}

const rowToCsv = (row) => {
  if (row) return [[row.susceptible, row.susceptible_std], [row.infected, row.infected_std],
    [row.hospitalized, row.hospitalized_std], [row.recovered, row.recovered_std], [row.deceased, row.deceased_std]]
  return [[], [], [], [], []]
}

const makeCSV = (graphData) => {
  return graphData.map(d => ([d.hour, ...rowToCsv(d.job1), ...rowToCsv(d.job2)]))
}


function ComparerConfig({updateSelectedCurves}) {
  return <div className="comparer-config">
    <label htmlFor="series-filer">Select line-graphs to show:</label>
    <Select
      options={epiCurves.map(e => ({value: e, label: e}))}
      isMulti
      inputId="series-filer"
      name="tags"
      aria-label="tags"
      onChange={(cs) => updateSelectedCurves(cs ? cs.map(c => c.value) : [])}
    />
  </div>
}