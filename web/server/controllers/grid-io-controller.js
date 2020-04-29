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


const { SimulationStatus } = require("../db/models/Simulation");
const {fetchSimulation} = require('../db/services/SimulationService')
const {findSortedById} = require('../db/services/GridService')
const {toObjectId} = require('../common/util');
const {fetchJob} = require("../db/services/JobService")

async function sendGridData(simulationId, socket, totalConsumerRecords) {
  const cursor = findSortedById(simulationId, totalConsumerRecords);

  if(socket.disconnected){
    cursor.close();
    return;
  }

  let countOfMessagesConsumed = 0;
  for await (const data of cursor) {
    if(socket.disconnected) {
      cursor.close();
      return;
    }
    countOfMessagesConsumed += 1;
    socket.emit('gridData', data);
  }

  await fetchSimulation(simulationId, ['status', 'grid_consumption_finished', 'config.enable_citizen_state_messages'])
      .then(simulation => {
        if (simulation.grid_consumption_finished || simulation.status === SimulationStatus.FAILED) {
          if (socket.disconnected)
            return;
          socket.emit('gridData', {"simulation_ended": true});
        } else if (!simulation.config.enable_citizen_state_messages) {
          if (socket.disconnected)
            return;
          socket.emit('gridData', {message: "no grid data"});
        } else sendGridData(simulationId, socket, totalConsumerRecords + countOfMessagesConsumed);
      });
}

const handleRequest = (socket, jobId) => {
  fetchJob(toObjectId(jobId))
    .then(({simulations}) => {
      if(simulations.length === 0)
        throw new Error("No simulation for provided jobId or invalid job-id provided")

      if(simulations.length > 1)
        throw new Error("Grid visualization is not supported for multi-simulation job")

      return sendGridData(simulations[0]._id, socket, 0)
    })
    .catch(err => socket.emit('error', {message: err.message}))
}

module.exports = {
  handleRequest
};