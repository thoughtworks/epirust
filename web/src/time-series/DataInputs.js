import React from 'react';
import FileDataInput from './FileDataInput';
import ParametersForm from './ParamtersForm';
import PropTypes from 'prop-types'

export default function DataInputs({ onSubmit, onFileDataInput }) {

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
        pushData(data).then(
            onSubmit()
        );
    }

    function handleFileData(data) {
        onFileDataInput(data)
    }

    return (
        <div className="graph-input">
            <ParametersForm onDataSubmit={handleSubmittedData} />
            <FileDataInput onFileDataSubmit={handleFileData} />
        </div>
    )
}

DataInputs.propTypes = {
    onSubmit: PropTypes.func.isRequired,
    onFileDataInput: PropTypes.func.isRequired
}