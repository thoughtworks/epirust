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

import React, {useState} from "react";
import JobsDropdown from "./JobsDropdown";
import PropTypes from "prop-types";

export default function ComparerDropdowns({jobs, onCompare}) {
  const [showError, updateShowError] = useState(false);

  const handleSubmit = (e) => {
    e.preventDefault();
    const formsData = new FormData(e.target);
    const selectedJobs = Object.fromEntries(formsData.entries())
    if (selectedJobs['job1'] === selectedJobs['job2']) {
      updateShowError(true)
    } else {
      onCompare(selectedJobs)
    }
  }

  return (
    <div className='comparer-dropdowns container'>
      <form className='row' onSubmit={handleSubmit}>
        <div className="col-5">
          <JobsDropdown jobs={jobs} label='Job 1'/>
        </div>
        <div className="col-5">
          <JobsDropdown jobs={jobs} label='Job 2'/>
        </div>
        <div className="col-2 margin-top-auto">
          <button className="btn btn-primary">Compare</button>
        </div>
        {showError && <div className="error-message alert-danger">Can't compare same jobs!</div>}
      </form>
    </div>
  );
}

ComparerDropdowns.propTypes = {
  jobs: PropTypes.array.isRequired,
  onCompare: PropTypes.func
}