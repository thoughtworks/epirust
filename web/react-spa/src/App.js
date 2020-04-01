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

import React, {useState} from 'react';
import Grid from "./grid/Grid";
import TimeSeries from "./time-series";
import SimulationConfiguration from "./simulation-configuration";

function NavItem({name, isActive, onClickHandle}) {
  const activeClassName = isActive ? "active" : "";
  return <li className={`nav-item ${activeClassName}`}>
    <a onClick={onClickHandle} className="nav-link" href="#">{name} </a>
  </li>;
}

function App() {
  const views = {
    HOME: "home",
    TIME_SERIES: "ts",
    GRID: "grid"
  };

  const [viewState, updateViewState] = useState({currentView: views.HOME});

  const onHomeClick = () => updateViewState({currentView: views.HOME});

  const onTimeSeriesClick = () => updateViewState({currentView: views.TIME_SERIES});

  const onGridClick = () => updateViewState({currentView: views.GRID});

  const isHomeActive = () => viewState.currentView === views.HOME;

  const isTimeSeriesActive = () => viewState.currentView === views.TIME_SERIES;

  const isGridActive = () => viewState.currentView === views.GRID;

  function getComponent() {
    switch (viewState.currentView) {
      case views.HOME:
        return <SimulationConfiguration/>;
      case views.GRID:
        return <Grid size={50}/>;
      case views.TIME_SERIES:
        return <TimeSeries/>;
      default:
        return <SimulationConfiguration/>;
    }
  }

  return (
    <>
      <nav className="navbar navbar-expand-lg navbar-dark bg-dark">
        <a className="navbar-brand" href="/">EpiViz</a>
        <button className="navbar-toggler" type="button" data-toggle="collapse" data-target="#navbarSupportedContent"
                aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
          <span className="navbar-toggler-icon"/>
        </button>
        <div className="collapse navbar-collapse" id="navbarSupportedContent">
          <ul className="navbar-nav mr-auto">
            <NavItem name={"Home"} isActive={isHomeActive()} onClickHandle={onHomeClick}/>
            <NavItem name={"Time Series"} isActive={isTimeSeriesActive()} onClickHandle={onTimeSeriesClick}/>
            <NavItem name={"Grid Visualization"} isActive={isGridActive()} onClickHandle={onGridClick}/>
          </ul>
        </div>
      </nav>
      <div className="container-fluid mt-4">
        {getComponent()}
      </div>
    </>
  );
}

export default App;