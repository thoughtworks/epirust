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

import React from "react";
import JobsDropdown from "./JobsDropdown";
import PropTypes from "prop-types";

export default function ComparerDropdowns({jobs, updateSelectedJobs}) {
  return (
    <div className='comparer-dropdowns container'>
      <div className='row justify-content-md-center'>
        <div className="form-group col-4">
          <JobsDropdown jobs={jobs} label='Job 1'/>
        </div>
        <div className="form-group col-4">
          <JobsDropdown jobs={jobs} label='Job 2'/>
        </div>
      </div>
    </div>
  );
}

ComparerDropdowns.propTypes = {
  jobs: PropTypes.array.isRequired,
  updateSelectedJobs: PropTypes.func
}