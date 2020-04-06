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

const app = require('../../app');
const supertest = require('supertest');
const request = supertest(app);
const Simulation = require('../../db/models/Simulation').Simulation
const mongoose = require('mongoose')
const { MongoMemoryServer } = require('mongodb-memory-server')


// May require additional time for downloading MongoDB binaries
jasmine.DEFAULT_TIMEOUT_INTERVAL = 600000;

let mongoServer;

beforeAll(async done => {
    mongoServer = new MongoMemoryServer();
    const mongoUri = await mongoServer.getUri();
    await mongoose.connect(mongoUri, (err) => {
        if (err) console.error(err);
    });
    done()
});

afterAll(async done => {
    await mongoose.disconnect();
    await mongoServer.stop();
    done()
});

test('should get all simulations from database', async done => {
    const simulation = {
        "_id": "5e883fc4a2f0353799b71671",
        "simulation_id": 1585987524000.0,
        "__v": 0,
        "grid_consumption_finished": true
    }
    await Simulation.create(simulation).then(async () => {
        const simulations = await request.get('/simulation')
        expect(simulations.body).toEqual([simulation])
        done()
    })
});

test('should get a specific simulation from database using simulation id', async done => {
    Simulation.deleteOne({}, () => { })
    const simulation1 = {
        "_id": "5e883fc4a2f0353799b71671",
        "simulation_id": 1585987524000,
        "__v": 0,
        "grid_consumption_finished": true
    }
    const simulation2 = {
        "_id": "5e883fc4a2f0353799b79987",
        "simulation_id": 15859875242267,
        "__v": 0,
        "grid_consumption_finished": true
    }
    await Simulation.create(simulation1, simulation2).then(async () => {
        await request.get('/simulation')
            .query({ "simulation_id": 1585987524000 })
            .then(response => {
                expect(response.body).toEqual([simulation1])
                done()
            })
    })
});
