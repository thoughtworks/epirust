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
import TimeSeries from "../time-series";
import "./job-details.scss"
import {NavItem} from "../common/NavItem";
import {Redirect, Route, Switch} from "react-router-dom";
import GridPage from "../grid";
import PropTypes from 'prop-types'

export const JobDetails = ({simulationId, details}) => {
  const linkPrefix = `/jobs/${simulationId}`;

  const isGridEnabled = details && details.config.enable_citizen_state_messages;

  return (
    <div className="job-details" style={{paddingTop:4}}>
      <h4>Simulation: {simulationId}</h4>
      <ul className="nav nav-tabs">
        <NavItem name="Time Series" linksTo={`${linkPrefix}/time-series`} activeOnExactMatch={true}/>
        {isGridEnabled && <NavItem name="Grid" linksTo={`${linkPrefix}/grid`}/>}
        <NavItem name="Config" linksTo={`${linkPrefix}/config`}/>
      </ul>
      <Switch>
        <Route exact path={"/jobs/:id/time-series"}><TimeSeries simulationId={simulationId}/></Route>
        <Route exact path={"/jobs/:id/grid"}>
            {isGridEnabled ? <GridPage/> : <Redirect to={`/jobs/${simulationId}/time-series`}/>}
        </Route>}
        <Route exact path={"/jobs/:id/config"}>
            {details && <pre>{JSON.stringify(details.config, undefined, 4)}</pre>}
        </Route>
      </Switch>
    </div>
  )
};

JobDetails.propTypes = {
    simulationId: PropTypes.number.isRequired,
    details: PropTypes.object.isRequired
};
