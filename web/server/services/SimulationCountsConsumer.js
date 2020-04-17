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
const {Simulation, SimulationStatus} = require("../db/models/Simulation");
const {Count} = require("../db/models/Count");
const SimulationService = require('../db/services/SimulationService');

class SimulationCountsConsumer {
  constructor() {
    this.kafkaConsumer =
      new KafkaServices.KafkaGroupConsumer(config.KAFKA_URL, config.COUNTS_TOPIC, config.KAFKA_GROUP);
  }

  async start() {
    for await (const data of this.kafkaConsumer.consumerStream) {
      const parsedMessage = JSON.parse(data.value);

      let simulationId = parseInt(data.key.toString());
      const query = {simulation_id: simulationId};

      const simulationEnded = "simulation_ended" in parsedMessage;
      const isInterventionMessage = "intervention" in parsedMessage;

      if (simulationEnded) {
        await SimulationService.markSimulationEnd(simulationId);
        console.log("Consumed all messages for ", simulationId);
      } else if (isInterventionMessage) {
        const {intervention, data} = parsedMessage;
        const query = Count.updateOne(
          {simulation_id: simulationId, hour: parsedMessage.hour},
          {$addToSet: {interventions: {intervention, data}}},
          {upsert: true}
        );
        await query.exec()
      } else {
        await this.handleCountMessage(parsedMessage, simulationId, query);
      }
    }
  }

  async handleCountMessage(parsedMessage, simulationId, query) {
    parsedMessage["simulation_id"] = simulationId;
    const countInsertQuery = Count.updateOne(
      {simulation_id: simulationId, hour: parsedMessage.hour},
      parsedMessage,
      {upsert: true});

    if (parsedMessage.hour === 1) {
      const update = {status: SimulationStatus.RUNNING};
      Simulation.updateOne(query, update, {upsert: true}).exec();
    }

    await countInsertQuery.exec()
  }
}

module.exports = {SimulationCountsConsumer};
