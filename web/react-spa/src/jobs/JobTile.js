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
import '../styles/components/jobs/_job.scss'
import PropTypes from "prop-types";
import {Link} from "react-router-dom";
import Tags from "./Tags";

export const JobTile = ({jobId, status, tags = [], isActive = false, onClick}) => {
  const statusText = {
    "running": "In-Progress",
    "finished": "Finished",
    "failed": "Failed",
    "in-queue": "In-Queue"
  };

  return (
    <li className="list-group-item borderless item-less-padding">
      <div onClick={() => onClick(jobId)} className={`simulation-tab ${isActive ? "active shadow" : ""} ${status}`}>
        <div className="card-body card-body-less-padding">
          <p className="card-title title"><code>{jobId}</code></p>
          <p data-testid={`job-status-${jobId}`} className="job-status">{statusText[status]}</p>
          <Tags isSmall={true} tags={tags}/>
        </div>
      </div>
    </li>
  );
};

JobTile.propTypes = {
  jobId: PropTypes.string.isRequired,
  status: PropTypes.oneOf(["running", "finished", "failed", "in-queue"]).isRequired,
  isActive: PropTypes.bool,
  tags: PropTypes.arrayOf(PropTypes.object)
};
