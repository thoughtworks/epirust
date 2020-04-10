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

import "./job-details.scss"
import { NavItem } from "../common/NavItem";
import { Redirect, Route, Switch } from "react-router-dom";
import GridPage from "../grid";
import PropTypes from 'prop-types';
import { TimeSeriesDeviation, TimeSeries } from "../time-series";

export const JobDetails = ({ simulationId, details }) => {
  const linkPrefix = `/jobs/${simulationId}`;

  const isGridEnabled = details && details.config.enable_citizen_state_messages,
    isFinished = details && details.status === 'finished';

  function renderTabs() {
    return (
      <div className="row">
        <div className="col-8">
          <ul className="nav nav-tabs nav-fill">
            <NavItem name="Time Series" linksTo={`${linkPrefix}/time-series`} activeOnExactMatch={true} />
            {isFinished && <NavItem name="Time Series Deviation" linksTo={`${linkPrefix}/time-series-deviation`} />}
            {isGridEnabled && <NavItem name="Grid" linksTo={`${linkPrefix}/grid`} />}
            <NavItem name="Config" linksTo={`${linkPrefix}/config`} />
          </ul>
        </div>
        <div className="col-4">
          <h4 className="simulation-id">Simulation: {simulationId}</h4>
        </div>
      </div>
    );
  }

  function renderContentForTab() {
    return (
      <Switch>

        <Route exact path={"/jobs/:id/time-series"}>
          <TimeSeries simulationId={simulationId} />
        </Route>

        <Route exact path={"/jobs/:id/time-series-deviation"}>
          <TimeSeriesDeviation simulationId={simulationId} />
        </Route>

        <Route exact path={"/jobs/:id/grid"}>
          {isGridEnabled ? <GridPage /> : <Redirect to={`/jobs/${simulationId}/time-series`} />}
        </Route>

        <Route exact path={"/jobs/:id/config"}>
          {details && <pre>{JSON.stringify(details.config, undefined, 4)}</pre>}
        </Route>
      </Switch>
    );
  }


  return (
    <div className="job-details" style={{ paddingTop: 4 }}>
      {renderTabs()}
      {renderContentForTab()}
    </div>
  )
};

JobDetails.propTypes = {
  simulationId: PropTypes.number.isRequired,
  details: PropTypes.object
};
