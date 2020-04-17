const dbHandler = require('../db-handler');
const SimulationService = require('../../../db/services/SimulationService');
const {Simulation, SimulationStatus}  = require('../../../db/models/Simulation');

describe('Simulation Service', function () {
  beforeAll(async () => await dbHandler.connect());
  afterEach(async () => await dbHandler.clearDatabase());
  afterAll(async () => await dbHandler.closeDatabase());

  describe('markSimulationEnd', function () {
    it('should set simulation status to ended', async function () {
      const testSimulationId = 1234;
      await new Simulation({simulation_id: testSimulationId, status: SimulationStatus.RUNNING}).save();

      await SimulationService.markSimulationEnd(testSimulationId);

      const s = await Simulation.findOne({simulation_id: testSimulationId}).exec();
      expect(s.toObject().status).toBe(SimulationStatus.FINISHED);
    });
  });
});