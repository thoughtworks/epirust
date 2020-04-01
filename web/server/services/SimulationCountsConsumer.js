const KafkaServices = require('../services/kafka');
const config = require("../config");
const {Simulation, SimulationStatus} = require("../db/models/Simulation");

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

      if ("simulation_ended" in parsedMessage) {
        const update = {simulation_id: simulationId, status: SimulationStatus.FINISHED};
        const simulation = Simulation.update(query, update, {upsert: true});

        console.log("Consumed all counts for simulation id", simulationId);
        await simulation.exec()

      } else {
        let update = {simulation_id: simulationId, $push: {counts: parsedMessage}};
        let findOneAndUpdateQuery = Simulation.update(query, update, {upsert: true});

        await findOneAndUpdateQuery.exec()
      }
    }
  }
}

module.exports = {SimulationCountsConsumer};
