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
const {Grid} = require("../db/models/Grid");
const Simulation = require("../db/models/Simulation").Simulation;


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
      const simulationId = data.key.toString();
      parsedMessage["simulation_id"] = parseInt(simulationId);

      if('simulation_ended' in parsedMessage) {
        const query = {simulation_id: simulationId};
        const update = {simulation_id: simulationId, grid_consumption_finished: true};
        const simulationUpdate = Simulation.updateOne(query, update, {upsert: true});

        console.log("Grid messages consumption finished for simulation id", simulationId);
        await simulationUpdate.exec()
      }
      else if('grid_size' in parsedMessage) {
        const grid = new Grid(parsedMessage);
        grid.citizen_states = undefined;

        await grid.save();
      } else {
        const citizenState = new Grid(parsedMessage);
        citizenState.houses = undefined;
        citizenState.offices = undefined;
        await citizenState.save();
      }
    }
  }
}

module.exports = {SimulationGridConsumer};
