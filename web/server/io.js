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

const CountsIoController = require("./controllers/counts-io-controller");
const GridIoController = require("./controllers/grid-io-controller");
const JobStatusIoController = require("./controllers/job-status-io-controller");


module.exports = function setupIO(ioInstance) {
  ioInstance
    .of('/counts')
    .on('connection', (socket) => {
      socket.on('get', message => CountsIoController.handleRequest(socket, message.jobId));
      socket.on('disconnect', reason => console.log("Disconnect", reason));
    });

  ioInstance
    .of('/grid-updates')
    .on('connection', (socket) => {
      socket.on('get', (getRequest) => GridIoController.handleRequest(socket, getRequest.jobId))
      socket.on('disconnect', reason => console.log("Disconnect", reason));
    });

  ioInstance
    .of('/job-status')
    .on('connection', JobStatusIoController.handleRequest)
};
