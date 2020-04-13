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

export const JobsList = () => {
  const { id, view } = useParams();
  const [simulations, updateSimulations] = useState([]);

  useEffect(() => {
    fetch(`${config.API_HOST}/simulation/`)
      .then(res => res.json())
      .then(value => updateSimulations(value.reverse()))
  }, []);

  if (id && !view) {
    return (<Redirect to={`/jobs/${id}/time-series`} />);
  } else {
    const simulationDetails = simulations.find(s => s.simulation_id === parseInt(id));
    return (<div className="row jobs-list">
      <div className="col-3">
        <ul className="list-group scrollable">
          {simulations.map(s =>
            <Job
              key={s.simulation_id}
              simulationId={s.simulation_id}
              status={s.status} />
          )}
        </ul>
      </div>

      <div className="col-9 left-border scrollable">
        {simulationDetails && id && <JobDetails simulationId={parseInt(id)} details={simulationDetails} />}
      </div>

    </div>
    );
  }
}
