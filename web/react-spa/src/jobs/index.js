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

export const JobsList = () => {
  const { id, view } = useParams();
  const [simulations, updateSimulations] = useState([]);
  const [socket, setSocket] = useState(null);

  useEffect(() => {
    fetch(`${config.API_HOST}/simulation/`)
      .then(res => res.json())
      .then(value => updateSimulations(value.reverse()))
  }, []);

  useEffect(() => {
    if (!socket) {
      setSocket(io(`${config.API_HOST}/job-status`))
    }
    else {
      socket.on('jobStatus', (data) => {
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
              key={s.simulation_id}
              simulationId={s.simulation_id}
              status={s.status}
              isActive={s.simulation_id === parseInt(id)}
            />
          )}
        </ul>
      </div>
    );
  }

  function renderDetails() {
    const simulationDetails = simulations.find(s => s.simulation_id === parseInt(id));

    if (!simulationDetails) return null

    return (
      <div className="col-10 left-border scrollable details">
        <JobDetails simulationId={parseInt(id)} details={simulationDetails} />
      </div>
    );
  }

  if (!id && !simulations.length) {
    return "Loading"
  }

  if (!id && simulations.length) {
    return (<Redirect to={`/jobs/${simulations[0].simulation_id}/time-series`} />);
  }

  if (id && !view) {
    return (<Redirect to={`/jobs/${id}/time-series`} />);
  }

  return (
    <div className="row jobs-list">
      {renderSimulationTabs()}
      {renderDetails()}
    </div >
  );

}

