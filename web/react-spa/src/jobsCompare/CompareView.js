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

import {ComparerConfig} from "./ComparerConfig";
import Graph from "../time-series/LineGraph";
import React, {useEffect, useState} from "react";
import {epiCurves} from "../common/constants";
import {get} from "../common/apiCall";
import {parseAnnotations} from "../time-series/utils";
import GraphUpdater from "./GraphUpdater";

export function CompareView({selectedJobs}) {
  const [graphData, updateGraphData] = useState([]);
  const [selectedCurves, updateSelectedCurves] = useState([]);
  const [annotations, updateAnnotations] = useState([]);

  function clearGraphDataAndConfig() {
    updateGraphData([]);
    updateAnnotations([]);
    updateSelectedCurves([])
  }

  const updateBuffer = (buffer) => {
    updateGraphData(prevData => prevData.concat(buffer))
  };

  useEffect(() => {
    clearGraphDataAndConfig();
    const {job1, job2} = selectedJobs;
    new GraphUpdater(updateBuffer, job1._id, job2._id).start();

  }, [selectedJobs]);

  const onUpdateInterventions = (interventionsAskedFor) => {
    const series = selectedCurves.length > 0 ? `${selectedCurves[0]}_job1` : "susceptible_job1";

    function getInterventionsForSimulation(simulationId) {
      return get(`/simulations/${simulationId}/interventions`)
        .then(j => j.json())
        .then(res => {
          return res
            .map(i => parseAnnotations(i.interventions, i.hour))
            .reduce((acc, cur) => acc.concat(cur), [])
            .map(i => ({...i, series}))
        })
    }

    Promise.all(interventionsAskedFor.map(getInterventionsForSimulation))
      .then(multiParsedAnnotations => {
        const annotations = multiParsedAnnotations
          .reduce((acc, cur) => acc.concat(cur), [])
          .sort((a, b) => a.x < b.x);
        updateAnnotations(annotations)
      })
      .catch(err => {
        console.log("Failed to load all interventions", err)
      })
  };

  const handleSelectedCurvesUpdate = (curves) => {
    if (curves.length > 0) {
      updateAnnotations(prevState => prevState.map(x => ({...x, series: `${curves[0]}_job1`})))
    }
    updateSelectedCurves(curves)
  };

  return (
    <div className="row">
      <div className="col-2">
        <ComparerConfig
          selectedJobs={selectedJobs}
          updateSelectedCurves={handleSelectedCurvesUpdate}
          updateInterventions={onUpdateInterventions}
        />
      </div>
      <div className="col-10">
        <div className="jobs-compare-chart">
          {graphData.length > 0 &&
          <Graph
            dataBuffer={makeCSV(graphData)}
            dygraphsOptions={dygraphOptions()}
            visibility={visibility(selectedCurves)}
            annotations={annotations}
          />
          }
        </div>
      </div>
    </div>
  );
}

const dygraphOptions = () => ({
  colors: ["#0A1045", "#00C2D1", "#F9E900", "#F6AF65", "#ED33B9",
    "#8588a2", "#80e1e8", "#fcf480", "#fbd7b2", "#f699dc"],
  labels: ["hour",
    ...epiCurves.map(e => `${e}_job1`),
    ...epiCurves.map(e => `${e}_job2`),
  ]
});

const visibility = (curvesSelected) => {
  const curves = [...epiCurves.map(e => `${e}_job1`), ...epiCurves.map(e => `${e}_job2`)];
  if (curvesSelected.length > 0) {
    return curves.map(l => curvesSelected.some(cs => l.includes(cs)))
  }
  return curves.map(_ => true)
};

const rowToCsv = (row) => {
  if (row) return [[row.susceptible, row.susceptible_std], [row.infected, row.infected_std],
    [row.hospitalized, row.hospitalized_std], [row.recovered, row.recovered_std], [row.deceased, row.deceased_std]];
  return [[], [], [], [], []]
};

const makeCSV = (graphData) => {
  return graphData.map(d => ([d.hour, ...rowToCsv(d.job1), ...rowToCsv(d.job2)]))
};
