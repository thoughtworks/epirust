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
const {mockObjectId} = require('../../helpers');

describe('CountService', () => {
  describe('addIntervention', () => {
    it('should add intervention to the count', async () => {
      const simulationId = mockObjectId();
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

    it('should not duplicate interventions if same added twice', async () => {
      const simulationId = mockObjectId();
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

  describe('upsertCount', () => {
    it('should insert new count in the collection', async () => {
      const simulationId = mockObjectId();
      const countObject = {simulation_id: simulationId, hour: 12, infected: 67};

      await CountService.upsertCount(simulationId, countObject);

      const receivedCount = (await Count.findOne({simulation_id: simulationId, hour: 12}).exec()).toObject();
      expect(receivedCount).toBeDefined();
      expect(receivedCount.simulation_id).toEqual(simulationId);
      expect(receivedCount.hour).toBe(12);
      expect(receivedCount.infected).toBe(67);
    });

    it('should not create multiple count objects with same simulationId and hour on multiple inserts', async () => {
      const simulationId = mockObjectId();
      const countObject = {hour: 12, infected: 67};

      await CountService.upsertCount(simulationId, countObject);
      await CountService.upsertCount(simulationId, countObject);

      const receivedCounts = await Count.find({simulation_id: simulationId, hour: 12}).exec();
      expect(receivedCounts).toHaveLength(1);
      const count = receivedCounts[0].toObject();
      expect(count.simulation_id).toEqual(simulationId);
      expect(count.hour).toBe(12);
      expect(count.infected).toBe(67);
    });
  });

  describe('fetchCountsInSimulation', () => {
    it('should fetch the counts sorted by hour in ascending order', async () => {
      const simulationId = mockObjectId();
      const hour2Count = {simulation_id: simulationId, hour:2};
      await new Count(hour2Count).save()
      const hour1Count = {simulation_id: simulationId, hour:1};
      await new Count(hour1Count).save()

      const cursor = CountService.fetchCountsInSimulation(simulationId, 0);

      let receivedCounts = []
      for await(const data of cursor) {
        receivedCounts = receivedCounts.concat(data.toObject())
      }

      expect(receivedCounts)

      expect(receivedCounts).toEqual([hour1Count, hour2Count]);
    });

    it('should fetch the counts sorted by hour in ascending order and skip by 1', async () => {
      const simulationId = mockObjectId();
      const hour2Count = {simulation_id: simulationId, hour:2};
      await new Count(hour2Count).save()
      const hour1Count = {simulation_id: simulationId, hour:1};
      await new Count(hour1Count).save()

      const cursor = CountService.fetchCountsInSimulation(simulationId, 1);

      let receivedCounts = []
      for await(const data of cursor) {
        receivedCounts = receivedCounts.concat(data.toObject())
      }

      expect(receivedCounts)

      expect(receivedCounts).toEqual([hour2Count]);
    });
  });

  describe('fetchInterventionsForSimulation', () => {
    it('should fetch all the interventiosn for the simulation', async () => {
      const simulationId = mockObjectId()
      await new Count({simulation_id: simulationId, hour:1, interventions: [{intervention: "hospital"}]}).save()
      await new Count({simulation_id: simulationId, hour:2, interventions: [{intervention: "lockdown-start"}]}).save()
      await new Count({simulation_id: simulationId, hour:3}).save()
      await new Count({simulation_id: mockObjectId(), hour:1, interventions: [{intervention: "lockdown-start"}]}).save()
      const interventions = await CountService.fetchInterventionsForSimulation(simulationId)

      expect(interventions).toEqual([{hour:1, interventions: [{intervention: "hospital"}]},
      {hour:2, interventions: [{intervention: "lockdown-start"}]}])
    });
  });

  beforeAll(async () => await dbHandler.connect());
  afterEach(async () => await dbHandler.clearDatabase());
  afterAll(async () => await dbHandler.closeDatabase());

  const buildTestIntervention = () => {
    const interventionName = "test-intervention";
    const interventionData = {status: "done"};
    return {hour: 12, intervention: interventionName, data: interventionData};
  };
});