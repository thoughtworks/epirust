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
  const message = req.body; 
  const simulation_config = {
    "population": {
      "Auto": {
        "number_of_agents": message.number_of_agents,
        "public_transport_percentage": message.public_transport_percentage,
        "working_percentage": message.working_percentage
      }
    },
    "disease": {
      "regular_transmission_start_day": message.regular_transmission_start_day,
      "high_transmission_start_day": message.high_transmission_start_day,
      "last_day": message.last_day,
      "regular_transmission_rate": message.regular_transmission_rate,
      "high_transmission_rate": message.high_transmission_rate,
      "death_rate": message.death_rate
    },
    "grid_size": message.grid_size,
    "hours": message.simulation_hrs,
    "interventions": [
      {
        "Vaccinate": {
          "at_hour": message.vaccinate_at,
          "percent": message.vaccinate_percentage
        },
        "Lockdown": {
          "at_number_of_infections": message.lockdown_at_number_of_infections,
          "emergency_workers_population": message.emergency_workers_population
        },
        "BuildNewHospital": {
          "spread_rate_threshold": message.hospital_spread_rate_threshold
        }
      }
    ]
  };
  console.log(simulation_config)
  const kafkaProducer = new KafkaServices.KafkaProducerService();

  kafkaProducer.send('simulation_requests', simulation_config);

  res.status(200);
  res.send({ status: "Simulation started" });
});

module.exports = router;
