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
import '../styles/components/jobs/_jobs-list.scss'
import {LOADING_STATES} from "../common/constants";
import {JobsContainer} from "./JobsContainer";
import {LoadingComponent} from "../common/LoadingComponent";
import {JobsRefresher} from "./JobsRefresher";

export const JobsView = () => {
  const [loadingState, updateLoadingState] = useState(LOADING_STATES.LOADING);
  const [jobs, updateJobs] = useState([]);

  useEffect(() => {
    const jobsRefresher = new JobsRefresher(updateJobs, updateLoadingState);
    jobsRefresher.start()
  }, []);

  return <LoadingComponent loadingState={loadingState}>
    <JobsContainer jobs={jobs}/>
  </LoadingComponent>;
};
