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

import {JobsList} from "./JobsList";
import {JobDetails} from "./JobDetails";
import React from "react";

export const Jobs = ({jobs, activeJob}) => {
  return (
    <div className="row jobs-page">
      <div className="col-2 jobs">
        <JobsList jobs={jobs} activeJob={activeJob}/>
      </div>
      <div className="col-10 left-border scrollable details">
        {activeJob ?
          <JobDetails jobId={activeJob._id} config={activeJob.config}/>
          : <div>Invalid job id</div>}
      </div>
    </div>
  )
};
