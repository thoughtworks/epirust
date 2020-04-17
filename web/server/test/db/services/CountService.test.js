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

const dbHandler = require('../db-handler');
const CountService = require("../../../db/services/CountService");
const {Count} = require('../../../db/models/Count');

describe('CountService', function () {
  beforeAll(async () => await dbHandler.connect());
  afterEach(async () => await dbHandler.clearDatabase());
  afterAll(async () => await dbHandler.closeDatabase());

  describe('addIntervention', function () {
    it('should add intervention to the count', async function () {
      const simulationId= 12434;
      const interventionName = "test-intervention";
      const interventionData = {status: "done"};
      const intervention = {hour: 12, intervention: interventionName, data: interventionData};
      await new Count({simulation_id: simulationId, hour: 12}).save();

      await CountService.addIntervention(simulationId, intervention);

      const count = await Count.findOne({simulation_id: simulationId, hour: 12}).exec();
      const receivedCount = count.toObject();
      expect(receivedCount.interventions).toHaveLength(1);
      expect(receivedCount.interventions[0]).toMatchObject({intervention: interventionName, data: interventionData});
    });
  });
});