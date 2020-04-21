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
const {fetchSimulation} = require('../db/services/SimulationService')
const {fetchCountsInSimulation} = require('../db/services/CountService')

async function sendCountsData(simulationId, socket, totalConsumedRecords) {
  const cursor = fetchCountsInSimulation(simulationId, totalConsumedRecords)

  if(socket.disconnected){
    cursor.close();
    return ;
  }

  let recordsConsumedInThisGo = 0;
  for await(const data of cursor) {
    if(socket.disconnected){
      cursor.close();
      return ;
    }
    recordsConsumedInThisGo += 1;
    socket.emit('epidemicStats', data);
  }
  await fetchSimulation(simulationId, ['status'])
      .then((simulation) => {
        if (simulation.status === SimulationStatus.FINISHED || simulation.status === SimulationStatus.FAILED) {
          if (socket.disconnected)
            return;
          socket.emit('epidemicStats', {"simulation_ended": true});
        } else sendCountsData(simulationId, socket, totalConsumedRecords + recordsConsumedInThisGo);
      });
}

const handleRequest = (socket) => {
  socket.on('simulation_id', (message) => {
    sendCountsData(parseInt(message), socket, 0);
  });

  socket.on('disconnect', reason => console.log("Disconnect", reason));
};

module.exports = {
  handleRequest
};