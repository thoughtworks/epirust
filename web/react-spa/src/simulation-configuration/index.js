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
import DiseaseDynamics from './DiseaseDynamics'
import Interventions from './Interventions'
import MiscellaneousConfig from "./MiscellaneousConfig";
import {useHistory} from "react-router-dom";
import {post} from "../common/apiCall";
import TagsSelect from "./TagsSelect";

export default function SimulationConfiguration() {
  const [buttonDisabled, setButtonDisabled] = useState(false);
  const [errorOccurred, setErrorOccurred] = useState(false);

  const history = useHistory();

  function pushData(paramsData) {
    post("/jobs/init", paramsData)
      .then(res => res.json())
      .then(data => history.push(`/jobs/${data.jobId}`))
      .catch(err => {
        console.error(err);
        setButtonDisabled(false);
        setErrorOccurred(true)
      })
  }

  function handleSubmit(e) {
    setButtonDisabled(true);
    e.preventDefault();
    let paramsData = {};
    console.log(new FormData(e.target), "---------------");
    new FormData(e.target).forEach(function (value, key) {
      if ([
        "number_of_agents",
        "grid_size",
        "simulation_hrs",
        "public_transport_percentage",
        "working_percentage",
        "vaccinate_at",
        "vaccinate_percentage",
        "death_rate",
        "high_transmission_rate",
        "high_transmission_start_day",
        "last_day",
        "regular_transmission_rate",
        "regular_transmission_start_day",
        "lockdown_at_number_of_infections",
        "essential_workers_population",
        "hospital_spread_rate_threshold",
        "percentage_asymptomatic_population",
        "percentage_severe_infected_population",
        "exposed_duration",
        "pre_symptomatic_duration",
        "number_of_simulations"
      ].includes(key)) {
        value = Number(value);
      }
      if (key === "tags") {

        paramsData["tags"] = Array.isArray(paramsData["tags"])
          ? [...paramsData["tags"], value]
          : (value ? [value] : [])

      } else paramsData[key] = value;
    });

    if ("generate_grid_vis" in paramsData) {
      const value = paramsData["generate_grid_vis"];
      delete paramsData.generate_grid_vis;
      paramsData["enable_citizen_state_messages"] = value === "on";
    } else {
      paramsData["enable_citizen_state_messages"] = false;
    }

    pushData(paramsData)
  }

  function renderPopulation() {
    return (
      <fieldset>
        <legend>Population</legend>
        <div className="input-control">
          <label className="col-form-label-sm" htmlFor="number_of_agents">Number of Agents</label>
          <input type="number" name="number_of_agents" className="form-control form-control-sm" id="number_of_agents"
                 aria-describedby="number_of_agents" placeholder="Number of Agents" defaultValue="10000"/>
        </div>

        <div className="input-control">
          <label className="col-form-label-sm" htmlFor="public_transport_percentage">Public Transport Percentage</label>
          <input type="number" name="public_transport_percentage" className="form-control form-control-sm"
                 id="public_transport_percentage" aria-describedby="public_transport_percentage"
                 placeholder="Public Transport Percentage" defaultValue="0.2" step="any"/>
        </div>

        <div className="input-control">
          <label className="col-form-label-sm" htmlFor="working_percentage">Working Percentage</label>
          <input type="number" name="working_percentage" className="form-control form-control-sm"
                 id="working_percentage" aria-describedby="working_percentage" placeholder="Working Percentage" min="0"
                 max="1" defaultValue="0.7" step="any"/>
        </div>
      </fieldset>
    )
  }

  const loading = () => (<>
    <span className="spinner-grow spinner-grow-lg"/>
    <span className="button-text"> Submitting..</span>
  </>);

  return (
    <div className="graph-input">
      <form className="user-inputs" onSubmit={handleSubmit} data-testid='simulationForm'>

        <div className="form-row">

          <div className="col inputs">
            <div>
              {renderPopulation()}
              <MiscellaneousConfig/>
            </div>
            <DiseaseDynamics/>
            <Interventions/>
          </div>
          <div className="form-row tag-action">
            <TagsSelect/>
            {/*<div className="col actions">*/}
            <button type="submit" className="btn btn-primary btn-lg" id="submitBtn" disabled={buttonDisabled}>
              {buttonDisabled ? loading() : "Submit"}
            </button>
            {/*</div>*/}
          </div>
          {errorOccurred && <div className="error-message alert-danger">Error Occurred please try again!</div>}
        </div>

      </form>
    </div>
  )
}
