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
const NotFound = require('../exceptions/NotFound')
const {Job} = require('../models/Job');
const {range} = require('../../common/util');
const {SimulationStatus} = require('../models/Simulation')

const saveJob = (config, numSimulations) => {
  const simulations = range(numSimulations).map(() => ({status: SimulationStatus.INQUEUE}))
  return new Job({config, simulations}).save()
}

const fetchJob = (jobId) => {
  return Job.findOne({_id: jobId}).exec()
    .then(doc => {
      if(!doc)
        throw new NotFound(jobId)
      return doc;
    })
}

const fetchJobs = (jobIds) => {
  if(jobIds && jobIds.length > 0){
    return Job.find({_id: {$in: jobIds}}).cursor()
  }
  return Job.find().cursor()
}

module.exports = { saveJob, fetchJob, fetchJobs }
