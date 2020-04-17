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


const KafkaServices = require('../services/kafka');
const config = require("../config");
const {Grid, CitizenState} = require("../db/models/Grid");
const SimulationService = require('../db/services/SimulationService')
const GridService = require('../db/services/GridService')


class SimulationGridConsumer {
  constructor() {
    this.kafkaConsumer = new KafkaServices.KafkaGroupConsumer(
      config.KAFKA_URL,
      config.CITIZEN_STATES_UPDATED,
      config.KAFKA_GROUP
    );
  }

  async start() {
    for await (const data of this.kafkaConsumer.consumerStream) {
      const parsedMessage = JSON.parse(data.value);
      const simulationId = parseInt(data.key.toString());
      parsedMessage["simulation_id"] = simulationId;

      if('simulation_ended' in parsedMessage) {
        await SimulationService.markGridConsumptionFinished(simulationId);

        console.log("Grid messages consumption finished for simulation id", simulationId);
      }
      else if('grid_size' in parsedMessage) {
        await GridService.saveGridLayout(parsedMessage)
      } else {
        const updateQuery = CitizenState.updateOne({simulation_id: simulationId, hr: parsedMessage.hr}, parsedMessage, {upsert: true});
        await updateQuery.exec()
      }
    }
  }
}

module.exports = {SimulationGridConsumer};
