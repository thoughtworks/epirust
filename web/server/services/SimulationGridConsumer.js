const KafkaServices = require('../services/kafka');
const config = require("../config");
const {Grid} = require("../db/models/Grid");

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
            if('hr' in parsedMessage){
              let simulationId = parseInt(data.key.toString());
              const findQuery = {simulation_id: simulationId, hr: parsedMessage.hr};

              let update = {simulation_id: simulationId, ...parsedMessage};
              let updateQuery = Grid.update(findQuery, update, {upsert: true});

              await updateQuery.exec()
            }
        }
    }
}

module.exports = {SimulationGridConsumer};
