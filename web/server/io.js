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

module.exports = function setupIO(ioInstance) {
  const countIO = ioInstance
    .of('/counts')
    .on('connection', (socket) => {
      const findLastRecordQuery = Simulation.findOne({}, {counts: 1}, {sort: {'_id': -1}});
      const promise = findLastRecordQuery.exec();

      promise.then((doc) => {
        doc.counts.sort((a, b) => {
          if (a.hour < b.hour) return -1;
          else return 1;
        }).forEach(count => {
          socket.emit('epidemicStats', count);
        });

        socket.emit('epidemicStats', {"simulation_ended": true});

        socket.on('disconnect', reason => console.log("Disconnect", reason));
      });
    })
};
