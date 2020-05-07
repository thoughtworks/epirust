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

const {Count} = require('../models/Count');

const addIntervention = (simulationId, intervention) => {
  const query = Count.updateOne(
    {simulation_id: simulationId, hour: intervention.hour},
    {$addToSet: {interventions: intervention}},
    {upsert: true}
  );

  return query.exec()
};

const upsertCount = (simulationId, count) => {
  const query = Count.updateOne({simulation_id: simulationId, hour: count.hour},
    count, {upsert: true});

  return query.exec();
};

const fetchCountsInSimulation = (simulationId, skipBy) => {
  return Count.find({simulation_id: simulationId}, {__v: 0, _id: 0})
      .sort({hour: 1})
      .skip(skipBy)
      .cursor();
};

const aggregateSimulations = (simulationIds) => {
  return Count
    .aggregate([
      { $match: { simulation_id: { $in: simulationIds } } },
      {
        $group: {
          _id: '$hour',
          infected: { $avg: '$infected' },
          susceptible: { $avg: '$susceptible' },
          hospitalized: { $avg: '$hospitalized' },
          recovered: { $avg: '$recovered' },
          deceased: { $avg: '$deceased' },
          infected_std: { $stdDevPop: '$infected' },
          susceptible_std: { $stdDevPop: '$susceptible' },
          hospitalized_std: { $stdDevPop: '$hospitalized' },
          recovered_std: { $stdDevPop: '$recovered' },
          deceased_std: { $stdDevPop: '$deceased' },
        }
      },
      {$addFields: {hour: '$_id'}},
      { $sort: { _id: 1 } }
    ])
}

const fetchInterventionsForSimulation = (simulationId) => {
  return Count.find({simulation_id: simulationId, "interventions.0": {$exists: 1}}, {hour: 1, interventions: 1, _id: 0}, {sort: {hour: 1}})
    .then(x => x.map(d => d.toObject()))
}

module.exports = {addIntervention, upsertCount, fetchCountsInSimulation, aggregateSimulations, fetchInterventionsForSimulation};