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
const Grid = require("../db/models/Grid");

function sendGridData(socket, lastConsumedId) {
  const findLastRecordQuery = Simulation.findOne({}, {simulation_id: 1}, {sort: {'_id': -1}});
  const promise = findLastRecordQuery.exec();

  promise.then(async (simulation) => {
    let query = {simulation_id: simulation.simulation_id, _id: {$gt: lastConsumedId}};
    if(lastConsumedId === null) delete query._id;
    let cursor = Grid.find(query, {}, {sort: {'_id': 1}}).cursor();

    let id;
    for await(const data of cursor) {
      id = data._id;
      socket.emit('gridData', data);
    }
    const findLastRecordQuery = Simulation.findOne({}, {status: 1}, {sort: {'_id': -1}});
    const promise = findLastRecordQuery.exec();

    await promise.then((simulation) => {
      if (simulation.grid_consumption_finished) {
        socket.emit('gridData', {"simulation_ended": true});
      } else sendGridData(socket, id);
    })
  });
}

function handleRequest(socket) {
  sendGridData(socket, null);
  socket.on('disconnect', reason => console.log("Disconnect", reason));
}

module.exports = {
  handleRequest
};