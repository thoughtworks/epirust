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
import io from "socket.io-client";
import config from "../config";

const RENDER_COUNT = 300;

export default class GraphUpdater {
  constructor(updateBuffer, jobId1, jobId2) {
    this.updateBuffer = updateBuffer;
    this.jobId1 = jobId1
    this.jobId2 = jobId2
    this.csvData = {}
    this.lastSent = 0;
    this.job1Ended = false;
    this.job2Ended = false;
  }

  start() {
    this.startJob(this.jobId1, "job1");
    this.startJob(this.jobId2, "job2");
  }

  #checkAndSend = (sendLimit) => {
    let shouldSend = true;
    const tempBuffer = []
    for (let i = this.lastSent + 1; i <= this.lastSent + sendLimit; i++) {
      if (this.csvData[i]) {
        if (!this.job1Ended && !this.job2Ended) {
          const bothPresent = this.csvData[i]["job1"] && this.csvData[i]["job2"]
          if (!bothPresent) {
            shouldSend = false;
            break;
          }
        }
      } else {
        shouldSend = false;
        break;
      }
      tempBuffer.push({...this.csvData[i], hour: i})
    }

    if (shouldSend) {
      this.updateBuffer(tempBuffer)
      this.lastSent = this.lastSent + sendLimit;
    }
  }

  startJob(jobId, jobName) {
    let consumedCount = 0;
    const socket = io(`${config.API_HOST}/counts`)
    socket.emit('get', {jobId});

    socket.on('epidemicStats', (message) => {
      if ("simulation_ended" in message) {
        if (jobName === 'job1') this.job1Ended = true
        else this.job2Ended = true;
        if (this.job1Ended && this.job2Ended) {
          this.#checkAndSend(consumedCount - this.lastSent)
        }
        socket.close()
        return;
      }

      const hour = message.hour;

      if (this.csvData[hour]) {
        this.csvData[hour][jobName] = message
      } else {
        this.csvData[hour] = {[jobName]: message}
      }

      consumedCount += 1;
      if (jobName === "job1" && consumedCount % RENDER_COUNT === 0) {
        this.#checkAndSend(RENDER_COUNT)
      }
    })
  }
}
