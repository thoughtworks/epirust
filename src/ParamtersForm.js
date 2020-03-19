import React from 'react';

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
                "vaccinate_percentage"].includes(key)) {
                value = Number(value);
            }
            paramsData[key] = value;
        });

        onDataSubmit(paramsData)
    }

    return (
        <form onSubmit={handleSubmit} data-testid='simulationForm'>

            <div className="form-row">
                <div className="col inputs">
                    <div className="input-control">
                        <label htmlFor="number_of_agents">Number of Agents</label>
                        <input type="number" name="number_of_agents" className="form-control" id="number_of_agents" aria-describedby="number_of_agents" placeholder="Number of Agents" defaultValue="10000" />
                    </div>

                    <div className="input-control">
                        <label htmlFor="disease_name">Disease Name</label>
                        <input type="text" name="disease_name" className="form-control" id="disease_name" aria-describedby="disease_name" placeholder="Disease Name" defaultValue="small_pox" />
                    </div>

                    <div className="input-control">
                        <label htmlFor="grid_size">Grid Size</label>
                        <input type="number" name="grid_size" className="form-control" id="grid_size" aria-describedby="grid_size" placeholder="Grid Size" defaultValue="250" />
                    </div>

                    <div className="input-control">
                        <label htmlFor="simulation_hrs">Simulation Hours</label>
                        <input type="number" name="simulation_hrs" className="form-control" id="simulation_hrs" aria-describedby="simulation_hrs" placeholder="Simulation Hours" defaultValue="10000" />
                    </div>

                    <div className="input-control">
                        <label htmlFor="public_transport_percentage">Public Transport Percentage</label>
                        <input type="number" name="public_transport_percentage" className="form-control" id="public_transport_percentage" aria-describedby="public_transport_percentage" placeholder="Public Transport Percentage" defaultValue="0.2" step="any" />
                    </div>

                    <div className="input-control">
                        <label htmlFor="working_percentage">Working Percentage</label>
                        <input type="number" name="working_percentage" className="form-control" id="working_percentage" aria-describedby="working_percentage" placeholder="Working Percentage" min="0" max="1" defaultValue="0.7" step="any" />
                    </div>

                    <div className="input-control">
                        <label htmlFor="vaccinate_at">Vaccinate At</label>
                        <input type="number" name="vaccinate_at" className="form-control" id="vaccinate_at" aria-describedby="vaccinate_at" placeholder="Vaccinate At" defaultValue="5000" />
                    </div>

                    <div className="input-control">
                        <label htmlFor="vaccinate_percentage">Vaccinate Percentage</label>
                        <input type="number" name="vaccinate_percentage" className="form-control" id="vaccinate_percentage" aria-describedby="vaccinate_percentage" placeholder="Vaccinate Percentage" defaultValue="0.2" step="any" />
                    </div>
                </div>
                <div className="col">
                    <button type="submit" className="btn btn-primary" id="submitBtn">Start</button>
                </div>
            </div>

        </form>
    )
}
