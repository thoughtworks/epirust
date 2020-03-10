/* GET simulation listing. */
const express = require('express');
const router = express.Router();
const EpirustService = require('../services/epirust');
const KafkaServices = require('../services/kafka');

var ioInstance = require('../io');

router.post('/', function (req, res, next) {
  const numberOfAgents = req.body.numberOfAgents;
  const epirustService = new EpirustService(numberOfAgents);
  const kafkaConsumer = new KafkaServices.KafkaConsumerService('localhost:9092', 'counts_updated', 1);
  epirustService.start(numberOfAgents);
  const io = ioInstance();
  io.on('connect', function (socket) {
    kafkaConsumer.consumer.on('message', function (message) {
      socket.emit('epidemicStats', message.value);
    });
  });
  res.status(200);
  res.send({ status: "Simulation started" });
});

router.post('/init', (req, res, next) => {
  const message = req.body; // { disease_name, grid_size, number_of_agents, simulation_hrs, public_transport_percentage, working_percentage, vaccinate_at, vaccinate_percentage }

  const kafkaProducer = new KafkaServices.KafkaProducerService();

  kafkaProducer.send('simulation_requests', message)

  res.status(200);
  res.send({ status: "Simulation started" });
})

module.exports = router;