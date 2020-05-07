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

export default function JobsCompare() {
  const [jobs, updateJobs] = useState([])
  const [loadingState, updateLoadingState] = useState(LOADING_STATES.LOADING)
  const [graphData, updateGraphData] = useState([])

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

  return <div className='jobs-compare'>
    <LoadingComponent loadingState={loadingState}>
      <ComparerDropdowns jobs={jobs} onCompare={onCompare}/>
      <div className='jobs-compare-chart'>
        {graphData.length > 0 && <Graph dataBuffer={makeCSV(graphData)}/>}
      </div>
    </LoadingComponent>
  </div>
}

const rowToCsv = (row) => {
  if (row) return [row.susceptible, row.susceptible_std, row.infected, row.infected_std, row.quarantined,
    row.quarantined_std, row.recovered, row.recovered_std, row.deceased, row.deceased_std]
  return ['', 0, '', 0, '', 0, '', 0, '', 0]
}

const makeCSV = (graphData) => {

  return [["hour",
    "susceptible_job1", "infected_job1", "quarantined_job1", "recovered_job1", "deceased_job1",
    "susceptible_job2", "infected_job2", "quarantined_job2", "recovered_job2", "deceased_job2"
  ]].concat(graphData.map(d => ([d.hour, ...rowToCsv(d.job1), ...rowToCsv(d.job2)]))).join('\n')
}
