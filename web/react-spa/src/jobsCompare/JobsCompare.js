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

export default function JobsCompare() {
  const [jobs, updateJobs] = useState([])
  const [loadingState,updateLoadingState] = useState(LOADING_STATES.LOADING)
  useEffect(() => {
    get('/jobs')
        .then((res) => res.json())
        .then((jobsResponse) => {
          updateJobs(jobsResponse)
          updateLoadingState(LOADING_STATES.FINISHED)
        })
      .catch(() => updateLoadingState(LOADING_STATES.FAILED))
  }, [])

  return <>
    <LoadingComponent loadingState={loadingState}>
      <ComparerDropdowns jobs={jobs}/>
    </LoadingComponent>
  </>
}