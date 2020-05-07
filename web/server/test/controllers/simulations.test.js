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

const router = require("../../routes/router")
const express = require("express");
const app = express();
const supertest = require('supertest');
app.use(express.json());
app.use(express.urlencoded({extended: false}));
app.use("/", router);
const request = supertest(app);
const {mockObjectId} = require("../helpers")
const CountsService = require("../../db/services/CountService")
jest.mock("../../db/services/CountService")

describe('Simulation Controller', () => {
  afterAll(async () => {
    await app.close()
  });

  afterEach(() => {
    jest.clearAllMocks()
  });

  describe('/simulations', () => {
    describe("/:sim_id/interventions", () => {
      it('should return interventions for given simulation', async () => {
        const simulationId = mockObjectId()
        const interventions = [{hour: 1, interventions: ["hosiptal"]}];
        CountsService.fetchInterventionsForSimulation.mockResolvedValueOnce(interventions)

        const response = await request.get(`/simulations/${simulationId.toString()}/interventions`)

        expect(CountsService.fetchInterventionsForSimulation).toHaveBeenCalledTimes(1)
        expect(CountsService.fetchInterventionsForSimulation).toHaveBeenCalledWith(simulationId)
        expect(response.body).toEqual(interventions)
        expect(response.status).toBe(200)
      });
    })
  });

});
