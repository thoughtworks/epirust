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
const { Count } = require("../db/models/Count");

const express = require('express');
const router = express.Router();
const KafkaServices = require('../services/kafka');
const { Simulation, SimulationStatus } = require("../db/models/Simulation");

const configMatch = {
  "config.population.Auto.number_of_agents": 10000,
  "config.population.Auto.public_transport_percentage": 0.2,
  "config.population.Auto.working_percentage": 0.7,

  "config.disease.regular_transmission_start_day": 5,
  "config.disease.high_transmission_start_day": 20,
  "config.disease.last_day": 40,
  "config.disease.regular_transmission_rate": 0.025,
  "config.disease.high_transmission_rate": 0.25,
  "config.disease.death_rate": 0.2,
  "config.percentage_asymptomatic_population": 0.3,
  "config.percentage_severe_infected_population": 0.3,
  "config.exposed_duration": 48,

  "config.hours": 10000,
  "config.grid_size": 250,
  "config.interventions": { $size: 0 }
};

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
    enable_citizen_state_messages,
    percentage_asymptomatic_population,
    percentage_severe_infected_population,
    exposed_duration,
  } = message;

  console.log(typeof percentage_asymptomatic_population, percentage_asymptomatic_population)
  console.log(typeof percentage_severe_infected_population, percentage_severe_infected_population)
  console.log(typeof exposed_duration, exposed_duration)

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
      "death_rate": death_rate,
      "percentage_asymptomatic_population": percentage_asymptomatic_population,
      "percentage_severe_infected_population": percentage_severe_infected_population,
      "exposed_duration": exposed_duration
    },
    "grid_size": grid_size,
    "hours": simulation_hrs,
    "interventions": modelInterventions(message)
  };
  const { sim_id, ...configToStore } = simulation_config;
  const updateQuery = {
    simulation_id: simulationId,
    status: SimulationStatus.INQUEUE,
    config: configToStore
  };
  const simulation = new Simulation(updateQuery);
  simulation.save()
    .then(() => {
      const kafkaProducer = new KafkaServices.KafkaProducerService();
      return kafkaProducer.send('simulation_requests', simulation_config).catch(err => {
        console.error("Error occurred while sending kafka message", err);
        return Simulation.updateOne({ simulation_id: simulationId }, { status: SimulationStatus.FAILED })
          .exec().then(() => {
            throw new Error(err.message)
          });
      })
    })
    .then(() => {
      res.status(201);
      res.send({ status: "Simulation started", simulationId });
    })
    .catch((err) => {
      res.status(500);
      res.send({ message: err.message });
      console.error("Failed to create Simulation entry ", err);
    });
});

router.get('/', async (req, res, next) => {
  if (req.query.simulation_id) {
    Simulation.find({ "simulation_id": req.query.simulation_id }, function (err, simulation) {
      res.json(simulation)
    })
  } else {
    Simulation.find({}, function (err, simulations) {
      res.json(simulations)
    })
  }
});

async function extractFromCursor(stream) {
  const aggregate = [];
  for await (const data of stream) {
    aggregate.push(data)
  }
  return aggregate;
}

router.get("/:simulation_id/time-series-deviation", async (req, res, next) => {

  const simulationToAggregate = Simulation.find(configMatch, { simulation_id: 1 })
    .exec()
    .then(async (docs) => {
      let simulationIds = docs.map(a => a.simulation_id);

      const aggregateStream = Count
        .aggregate([
          { $match: { simulation_id: { $in: simulationIds } } },
          {
            $group: {
              _id: '$hour',
              infected_mean: { $avg: '$infected' },
              susceptible_mean: { $avg: '$susceptible' },
              quarantined_mean: { $avg: '$quarantined' },
              recovered_mean: { $avg: '$recovered' },
              deceased_mean: { $avg: '$deceased' },
              infected_std: { $stdDevPop: '$infected' },
              susceptible_std: { $stdDevPop: '$susceptible' },
              quarantined_std: { $stdDevPop: '$quarantined' },
              recovered_std: { $stdDevPop: '$recovered' },
              deceased_std: { $stdDevPop: '$deceased' },
            }
          },
          { $sort: { _id: 1 } }
        ]);

      const aggregate = await extractFromCursor(aggregateStream);

      const countsCursor = Count.find({ simulation_id: parseInt(req.params.simulation_id) }, {}, { sort: { hour: 1 } })
        .cursor();

      const counts = await extractFromCursor(countsCursor);

      if (counts.length < aggregate.length) {
        res.json(counts.map((c, i) => ({ ...c.toObject(), ...aggregate[i] })));
      } else {
        res.json(aggregate.map((a, i) => ({ ...a, ...counts[i].toObject() })));
      }
    })

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

