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

import React, {useEffect, useState} from "react";
import './jobs-list.scss'
import {Job} from "./Job";
import {JobDetails} from "./JobDetails";
import {Redirect, useParams} from 'react-router-dom';

export const JobsList = () => {
  const {id, view} = useParams();
  const [simulations, updateSimulations] = useState([]);

  useEffect(() => {
    fetch("http://localhost:3000/simulation/",
      {
        method: 'GET',
        headers: {'Content-Type': 'application/json'}
      })
      .then(value => value.json())
      .then(value => updateSimulations(value.reverse()))
  }, []);

  if (id && !view) {
    return (<Redirect to={`/jobs/${id}/time-series`}/>);
  } else {
    return (<div className="row jobs-list">
      <div className="col-3">
        <ul className="list-group scrollable">
          {simulations.map(s => <Job key={s.simulation_id} simulationId={s.simulation_id} status={s.status}/>)}
        </ul>
      </div>
      <div className="col-9">
        {id && <JobDetails simulationId={id}/>}
      </div>
    </div>);
  }
};
