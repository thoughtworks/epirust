const express = require('express');
const http = require('http');
const path = require('path');
const logger = require('morgan');
const debug = require('debug')('epirust-server');
const router = require('./routes/router');
const ioInstance = require('./io');
const app = express();
const cors = require('cors');

var server = http.createServer(app);
server.listen(3000);
server.on('error', onError);
server.on('listening', onListening);
ioInstance(server);
app.use(cors({origin:'http://localhost:3001'}));
app.use(logger('dev'));
app.use(cors());
app.use(express.json());
app.use(express.urlencoded({ extended: false }));
app.use(express.static(path.join(__dirname, 'public')));
app.use('/', router);

function onError(error) {
    if (error.syscall !== 'listen') {
      throw error;
    }
  
    var bind = typeof port === 'string'
      ? 'Pipe ' + port
      : 'Port ' + port;
  
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
    var addr = server.address();
    var bind = typeof addr === 'string'
      ? 'pipe ' + addr
      : 'port ' + addr.port;
    debug('Listening on ' + bind);
  }
