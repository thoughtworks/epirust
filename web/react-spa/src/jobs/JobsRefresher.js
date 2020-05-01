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

import {get} from "../common/apiCall";
import {reduceStatus} from "./JobTransformer";
import {LOADING_STATES} from "../common/constants";

export class JobsRefresher {
  constructor(updateJobs, updateLoadingState) {
    this.updateJobs = updateJobs;
    this.updateLoadingState = updateLoadingState;
  }

  #refreshJobs = (jobsToProcess) => {
    const jobsToFetch = jobsToProcess.filter(j => j.status !== "finished")
    if (jobsToFetch.length > 0) {
      const url = `/jobs?jobIds=${jobsToFetch.map(j => j._id).join(",")}`
      get(url)
        .then(res => res.json())
        .then(jobsToUpdate => {
          this.updateJobs(prevJobs => {
            const updatedJobs = prevJobs.map(pj => {
              const jobToUpdate = jobsToUpdate.find(ju => ju._id === pj._id);
              return jobToUpdate ? reduceStatus(jobToUpdate) : pj;
            });

            setTimeout(() => this.#refreshJobs(updatedJobs), 15000);
            return updatedJobs;
          })
        })
    }
  }

  start = () => {
    get("/jobs")
      .then(res => res.json())
      .then(receivedJobs => {
        const convertedJobs = receivedJobs.map(reduceStatus).reverse();
        this.updateJobs(convertedJobs)
        this.updateLoadingState(LOADING_STATES.FINISHED)
        setTimeout(() => this.#refreshJobs(convertedJobs), 15000)
      })
      .catch(() => this.updateLoadingState(LOADING_STATES.FAILED))
  }
}