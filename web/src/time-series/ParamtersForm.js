import React from 'react';
import PropTypes from 'prop-types'
import DiseaseDynamics from './DiseaseDynamics'
import Interventions from './Interventions'

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
                "essential_workers_population",
                "hospital_spread_rate_threshold"].includes(key)) {
                value = Number(value);
            }
            paramsData[key] = value;
        });

        onDataSubmit(paramsData)
    }

    function renderPopulation() {
        return (
            <fieldset>
                <legend>Population</legend>
                <div className="input-control">
                    <label className="col-form-label-sm" htmlFor="number_of_agents">Number of Agents</label>
                    <input type="number" name="number_of_agents" className="form-control form-control-sm" id="number_of_agents" aria-describedby="number_of_agents" placeholder="Number of Agents" defaultValue="10000" />
                </div>

                <div className="input-control">
                    <label className="col-form-label-sm" htmlFor="public_transport_percentage">Public Transport Percentage</label>
                    <input type="number" name="public_transport_percentage" className="form-control form-control-sm" id="public_transport_percentage" aria-describedby="public_transport_percentage" placeholder="Public Transport Percentage" defaultValue="0.2" step="any" />
                </div>

                <div className="input-control">
                    <label className="col-form-label-sm" htmlFor="working_percentage">Working Percentage</label>
                    <input type="number" name="working_percentage" className="form-control form-control-sm" id="working_percentage" aria-describedby="working_percentage" placeholder="Working Percentage" min="0" max="1" defaultValue="0.7" step="any" />
                </div>
            </fieldset>
        )
    }

    function renderMiscellaneousConfig() {
        return (
            <fieldset>
                <div className="input-control">
                    <label className="col-form-label-sm" htmlFor="grid_size">Grid Size</label>
                    <input type="number" name="grid_size" className="form-control form-control-sm" id="grid_size" aria-describedby="grid_size" placeholder="Grid Size" defaultValue="250" />
                </div>


                <div className="input-control">
                    <label className="col-form-label-sm" htmlFor="simulation_hrs">Simulation Hours</label>
                    <input type="number" name="simulation_hrs" className="form-control form-control-sm" id="simulation_hrs" aria-describedby="simulation_hrs" placeholder="Simulation Hours" defaultValue="10000" />
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
                    <DiseaseDynamics />
                    <Interventions />
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