import React from 'react';
import ParametersForm from './ParamtersForm';

export default function DataInputs() {

    function pushData(paramsData) {
        return fetch("http://localhost:3000/simulation/init", {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(paramsData)
        });
    }

    function handleSubmittedData(data) {
        pushData(data);
    }

    return (
        <div className="graph-input">
            <ParametersForm onDataSubmit={handleSubmittedData} />
        </div>
    )
}

DataInputs.propTypes = {
};