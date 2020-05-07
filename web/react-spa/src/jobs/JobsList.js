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

import {JobTile} from "./JobTile";
import React, {useState} from "react";
import TagsSelect from "../simulation-configuration/TagsSelect";

const REACT_SELECT_ACTIONS = {
  CLEAR: 'clear',
  SELECT: 'select-option',
  POP: 'pop-value',
  REMOVE: 'remove-value'
};


export const JobsList = ({jobs, activeJob}) => {
  const [tagIdsToFilter, updateTagIdsToFilter] = useState([]);

  function handleOnSubmit(e) {
    e.preventDefault();
  }

  function handleSelectChange(changeValues, {action}) {
    const changedIds = changeValues ? changeValues.map(option => option.value) : [];

    switch (action) {
      case REACT_SELECT_ACTIONS.SELECT:
        updateTagIdsToFilter(changedIds);
        break;

      case REACT_SELECT_ACTIONS.CLEAR:
        updateTagIdsToFilter([]);
        break;

      case REACT_SELECT_ACTIONS.POP:
      case REACT_SELECT_ACTIONS.REMOVE:
        updateTagIdsToFilter(changedIds);
        break;

    }
  }

  function jobIsTagged(job) {
    if (!tagIdsToFilter.length)
      return true;

    return job.config.tags.some(({id}) => tagIdsToFilter.includes(id));
  }

  return (
    <>
      <form className="job-filter-controls" onSubmit={handleOnSubmit}>
        <TagsSelect label={"Filter Jobs"} placeholder={"Type tag name..."} onChange={handleSelectChange}/>
      </form>
      <ul className="list-group scrollable job-list">
        {jobs
          .filter(jobIsTagged)
          .map(job =>
            <JobTile
              key={job._id}
              jobId={job._id}
              status={job.status}
              tags={job.config.tags}
              isActive={activeJob ? job._id === activeJob._id : false}
            />)
        }
      </ul>
    </>
  );
};
