var express = require('express');
var path = require('path');
var logger = require('morgan');
var router = require('./routes/router');

var app = express();  

app.use(logger('dev'));
app.use(express.json());
app.use(express.urlencoded({ extended: false }));
app.use(express.static(path.join(__dirname, 'public')));

app.use('/', router);

// TODO: Add error handler

const server = require('http').createServer(app);

server.listen(4200);

module.exports = app;