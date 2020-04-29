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

const {Job} = require("../models/Job")
const NotFound = require("../exceptions/NotFound")

const updateSimulationStatus = (simulationId, status) => {
  const query = {"simulations._id": simulationId};
  const update = {"simulations.$.status": status};
  return Job.updateOne(query, update).exec()
};

const markGridConsumptionFinished = (simulationId) => {
  const query = {"simulations._id": simulationId};
  const update = {"simulations.$.grid_consumption_finished": true};
  const simulationUpdate = Job.updateOne(query, update);
  return simulationUpdate.exec()
};

const fetchSimulation = (simulationId) => {
  const query = {"simulations._id": simulationId};
  const findSimulationQuery = Job.findOne(query, {"simulations.$": 1});
  return findSimulationQuery.exec()
    .then(doc => {
      if(!doc) {
        throw new NotFound(simulationId)
      }
      return doc.simulations[0]
    })
}

module.exports = {
  updateSimulationStatus,
  markGridConsumptionFinished,
  fetchSimulation
};