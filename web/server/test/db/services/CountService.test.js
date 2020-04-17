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