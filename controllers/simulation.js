/* GET simulation listing. */
const express = require('express');
const router = express.Router();
const EpirustService = require('../services/epirust');
const KafkaConsumerService = require('../services/kafka');

var ioInstance = require('../io');

router.post('/', function (req, res, next) {
  const numberOfAgents = req.body.numberOfAgents;
  const epirustService = new EpirustService(numberOfAgents);
  const kafkaConsumer = new KafkaConsumerService('localhost:9092', 'counts_updated', 1);
  epirustService.start(numberOfAgents);
  const io = ioInstance();
  io.on('connect', function (socket) {
    console.log('a user connected');
    kafkaConsumer.consumer.on('message', function (message) {
      socket.emit('epidemicStats', message);
    });
  });
  res.status(200);
  res.send({ status: "Simulation started" });
});

module.exports = router;
