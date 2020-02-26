/* GET simulation listing. */
const app = require('express');
const EpirustService = require('../services/epirust');
const KafkaConsumerService = require('../services/kafka');

const router = app.Router();

router.get('/', function(req, res, next) {
  res.send("In simulation");
});

router.post('/', function(req, res, next) {
  const numberOfAgents = req.body.numberOfAgents;
  const epirustService = new EpirustService(numberOfAgents);
  const kafkaConsumer = new KafkaConsumerService('localhost:9092', 'counts_updated', 1);
  epirustService.start(numberOfAgents);
  res.status(201);
  res.send("Simulation started");
});

module.exports = router;
