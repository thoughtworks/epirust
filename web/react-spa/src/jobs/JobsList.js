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
import './jobs-list.scss'
import {Job} from "./Job";
import {JobDetails} from "./JobDetails";
import {Redirect, useParams} from 'react-router-dom';

export const JobsList = () => {
  const {id, view} = useParams();
  const simulations = [
    {simulation_id: 123, status: "running"},
    {simulation_id: 143, status: "finished"},
    {simulation_id: 163, status: "failed"},
    {simulation_id: 173, status: "finished"},
    {simulation_id: 15453, status: "finished"},
    {simulation_id: 1233, status: "finished"},
    {simulation_id: 12343, status: "finished"},
    {simulation_id: 172343, status: "finished"},
    {simulation_id: 17133, status: "finished"},
    {simulation_id: 1723433, status: "finished"},
    {simulation_id: 17233, status: "finished"},
    {simulation_id: 178233, status: "finished"},
    {simulation_id: 17453, status: "finished"},
    {simulation_id: 17553, status: "finished"},
    {simulation_id: 1752353, status: "finished"},
  ];

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
