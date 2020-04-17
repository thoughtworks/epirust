const {Count} = require('../models/Count');

const addIntervention = async (simulationId, intervention) => {
  const query = Count.updateOne(
    {simulation_id: simulationId, hour: intervention.hour},
    {$addToSet: {interventions: intervention}},
    {upsert: true}
  );

  await query.exec()
};

module.exports = {addIntervention};