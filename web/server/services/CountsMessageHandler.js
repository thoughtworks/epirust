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

const {SimulationStatus} = require("../db/models/Simulation");
const SimulationService = require('../db/services/SimulationService');
const CountService = require('../db/services/CountService');

const {toObjectId} = require('../common/util')

class CountsMessageHandler {
  async handleMessage(data) {
    const parsedMessage = JSON.parse(data.value);

    const simulationId = toObjectId(data.key.toString());

    const simulationEnded = "simulation_ended" in parsedMessage;
    const isInterventionMessage = "intervention" in parsedMessage;

    if (simulationEnded) {
      await SimulationService.updateSimulationStatus(simulationId, SimulationStatus.FINISHED);
      console.log("Consumed all messages for ", simulationId);
    } else if (isInterventionMessage) {
      await CountService.addIntervention(simulationId, parsedMessage);
    } else {
      await this._handleCountMessage(parsedMessage, simulationId);
    }
  }

  async _handleCountMessage(parsedMessage, simulationId) {
    if (parsedMessage.hour === 1)
      SimulationService.updateSimulationStatus(simulationId, SimulationStatus.RUNNING)

    await CountService.upsertCount(simulationId, parsedMessage);
  }
}

module.exports = {CountsMessageHandler};
