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

const {KafkaStreamProcessor} = require("./services/KafkaStreamProcessor");
const mongoose = require('mongoose');
const {CountsMessageHandler} = require("./services/CountsMessageHandler");
const {GridMessageHandler} = require("./services/GridMessageHandler");
const config = require('./config');

mongoose.connect(config.DATABASE_URL, {useNewUrlParser: true, useUnifiedTopology: true});
mongoose.Promise = global.Promise;
const db = mongoose.connection;
db.on('error', console.error.bind(console, 'MongoDB connection error:'));
mongoose.set('useCreateIndex', true);


const countsMessageHandler = new CountsMessageHandler();
const kafkaCountsStreamProcessor = new KafkaStreamProcessor(countsMessageHandler, config.COUNTS_TOPIC)

const gridMessageHandler = new GridMessageHandler();
const kafkaGridStreamProcessor = new KafkaStreamProcessor(gridMessageHandler, config.GRID_MESSAGE_TOPIC)

kafkaCountsStreamProcessor.start()
kafkaGridStreamProcessor.start();
