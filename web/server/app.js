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

const express = require('express');
const http = require('http');
const path = require('path');
const logger = require('morgan');
const debug = require('debug')('epirust-server');
const router = require('./routes/router');
const cors = require('cors');
const mongoose = require('mongoose');
const config = require("./config");

const app = express();

if (process.env.NODE_ENV !== "test") {
  mongoose.connect(config.DATABASE_URL, { useNewUrlParser: true, useUnifiedTopology: true });
  mongoose.Promise = global.Promise;
  mongoose.set('useCreateIndex', true);
  const db = mongoose.connection;
  db.on('error', console.error.bind(console, 'MongoDB connection error:'));
}


const server = http.createServer(app);
server.listen(config.PORT);
server.on('error', onError);
server.on('listening', onListening);

const setupIO = require("./io");
const io = require("socket.io")(server);
setupIO(io);


app.use(cors({ origin: config.CLIENT_URL }));
app.use(logger('dev'));
app.use(cors());
app.use(express.json());
app.use(express.urlencoded({ extended: false }));
app.use(express.static(path.join(__dirname, 'public')));

app.use('/api', router);

app.get('*', (req,res) =>{
  res.sendFile(path.join(__dirname+'/public/index.html'));
});

function onError(error) {
  if (error.syscall !== 'listen') {
    throw error;
  }

  const addr = server.address();
  const bind = typeof addr === 'string'
    ? 'pipe ' + addr
    : 'port ' + addr.port;

  // handle specific listen errors with friendly messages
  switch (error.code) {
    case 'EACCES':
      console.error(bind + ' requires elevated privileges');
      process.exit(1);
      break;
    case 'EADDRINUSE':
      console.error(bind + ' is already in use');
      process.exit(1);
      break;
    default:
      throw error;
  }
}

function onListening() {
  const addr = server.address();
  const bind = typeof addr === 'string'
    ? 'pipe ' + addr
    : 'port ' + addr.port;
  debug('Listening on ' + bind);
}

module.exports = server;
