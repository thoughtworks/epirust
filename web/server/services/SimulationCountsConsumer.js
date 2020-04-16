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
      const interventionMessage = "intervention" in parsedMessage;

      if (simulationEnded) {
        await this.handleSimulationEnd(query, simulationId);
      } else if (interventionMessage) {
        //ignore this message for now to avoid breaking ui
      } else {
        await this.handleCountMessage(parsedMessage, simulationId, query);
      }
    }
  }

  async handleCountMessage(parsedMessage, simulationId, query) {
    parsedMessage["simulation_id"] = simulationId;
    const countInsertQuery = new Count(parsedMessage);

    if (parsedMessage.hour === 1) {
      const update = {status: SimulationStatus.RUNNING};
      Simulation.updateOne(query, update, {upsert: true}).exec();
    }

    await countInsertQuery.save()
  }

  async handleSimulationEnd(query, simulationId) {
    const update = {status: SimulationStatus.FINISHED};
    const simulation = Simulation.updateOne(query, update, {upsert: true});

    console.log("Consumed all counts for simulation id", simulationId);
    await simulation.exec()
  }
}

module.exports = {SimulationCountsConsumer};
