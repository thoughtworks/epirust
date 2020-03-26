import React from 'react';
import PropTypes from 'prop-types'
import DiseaseDynamics from './DiseaseDynamics'

export default function ParametersForm({ onDataSubmit }) {

    function handleSubmit(e) {
        e.preventDefault();
        let paramsData = {}
        new FormData(e.target).forEach(function (value, key) {
            if (["number_of_agents",
                "grid_size",
                "simulation_hrs",
                "public_transport_percentage",
                "working_percentage",
                "vaccinate_at",
                "vaccinate_percentage",
                "death_rate",
                "high_transmission_rate",
                "high_transmission_start_day",
                "last_day",
                "regular_transmission_rate",
                "regular_transmission_start_day",
                "lockdown_at_number_of_infections",
                "emergency_workers_population",
                "hospital_spread_rate_threshold"].includes(key)) {
                value = Number(value);
            }
            paramsData[key] = value;
        });

        onDataSubmit(paramsData)
    }

    function renderInterventions() {
        return (
            <fieldset>
                <legend>Interventions</legend>
                <div className="input-control">
                    <label htmlFor="vaccinate_at">Vaccinate At</label>
                    <input type="number" name="vaccinate_at" className="form-control" id="vaccinate_at" aria-describedby="vaccinate_at" placeholder="Vaccinate At" defaultValue="5000" />
                </div>
                <div className="input-control">
                    <label htmlFor="vaccinate_percentage">Vaccinate Percentage</label>
                    <input type="number" name="vaccinate_percentage" className="form-control" id="vaccinate_percentage" aria-describedby="vaccinate_percentage" placeholder="Vaccinate Percentage" defaultValue="0.2" step="any" />
                </div>
                <div className="input-control">
                    <label htmlFor="lockdown_at_number_of_infections">Vaccinate Percentage</label>
                    <input type="number" name="lockdown_at_number_of_infections" className="form-control" id="lockdown_at_number_of_infections" aria-describedby="lockdown_at_number_of_infections" placeholder="Lockdown At(number of infections)" defaultValue="100" step="any" />
                </div>
                <div className="input-control">
                    <label htmlFor="emergency_workers_population">Emergency Workers Population</label>
                    <input type="number" name="emergency_workers_population" className="form-control" id="emergency_workers_population" aria-describedby="emergency_workers_population" placeholder="Emergency Workers Population" defaultValue="0.1" step="any" />
                </div>
                <div className="input-control">
                    <label htmlFor="hospital_spread_rate_threshold">Hospital Spread Rate Threshold</label>
                    <input type="number" name="hospital_spread_rate_threshold" className="form-control" id="hospital_spread_rate_threshold" aria-describedby="hospital_spread_rate_threshold" placeholder="Hospital Spread Rate Threshold" defaultValue="100" step="any" />
                </div>
            </fieldset>)
    }

    function renderPopulation() {
        return (<fieldset>
            <legend>Population</legend>
            <div className="input-control">
                <label htmlFor="number_of_agents">Number of Agents</label>
                <input type="number" name="number_of_agents" className="form-control" id="number_of_agents" aria-describedby="number_of_agents" placeholder="Number of Agents" defaultValue="10000" />
            </div>

            <div className="input-control">
                <label htmlFor="public_transport_percentage">Public Transport Percentage</label>
                <input type="number" name="public_transport_percentage" className="form-control" id="public_transport_percentage" aria-describedby="public_transport_percentage" placeholder="Public Transport Percentage" defaultValue="0.2" step="any" />
            </div>

            <div className="input-control">
                <label htmlFor="working_percentage">Working Percentage</label>
                <input type="number" name="working_percentage" className="form-control" id="working_percentage" aria-describedby="working_percentage" placeholder="Working Percentage" min="0" max="1" defaultValue="0.7" step="any" />
            </div>
        </fieldset>)
    }

    function renderMiscellaneousConfig() {
        return (
            <fieldset>
                <div className="input-control">
                    <label htmlFor="grid_size">Grid Size</label>
                    <input type="number" name="grid_size" className="form-control" id="grid_size" aria-describedby="grid_size" placeholder="Grid Size" defaultValue="250" />
                </div>


                <div className="input-control">
                    <label htmlFor="simulation_hrs">Simulation Hours</label>
                    <input type="number" name="simulation_hrs" className="form-control" id="simulation_hrs" aria-describedby="simulation_hrs" placeholder="Simulation Hours" defaultValue="10000" />
                </div>
            </fieldset>
        )
    }

    return (
        <form className="user-inputs" onSubmit={handleSubmit} data-testid='simulationForm'>

            <div className="form-row">

                <div className="col inputs">
                    <div>
                    {renderPopulation()}
                    {renderMiscellaneousConfig()}
                    </div>
                    {renderInterventions()}
                    <DiseaseDynamics />
                </div>
                <div className="col actions">
                    <button type="submit" className="btn btn-primary" id="submitBtn">Start</button>
                </div>
            </div>

        </form>
    )
}

ParametersForm.propTypes = {
    onDataSubmit: PropTypes.func.isRequired
}