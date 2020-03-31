const mongoose = require("mongoose");
const Schema = mongoose.Schema;

const SimulationStatus = {
  RUNNING: "running",
  FAILED: "failed",
  FINISHED: "finished"
};
Object.freeze(SimulationStatus);

const simulationSchema = new Schema({
  simulation_id: {type: Number, required: true, unique: true},
  status: String,
  counts: [{
    hour: Number,
    susceptible: Number,
    infected: Number,
    quarantined: Number,
    recovered: Number,
    deceased: Number
  }],
});


const Simulation = mongoose.model('Simulation', simulationSchema);
module.exports = {Simulation, SimulationStatus};