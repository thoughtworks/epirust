/* GET simulation listing. */
const express = require('express');
const router = express.Router();
const KafkaServices = require('../services/kafka');
var ioInstance = require('../io');


router.post('/', (req, res, next) => {
  const kafkaConsumer = new KafkaServices.KafkaConsumerService('localhost:9092', 'counts_updated', 1);
  const io = ioInstance();

  io.once('connect', function (socket) {

    kafkaConsumer.consumer.resumeTopics(['counts_updated']);

    kafkaConsumer.consumer.on('message', function (message) {
      socket.emit('epidemicStats', message.value);
    });

    socket.on('disconnecting', reason => {
      kafkaConsumer.consumer.pauseTopics(['counts_updated']);
    });

    socket.on('disconnect', reason => console.log("Disconnect", reason));
  });

  res.sendStatus(200);
})

router.post('/init', (req, res, next) => {
  const message = req.body; // { disease_name, grid_size, number_of_agents, simulation_hrs, public_transport_percentage, working_percentage, vaccinate_at, vaccinate_percentage }
  const simulation_config = {
    "population": {
      "Auto": {
        "number_of_agents": message.number_of_agents,
        "public_transport_percentage": message.public_transport_percentage,
        "working_percentage": message.working_percentage
      }
    },
    //TODO Currently hardcoded. These params must come from the UI
    "disease": {
      "regular_transmission_start_day": 10,
      "high_transmission_start_day": 16,
      "last_day": 22,
      "regular_transmission_rate": 0.05,
      "high_transmission_rate": 0.5,
      "death_rate": 0.2
    },
    "grid_size": message.grid_size,
    "hours": message.simulation_hrs,
    "interventions": [
      {
        "Vaccinate": {
          "at_hour": message.vaccinate_at,
          "percent": message.vaccinate_percentage
        }
      }
    ]
  };
  const kafkaProducer = new KafkaServices.KafkaProducerService();

  kafkaProducer.send('simulation_requests', simulation_config);

  res.status(200);
  res.send({ status: "Simulation started" });
});

module.exports = router;
