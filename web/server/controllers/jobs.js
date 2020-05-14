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

const express = require('express');
const router = express.Router();
const KafkaServices = require('../services/kafka');
const { SimulationStatus } = require("../db/models/Simulation");
const {updateSimulationStatus} = require('../db/services/SimulationService')
const JobService = require('../db/services/JobService');
const {toObjectId} = require("../common/util")
const {fetchJob} = require("../db/services/JobService")
const NotFound = require("../db/exceptions/NotFound")
const {predefinedTags} = require("../db/resources/predefined-tags")

router.post('/init', (req, res) => {
  const simulation_config = makeSimulationConfig(req.body);
  console.log(JSON.stringify(simulation_config));
  const kafkaProducer = new KafkaServices.KafkaProducerService();
  let jobId;

  JobService.saveJob(simulation_config, simulation_config.number_of_simulations)
    .then((job) => {
      jobId = job._id;
      return Promise.all(job.simulations.map(s => {
        return kafkaProducer
          .send('simulation_requests', {...simulation_config, sim_id: s._id.toString()})
          .catch(err => {
            console.error("Error occurred while sending kafka message", err);
            return updateSimulationStatus(s._id, SimulationStatus.FAILED)
              .then(() => {throw new Error(err.message)});
        })
      }))
    })
    .then(() => {
      res.status(201);
      res.send({ status: "Job started", jobId });
    })
    .catch((err) => {
      res.status(500);
      res.send({ message: err.message });
      console.error("Failed to create Simulation entry ", err);
    });
});

router.get('/tags', (req, res) => {
  res.send(predefinedTags);
});

router.get('/', (req, res) => {
  const jobIds = req.query.jobIds && req.query.jobIds.split(",")
  JobService.fetchJobsWithTagDetails(jobIds)
  .then(jobsStatus => {
    return res.send(jobsStatus);
  })
  .catch(() => {
    console.log("Error occurred while fetching document")
    res.sendStatus(500)
  })
});

router.get('/:job_id', (req, res) => {
  fetchJob(toObjectId(req.params.job_id))
      .then((job) => {
        res.status(200);
        res.send(job);
      })
      .catch((error) => {
        switch (error.constructor) {
          case NotFound:
            res.status(404);
            res.send({message: error.message});
            break;
          default:
            res.status(500);
            res.send();
        }
      })
});

function makeSimulationConfig(message) {
  const {
    number_of_agents,
    public_transport_percentage,
    working_percentage,
    regular_transmission_start_day,
    high_transmission_start_day,
    last_day,
    asymptomatic_last_day,
    mild_infected_last_day,
    regular_transmission_rate,
    high_transmission_rate,
    death_rate,
    grid_size,
    simulation_hrs,
    enable_citizen_state_messages,
    percentage_asymptomatic_population,
    percentage_severe_infected_population,
    exposed_duration,
    pre_symptomatic_duration,
    number_of_simulations,
    tags,
    hospital_beds_percentage
  } = message;

  return {
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
      "asymptomatic_last_day": asymptomatic_last_day,
      "mild_infected_last_day": mild_infected_last_day,
      "regular_transmission_rate": regular_transmission_rate,
      "high_transmission_rate": high_transmission_rate,
      "death_rate": death_rate,
      "percentage_asymptomatic_population": percentage_asymptomatic_population,
      "percentage_severe_infected_population": percentage_severe_infected_population,
      "exposed_duration": exposed_duration,
      "pre_symptomatic_duration": pre_symptomatic_duration
    },
    "geography_parameters" : {
      "grid_size": grid_size,
      "hospital_beds_percentage":hospital_beds_percentage,
    },
    "hours": simulation_hrs,
    "interventions": modelInterventions(message),
    "number_of_simulations": number_of_simulations,
    "tags": tags
  };
}

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
        "essential_workers_population": essential_workers_population
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

module.exports = router;


