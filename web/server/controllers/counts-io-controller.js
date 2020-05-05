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

const {SimulationStatus} = require("../db/models/Simulation");
const {fetchCountsInSimulation, aggregateSimulations} = require('../db/services/CountService')
const {toObjectId} = require('../common/util')
const {fetchJob} = require('../db/services/JobService')
const {fetchSimulation} = require('../db/services/SimulationService')

const handleRequest = (socket, jobId) => {
  const mongoJobId = toObjectId(jobId);
  fetchJob(mongoJobId)
    .then(job => {
      const simulations = job.simulations;
      if(simulations.length === 0) return; //TODO: remove this when job is required to have at least one simulation

      if(simulations.length === 1) {
        // this can be shown live
        return sendCountsData(simulations[0]._id, socket, 0);
      } else {
        return handleMultiSimulationRequest(mongoJobId, socket)
      }
    })
    .catch(err => {
      console.log("error", err.message);
      socket.emit('error', {message: err.message})
    })
};

async function sendCountsData(simulationId, socket, totalConsumedRecords) {
  const cursor = fetchCountsInSimulation(simulationId, totalConsumedRecords)

  if(socket.disconnected){
    cursor.close();
    return ;
  }

  let recordsConsumedInThisGo = 0;
  for await(const document of cursor) {
    if(socket.disconnected){
      cursor.close();
      return ;
    }
    recordsConsumedInThisGo += 1;
    socket.emit('epidemicStats', {...placeholderData, ...document.toObject()});
  }
  return await fetchSimulation(simulationId)
    .then((simulation) => {
      if (simulation.status === SimulationStatus.FINISHED || simulation.status === SimulationStatus.FAILED) {
        if (socket.disconnected)
          return;
        socket.emit('epidemicStats', {"simulation_ended": true});
      } else sendCountsData(simulationId, socket, totalConsumedRecords + recordsConsumedInThisGo);
    });
}

const sendAggregatedCounts = async (simulations, socket) => {
  const cursor = aggregateSimulations(simulations.map(s => s._id));
  for await (const data of cursor) {
    socket.emit('epidemicStats', data)
  }
}

const handleMultiSimulationRequest = async (jobId, socket) => {
  const simulations = (await fetchJob(jobId)).simulations;

  const anySimulationFailed = simulations.reduce((acc, cur) => acc || cur.status === SimulationStatus.FAILED, false);
  if(anySimulationFailed)
    throw new Error("One of the simulations failed to finish")

  const allSimulationsFinished = simulations.reduce((acc, cur) => acc && cur.status === SimulationStatus.FINISHED, true);
  if(allSimulationsFinished) {
    await sendAggregatedCounts(simulations, socket)
    socket.emit('epidemicStats', {"simulation_ended": true})
  } else {
    setTimeout(() => handleMultiSimulationRequest(jobId, socket), 2000);
  }
}

const placeholderData = {
  infected_std: 0,
  susceptible_std: 0,
  hospitalized_std: 0,
  recovered_std: 0,
  deceased_std: 0,
}

module.exports = {
  handleRequest
};