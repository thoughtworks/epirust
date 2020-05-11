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
import {reduceStatus} from "../jobs/JobTransformer";
import {CompareView} from "./CompareView";

export default function JobsCompare() {
  const [jobs, updateJobs] = useState([]);
  const [loadingState, updateLoadingState] = useState(LOADING_STATES.LOADING);
  const [selectedJobs, updateSelectedJobs] = useState(null);

  useEffect(() => {
    get('/jobs')
      .then((res) => res.json())
      .then((jobsResponse) => {
        updateJobs(jobsResponse.map(reduceStatus).filter(j => j.status === 'finished'));
        updateLoadingState(LOADING_STATES.FINISHED)
      })
      .catch(() => updateLoadingState(LOADING_STATES.FAILED))
  }, []);

  const onCompare = (jobPair) => {
    updateSelectedJobs({
      job1: jobs.find(x => x._id === jobPair.job1),
      job2: jobs.find(x => x._id === jobPair.job2)
    })
  };

  return (
    <div>
      <LoadingComponent loadingState={loadingState}>
        <div className='row justify-content-md-center'>
          <div className='col-7'>
            <ComparerDropdowns jobs={jobs} onCompare={onCompare}/>
          </div>
        </div>
        {selectedJobs && <CompareView selectedJobs={selectedJobs}/>}
      </LoadingComponent>
    </div>
  )
}
