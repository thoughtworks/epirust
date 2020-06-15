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

const interventionSchema = new Schema({
  intervention: String,
  data: Object
}, {_id: false});

const countSchema = new Schema({
  simulation_id: {type: mongoose.ObjectId, required: true},
  hour: Number,
  susceptible: Number,
  exposed: Number,
  infected: Number,
  hospitalized: Number,
  recovered: Number,
  deceased: Number,
  interventions: {type: [interventionSchema], default: undefined}
});

const Count = mongoose.model('Count', countSchema, "counts");
module.exports = {Count};

