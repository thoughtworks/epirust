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
import React, {useEffect, useState} from "react";
import TagsSelect from "../simulation-configuration/TagsSelect";
import {useHistory, useLocation} from "react-router-dom";

const REACT_SELECT_ACTIONS = {
  CLEAR: 'clear',
  SELECT: 'select-option',
  POP: 'pop-value',
  REMOVE: 'remove-value'
};


export const JobsList = ({jobs, activeJob}) => {
  let history = useHistory();
  const location = useLocation();

  const [tagIdsToFilter, updateTagIdsToFilter] = useState([]);
  const [listHeight, updateListHeight] = useState(0);
  const filterFormRef = React.useRef(null);

  useEffect(() => {
    if (!location)
      return;

    let filterTags = location.search
      .match(/filterTags=.+?(?=&|$)/);

    if (!filterTags)
      return;

    const filterTagsUrl = filterTags && filterTags[0]
      .split("=")[1]
      .split(',');

    updateTagIdsToFilter(filterTagsUrl)

  }, [location]);

  function handleSelectChange(changeValues, {action}) {
    const changedIds = changeValues ? changeValues.map(option => option.value) : [];

    const filterQueryParams = changedIds.length > 0
      ? `?filterTags=${changedIds.join(',')}`
      : "";

    switch (action) {
      case REACT_SELECT_ACTIONS.SELECT:
      case REACT_SELECT_ACTIONS.CLEAR:
      case REACT_SELECT_ACTIONS.POP:
      case REACT_SELECT_ACTIONS.REMOVE:
        updateTagIdsToFilter(changedIds);
        history.push(`${location.pathname}${filterQueryParams}`);
        break;
    }
  }

  useEffect(() => {
    if (!filterFormRef)
      return;

    const parentElementHeight = filterFormRef.current.parentElement.clientHeight,
      formHeight = filterFormRef.current.clientHeight;

    updateListHeight(parentElementHeight - formHeight)
  }, [filterFormRef, tagIdsToFilter]);

  function isJobTagged(job) {
    if (!tagIdsToFilter || !tagIdsToFilter.length)
      return true;
    return job.config.tags.some(({id}) => tagIdsToFilter.includes(id));
  }

  function handleJobClick(jobId) {
    const filterQueryParams = tagIdsToFilter.length > 0
      ? `?filterTags=${tagIdsToFilter.join(',')}`
      : "";

    history.push(`/jobs/${jobId}${filterQueryParams}`)
  }

  return (
    <>
      <form ref={filterFormRef} className="job-filter-controls" onSubmit={e => e.preventDefault()}>
        <TagsSelect
          label={"Filter"}
          placeholder={"Type tag name..."}
          onChange={handleSelectChange}
          tagIdsSelected={tagIdsToFilter}/>
      </form>
      <ul className="list-group scrollable job-list" data-testid="job-list" style={{maxHeight: listHeight}}>
        {jobs
          .filter(isJobTagged)
          .map(job =>
            <JobTile
              key={job._id}
              jobId={job._id}
              status={job.status}
              tags={job.config.tags}
              isActive={activeJob ? job._id === activeJob._id : false}
              onClick={handleJobClick}
            />)
        }
      </ul>
    </>
  );
};
