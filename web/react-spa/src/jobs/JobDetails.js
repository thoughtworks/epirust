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

import "./job-details.scss"
import {NavItem} from "../common/NavItem";
import {Redirect, Route, Switch} from "react-router-dom";
import GridPage from "../grid";
import PropTypes from 'prop-types';
import {TimeSeries} from "../time-series";
import {get} from "../common/apiCall";

export const JobDetails = ({ jobId }) => {
  const linkPrefix = `/jobs/${jobId}`;
  const [gridEnabled, setGridEnabled] = useState(false)
  const [jobConfig, setJobConfig] = useState({})
  const [apiResolved, setApiResolved] = useState(false)

  useEffect(() => {
    get(`/jobs/${jobId}`)
      .then(res => res.json())
      .then(job => {
        setGridEnabled(job.config.enable_citizen_state_messages)
        setJobConfig(job.config)
        setApiResolved(true)
      })
      .catch(() => {
        setApiResolved(true)
        setJobConfig({error: "Failed to load job config"})
      })
  }, [jobId])

  function renderGraphTabs() {
    return (
      <div className="row">
        <div className="col-8">
          <ul className="nav nav-tabs nav-fill">

            <NavItem name="Time Series" linksTo={`${linkPrefix}/time-series`} activeOnExactMatch={true} />
            {gridEnabled && <NavItem name="Grid" linksTo={`${linkPrefix}/grid`} />}
            <NavItem name="Config" linksTo={`${linkPrefix}/config`} />

          </ul>
        </div>

        <div className="col-4">
          <h4 className="simulation-id">{`Simulation: `}<code>{jobId}</code></h4>
        </div>
      </div>
    );
  }

  function renderContentForTab() {
    return (
      <div className='job-details-content'>
        <Switch>

          <Route exact path={"/jobs/:id/time-series"}>
            <TimeSeries jobId={jobId} />
          </Route>

          <Route exact path={"/jobs/:id/grid"}>
            {gridEnabled ? <GridPage /> : <Redirect to={`/jobs/${jobId}/time-series`} />}
          </Route>

          <Route exact path={"/jobs/:id/config"}>
            {<pre>{JSON.stringify(jobConfig, undefined, 4)}</pre>}
          </Route>
        </Switch>
      </div>
    );
  }


  return (
    apiResolved ? (<div className="job-details">
        {renderGraphTabs()}
        {renderContentForTab()}
      </div>) : (<div/>)
  )
};

JobDetails.propTypes = {
  jobId: PropTypes.string.isRequired,
};
