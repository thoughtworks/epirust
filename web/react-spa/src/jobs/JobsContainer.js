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

import {Redirect} from "react-router-dom";
import {Jobs} from "./Jobs";
import React from "react";

export const JobsContainer = ({activeJobId, currentView, jobs}) => {
  if (!activeJobId)
    return (<Redirect to={`/jobs/${jobs[0]._id}/time-series`}/>);
  if (!currentView)
    return (<Redirect to={`/jobs/${activeJobId}/time-series`}/>);

  const activeJob = jobs.find(j => j._id === activeJobId)
  if (!activeJob) return <div>Invalid job requested</div>
  else return <Jobs jobs={jobs} activeJob={activeJob}/>
}
