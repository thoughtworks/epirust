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

export const JobsList = () => {
  const { id: paramId, view } = useParams();
  const [simulations, updateSimulations] = useState([]);
  const [socket, setSocket] = useState(null);
  const [isLoading, setIsLoading] = useState(false);

  useEffect(() => {
    setIsLoading(true)
    //TODO: [multi-sim] jobs api
    fetch(`${config.API_HOST}/simulation/`)
      .then(res => res.json())
      .then(value => updateSimulations(value.reverse()))
      .then(() => setIsLoading(false))
  }, []);

  // "[{"_id":"5ea297e1d117e34f524f1470","status":"finished","job_id":"5ea297e1d117e34f524f146f","__v":0}]"
  useEffect(() => {
    if (!socket) {
      setSocket(io(`${config.API_HOST}/job-status`))
    }
    else {
      socket.on('jobStatus', (data) => {
        //TODO: [multi-sim] this structure should also change from backend
        updateSimulations(data.reverse())
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
              key={s.job_id}
              jobId={s.job_id}
              status={s.status}
              isActive={s.job_id === paramId}
            />
          )}
        </ul>
      </div>
    );
  }

  function renderDetails() {
    const simulationDetails = simulations.find(s => s.job_id === paramId);

    if (!simulationDetails) return null

    return (
      <div className="col-10 left-border scrollable details">
        <JobDetails jobId={paramId} details={simulationDetails} />
      </div>
    );
  }

  if (isLoading) {
    return <Loader />
  }

  if (!paramId && simulations.length) {
    return (<Redirect to={`/jobs/${simulations[0].job_id}/time-series`} />);
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

