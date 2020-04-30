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
import Loader from "../common/Loader";
import {get} from "../common/apiCall";
import {reduceStatus} from "./JobTransformer";

export const JobsList = () => {
  const {id: paramId, view} = useParams();
  const [jobs, updateJobs] = useState([]);
  const [isLoading, setIsLoading] = useState(true);

  const refreshJobs = (jobsToProcess) => {
    const jobsToFetch = jobsToProcess.filter(j => j.status !== "finished")
    console.log(jobsToFetch)
    if (jobsToFetch.length > 0) {
      const url = `/jobs?jobIds=${jobsToFetch.map(j => j._id).join(",")}`
      get(url)
        .then(res => res.json())
        .then(jobsToUpdate => {
          updateJobs(prevJobs => {
            const updatedJobs = prevJobs.map(pj => {
              const jobToUpdate = jobsToUpdate.find(ju => ju._id === pj._id);
              return jobToUpdate ? reduceStatus(jobToUpdate) : pj;
            });

            setTimeout(() => refreshJobs(updatedJobs), 15000);
            return updatedJobs;
          })
        })

    }
  }

  useEffect(() => {
    get("/jobs")
      .then(res => res.json())
      .then(receivedJobs => {
        const convertedJobs = receivedJobs.map(reduceStatus).reverse();
        updateJobs(convertedJobs)
        setIsLoading(false)
        setTimeout(() => refreshJobs(convertedJobs), 15000)
      })
  }, []);

  function renderSimulationTabs() {
    return (
      <div className="col-2">
        <ul className="list-group scrollable">
          {jobs.map(s =>
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
    const job = jobs.find(j => j._id === paramId)
    return job && (
      <div className="col-10 left-border scrollable details">
        <JobDetails jobId={paramId} config={job.config}/>
      </div>
    );
  }

  if (isLoading) {
    return <Loader/>
  }

  if (!paramId && jobs.length) {
    return (<Redirect to={`/jobs/${jobs[0]._id}/time-series`}/>);
  }

  if (paramId && !view) {
    return (<Redirect to={`/jobs/${paramId}/time-series`}/>);
  }

  return (
    <div className="row jobs-list">
      {renderSimulationTabs()}
      {renderDetails()}
    </div>
  );

}

