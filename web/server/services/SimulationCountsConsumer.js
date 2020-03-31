const KafkaServices = require('../services/kafka');
const config = require("../config");
const {Simulation, SimulationStatus} = require("../db/models/Simulation");

class SimulationCountsConsumer {
  constructor(simulationId) {
    this.kafkaConsumer = new KafkaServices.KafkaConsumerService(config.KAFKA_URL, config.COUNTS_TOPIC, 1);
    this.simulationId = simulationId;

    this._createSimulationEntry(simulationId)
  }


  start() {
    this.kafkaConsumer.consumer.resumeTopics([config.COUNTS_TOPIC]);

    this.kafkaConsumer.consumer.on("message", (message) => {
      let messageString = message.value.toString();
      const query = {simulation_id: this.simulationId};

      if (messageString.startsWith("{\"simulation_ended")) {
        this.kafkaConsumer.consumer.pause();
        this.kafkaConsumer.close();

        const update = {status: SimulationStatus.FINISHED};
        const simulation = Simulation.findOneAndUpdate(query, update);
        let promise = simulation.exec();

        promise.then((value) => {
          //exit the process
        })
      } else {
        const parsedMessage = JSON.parse(messageString);
        let findOneAndUpdateQuery = Simulation.findOneAndUpdate(query, {$push: {counts: parsedMessage}});
        findOneAndUpdateQuery.exec()
      }
    })
  }

  async _createSimulationEntry(simulationId) {
    const sim = new Simulation({simulation_id: simulationId});
    await sim.save()
  };
}

module.exports = {SimulationCountsConsumer};
