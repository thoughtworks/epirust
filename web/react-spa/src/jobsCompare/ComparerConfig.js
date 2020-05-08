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

import Select from "react-select";
import React from "react";
import {epiCurves} from "../common/constants";

export function ComparerConfig({selectedJobs, updateSelectedCurves, updateInterventions}) {
  const selectedInterventions = {}
  const updateSelectedInterventions = (job, info) => {
    selectedInterventions[job] = info
  }
  const handleSubmit = (e) => {
    e.preventDefault();

    const interventionsToAsk = []
    if(selectedInterventions.job1) interventionsToAsk.push(...selectedInterventions.job1)
    if(selectedInterventions.job2) interventionsToAsk.push(...selectedInterventions.job2)
    updateInterventions(interventionsToAsk)
  }

  return <div className="comparer-config">
    <label htmlFor="series-filer">Select line-graphs to show:</label>
    <Select
      options={epiCurves.map(e => ({value: e, label: e}))}
      isMulti
      inputId="series-filer"
      name="tags"
      aria-label="tags"
      onChange={(cs) => updateSelectedCurves(cs ? cs.map(c => c.value) : [])}
    />

    <form onSubmit={handleSubmit}>
      <label>Select interventions to show</label>
      Job 1:
      <Select
        options={selectedJobs.job1.simulations.map(s => ({value: s._id, label: s._id}))}
        isMulti
        inputId="job1-interventions-filter"
        name="job1-interventions"
        aria-label="tags"
        onChange={(cs) => updateSelectedInterventions("job1", cs ? cs.map(c => c.value) : [])}
      />

      Job 2:
      <Select
        options={selectedJobs.job2.simulations.map(s => ({value: s._id, label: s._id}))}
        isMulti
        inputId="job2-interventions-filter"
        name="job2-interventions"
        aria-label="tags"
        onChange={(cs) => updateSelectedInterventions("job2", cs ? cs.map(c => c.value) : [])}
      />
      <button className="btn btn-primary">Show Interventions</button>
    </form>
  </div>
}
