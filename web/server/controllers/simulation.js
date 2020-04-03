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
const Simulation = require("../db/models/Simulation").Simulation;

router.post('/init', (req, res, next) => {
  const message = req.body;
  const {
    number_of_agents,
    public_transport_percentage,
    working_percentage,
    regular_transmission_start_day,
    high_transmission_start_day,
    last_day,
    regular_transmission_rate,
    high_transmission_rate,
    death_rate,
    grid_size,
    simulation_hrs,
    enable_citizen_state_messages
  } = message;

  let simulationId = Date.now();
  const simulation_config = {
    "sim_id": `${simulationId}`,
    "enable_citizen_state_messages": enable_citizen_state_messages,
    "population": {
      "Auto": {
        "number_of_agents": number_of_agents,
        "public_transport_percentage": public_transport_percentage,
        "working_percentage": working_percentage
      }
    },
    "disease": {
      "regular_transmission_start_day": regular_transmission_start_day,
      "high_transmission_start_day": high_transmission_start_day,
      "last_day": last_day,
      "regular_transmission_rate": regular_transmission_rate,
      "high_transmission_rate": high_transmission_rate,
      "death_rate": death_rate
    },
    "grid_size": grid_size,
    "hours": simulation_hrs,
    "interventions": modelInterventions(message)
  };
  const kafkaProducer = new KafkaServices.KafkaProducerService();
  kafkaProducer.send('simulation_requests', simulation_config);

  const query = {simulation_id: simulationId};
  const simulation = Simulation.update(query, query, {upsert: true});
  simulation.exec()
    .catch((err) => console.error("Failed to create Simulation entry ", err));

  res.status(200);
  res.send({ status: "Simulation started" });
});

module.exports = router;

function modelInterventions(message) {
  const { vaccinate_at, vaccinate_percentage, lockdown_at_number_of_infections, essential_workers_population, hospital_spread_rate_threshold } = message;

  const areVaccinationParamsPresent = vaccinate_at && vaccinate_percentage,
    vaccinationIntervention = {
      "Vaccinate": {
        "at_hour": vaccinate_at,
        "percent": vaccinate_percentage
      },
    };
  const areLockdownParamsPresent = lockdown_at_number_of_infections && essential_workers_population,
    lockdownIntervention = {
      "Lockdown": {
        "at_number_of_infections": lockdown_at_number_of_infections,
        "essential_workers_population": essential_workers_population,
        "lock_down_period": 21
      }
    };

  const areHospitalSpreadParamsPresent = !!hospital_spread_rate_threshold,
    hospitalSpreadIntervention = {
      "BuildNewHospital": {
        "spread_rate_threshold": hospital_spread_rate_threshold
      }
    };

  return [
    ...(areVaccinationParamsPresent ? [vaccinationIntervention] : []),
    ...(areLockdownParamsPresent ? [lockdownIntervention] : []),
    ...(areHospitalSpreadParamsPresent ? [hospitalSpreadIntervention] : [])
  ];
}

