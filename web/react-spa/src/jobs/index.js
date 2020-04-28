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

import React, { useEffect, useState } from "react";
import './jobs-list.scss'
import { Job } from "./Job";
import { JobDetails } from "./JobDetails";
import { Redirect, useParams } from 'react-router-dom';
import config from "../config";
import io from 'socket.io-client'
import Loader from "../common/Loader";
import {reduceStatus} from "./JobTransformer";

export const JobsList = () => {
  const { id: paramId, view } = useParams();
  const [simulations, updateSimulations] = useState([]);
  const [socket, setSocket] = useState(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    if (!socket) {
      setSocket(io(`${config.API_HOST}/job-status`))
    }
    else {
      socket.on('jobStatus', (data) => {
        updateSimulations(data.map(reduceStatus).reverse())
        setIsLoading(false)
      })
    }
    return () => {
      socket && socket.close()
    }
  }, [socket]);

  function renderSimulationTabs() {
    return (
      <div className="col-2">
        <ul className="list-group scrollable">
          {simulations.map(s =>
            <Job
              key={s._id}
              jobId={s._id}
              status={s.status}
              isActive={s._id === paramId}
            />
          )}
        </ul>
      </div>
    );
  }

  function renderDetails() {
      return (
      <div className="col-10 left-border scrollable details">
        <JobDetails jobId={paramId} />
      </div>
    );
  }

  if (isLoading) {
    return <Loader />
  }

  if (!paramId && simulations.length) {
    return (<Redirect to={`/jobs/${simulations[0]._id}/time-series`} />);
  }

  if (paramId && !view) {
    return (<Redirect to={`/jobs/${paramId}/time-series`} />);
  }

  return (
    <div className="row jobs-list">
      {renderSimulationTabs()}
      {renderDetails()}
    </div >
  );

}

