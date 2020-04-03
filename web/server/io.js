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

const Simulation = require("./db/models/Simulation").Simulation;
const Count = require("./db/models/Count");

module.exports = function setupIO(ioInstance) {
  const countIO = ioInstance
    .of('/counts')
    .on('connection', (socket) => {
      const findLastRecordQuery = Simulation.findOne({}, {simulation_id: 1}, {sort: {'_id': -1}});
      const promise = findLastRecordQuery.exec();

      promise.then(async (doc) => {
        let cursor = Count.find({simulation_id: doc.simulation_id}, {}, {sort: {'hour': 1}}).cursor();

        for await(const data of cursor) {
          socket.emit('epidemicStats', data);
        }
        socket.emit('epidemicStats', {"simulation_ended": true});
      });
      socket.on('disconnect', reason => console.log("Disconnect", reason));
    });
};
