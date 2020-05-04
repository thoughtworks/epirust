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
import Header from './Header';

import {BrowserRouter as Router, Route, Switch} from "react-router-dom";
import {JobsView} from "./jobs";
import SimulationConfiguration from "./simulation-configuration";
import JobsCompare from "./jobsCompare/JobsCompare";

export const Routes = {
  HOME: "/",
  JOBS: "/jobs",
  COMPARE: "/compare"
};

function App() {
  return (
    <Router>
      <Header />
      <div className="container-fluid mt-4">
        <Switch>
          <Route exact path={Routes.HOME}><SimulationConfiguration /></Route>
          <Route path={`${Routes.JOBS}/:id?/:view?`}><JobsView /></Route>
          <Route path={Routes.COMPARE}><JobsCompare /></Route>
        </Switch>
      </div >
    </Router >
  );
}

export default App;