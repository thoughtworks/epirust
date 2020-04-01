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

import React from 'react';
import Grid from "./grid/Grid";
import TimeSeries from "./time-series";
import SimulationConfiguration from "./simulation-configuration";
import Header from './Header';

import {
  BrowserRouter as Router,
  Switch,
  Route
} from "react-router-dom";

export const Routes = {
  HOME: "/",
  TIME_SERIES: "/time-series",
  GRID: "/grid"
};

function App() {
  return (
    <Router>
      <Header />
      <div className="container-fluid mt-4">
        <Switch>
          <Route exact path={Routes.HOME}><SimulationConfiguration /></Route>
          <Route path={Routes.GRID}><Grid size={50} /></Route>
          <Route path={Routes.TIME_SERIES}><TimeSeries /></Route>
        </Switch>
      </div >
    </Router >
  );
}

export default App;