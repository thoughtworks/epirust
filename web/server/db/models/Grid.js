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

const mongoose = require("mongoose");
const Schema = mongoose.Schema;

const AreaSchema = new Schema({
  start_offset: {x: Number, y: Number},
  end_offset: {x: Number, y: Number},
  iter_index: {x: Number, y: Number}
}, {_id: false});

const citizenStatesSchema = new Schema({
  citizen_id: String,
  state: String,
  location: {
    x: Number,
    y: Number
  }
}, {_id: false})

const citizenStateSchema = new Schema({
  simulation_id: {type: mongoose.ObjectId, required: true},

  hr: {type: Number},
  citizen_states: [citizenStatesSchema]
});

const gridSchema = new Schema({
  simulation_id: {type: mongoose.ObjectId, required: true},

  grid_size: Number,
  housing_area: AreaSchema,
  work_area: AreaSchema,
  transport_area: AreaSchema,
  hospital_area: AreaSchema,
  houses: [AreaSchema],
  offices: [AreaSchema]
});


const Grid = mongoose.model('Grid', gridSchema, 'Grid');
const CitizenState = mongoose.model('CitizenState', citizenStateSchema, 'Grid');

module.exports = {Grid, CitizenState};