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

const {Simulation} = require("../models/Simulation");

const updateSimulationStatus = (simulationId, status) => {
  const query = {_id: simulationId};
  const update = {status};
  return Simulation.updateOne(query, update, {upsert: true}).exec()
};

const markGridConsumptionFinished = (simulationId) => {
  const query = {_id: simulationId};
  const update = {grid_consumption_finished: true};
  const simulationUpdate = Simulation.updateOne(query, update, {upsert: true});
  return simulationUpdate.exec()
};

const fetchSimulation = (simulationId, projectionFields = []) => {
  const projection = projectionFields.reduce((acc, cur) => {acc[cur] = 1; return acc;}, {});
  return Simulation.findOne({_id: simulationId}, projection).exec()
        .then((doc) => {
            if(!doc) {
                throw Error(`Simulation with id: ${simulationId} not found`)
            }
            return doc.toObject()
        })
};

const saveSimulation = (simulation) => {
    return Simulation(simulation).save()
};

module.exports = {
    updateSimulationStatus,
    markGridConsumptionFinished,
    fetchSimulation,
    saveSimulation
};