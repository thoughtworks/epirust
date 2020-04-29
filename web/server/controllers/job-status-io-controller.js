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

const JobService = require('../db/services/JobService')

async function sendJobsStatus(socket) {
    const cursor = JobService.fetchJobs()

    let jobStatus = []
    for await (const data of cursor) {
        if (socket.disconnected)
            cursor.close()

        jobStatus.push(data)
    }

    socket.emit('jobStatus', jobStatus);
}

async function handleRequest(socket) {
    sendJobsStatus(socket)
    const intervalId = setInterval(async() => await sendJobsStatus(socket), 15000)

    socket.on('disconnect', reason => {
        clearInterval(intervalId)
        console.log("Disconnect job-status socket:", reason)
    });
}

module.exports = {
    handleRequest
};