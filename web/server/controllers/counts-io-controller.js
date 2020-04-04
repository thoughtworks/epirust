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

const {Simulation} = require("../db/models/Simulation");
const Count = require("../db/models/Count");

function sendCountsData(socket, lastConsumedHour) {
  const findLastRecordQuery = Simulation.findOne({}, {simulation_id: 1}, {sort: {'_id': -1}});
  const promise = findLastRecordQuery.exec();

  promise.then(async (simulation) => {
    let cursor = Count.find({
      simulation_id: simulation.simulation_id,
      hour: {$gt: lastConsumedHour}
    }, {}, {sort: {'hour': 1}}).cursor();

    let currentHour;
    for await(const data of cursor) {
      currentHour = data.hour;
      socket.emit('epidemicStats', data);
    }
    const findLastRecordQuery = Simulation.findOne({}, {grid_consumption_finished: 1}, {sort: {'_id': -1}});
    const promise = findLastRecordQuery.exec();

    await promise.then((simulation) => {
      if (simulation.grid_consumption_finished) {
        socket.emit('epidemicStats', {"simulation_ended": true});
      } else sendCountsData(socket, currentHour);
    })
  });
}

const handleRequest = (socket) => {
  sendCountsData(socket, 0);
  socket.on('disconnect', reason => console.log("Disconnect", reason));
};

module.exports = {
  handleRequest
};