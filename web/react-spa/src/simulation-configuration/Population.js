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

export default function Population() {
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
