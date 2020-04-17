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
  describe('addIntervention', function () {
    it('should add intervention to the count', async function () {
      const simulationId = randomId();
      const interventionObject = buildTestIntervention();
      await new Count({simulation_id: simulationId, hour: 12}).save();

      await CountService.addIntervention(simulationId, interventionObject);

      const count = await Count.findOne({simulation_id: simulationId, hour: 12}).exec();
      const receivedCount = count.toObject();
      expect(receivedCount.interventions).toHaveLength(1);
      expect(receivedCount.interventions[0]).toMatchObject({
        intervention: interventionObject.intervention,
        data: interventionObject.data
      });
    });

    it('should not duplicate interventions if same added twice', async function () {
      const simulationId = randomId();
      const interventionObject = buildTestIntervention();
      await new Count({simulation_id: simulationId, hour: 12}).save();

      await CountService.addIntervention(simulationId, interventionObject);
      await CountService.addIntervention(simulationId, interventionObject);

      const count = await Count.findOne({simulation_id: simulationId, hour: 12}).exec();
      const receivedCount = count.toObject();
      expect(receivedCount.interventions).toHaveLength(1);
      expect(receivedCount.interventions[0]).toMatchObject({
        intervention: interventionObject.intervention,
        data: interventionObject.data
      });
    });
  });

  beforeAll(async () => await dbHandler.connect());
  afterEach(async () => await dbHandler.clearDatabase());
  afterAll(async () => await dbHandler.closeDatabase());

  const randomId = () => Math.random();

  const buildTestIntervention = () => {
    const interventionName = "test-intervention";
    const interventionData = {status: "done"};
    return {hour: 12, intervention: interventionName, data: interventionData};
  };
});