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


export default class GraphUpdater {
  #updateBuffer
  #jobData1
  #jobData2
  #lastSent
  #stopAll

  constructor(updateBuffer, jobId1, jobId2) {
    this.#updateBuffer = updateBuffer;

    this.#jobData1 = new JobTimeSeriesData(jobId1)
    this.#jobData2 = new JobTimeSeriesData(jobId2)
    this.#lastSent = 0;
    this.#stopAll = false;
  }

  start() {
    this.#startJob(this.#jobData1);
    this.#startJob(this.#jobData2);
  }

  stop() {
    this.#jobData1.closeSocket();
    this.#jobData2.closeSocket();
  }

  #checkConsistencyAndSend = (sendLimit) => {
    const start = this.#lastSent + 1;
    const end = start + sendLimit - 1;
    const isConsistent = this.#checkConsistency(end)

    if (isConsistent) {
      const data = this.#buildData(start, end)
      if(!this.#stopAll) {
        this.#updateBuffer(data)
        this.#lastSent = end;
      }
    }
  }

  #startJob = (jobData) => {
    const socket = io(`${config.API_HOST}/counts`)
    jobData.setSocket(socket);
    socket.on('epidemicStats', (message) => {
      if ("simulation_ended" in message) {
        jobData.fetchFinished = true;
        if (this.#bothJobsFinished())
          this.#checkConsistencyAndSend(jobData.consumedCount() - this.#lastSent)
        socket.close()
      } else {
        jobData.insertData(message.hour, message)
        if (jobData.consumedCount() % config.RENDER_COUNT === 0) {
          this.#checkConsistencyAndSend(config.RENDER_COUNT)
        }
      }
    })
    socket.emit('get', {jobId: jobData.jobId});
  }

  #bothJobsFinished = () => {
    return this.#jobData1.fetchFinished && this.#jobData2.fetchFinished;
  }

  #checkConsistency = (till) => {
    const eitherJobsNotConsistent = this.#jobData1.consumedCount() < till || this.#jobData2.consumedCount() < till
    return !(!this.#bothJobsFinished() && eitherJobsNotConsistent);
  }

  #buildData = (from, till) => {
    const data = []
    for (let i = from; i <= till; i++) {
      data.push({'hour': i, 'job1': this.#jobData1.dataBuffer[i], 'job2': this.#jobData2.dataBuffer[i]})
    }
    return data;
  }
}

class JobTimeSeriesData {
  #consumedMessageCount
  #socket

  constructor(jobId) {
    this.jobId = jobId
    this.fetchFinished = false;
    this.dataBuffer = {}
    this.#consumedMessageCount = 0;
  }

  insertData = (hour, message) => {
    this.dataBuffer[hour] = message
    this.#consumedMessageCount += 1
  }

  consumedCount = () => this.#consumedMessageCount

  setSocket = (socket) => this.#socket = socket;

  closeSocket = () => this.#socket.close();
}
