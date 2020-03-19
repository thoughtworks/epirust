import React from 'react';
import FileDataInput from './FileDataInput';
import ParametersForm from './ParamtersForm';
import './DataInputs.scss'

export default function DataInputs(props) {

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
            props.onSubmit()
        );
    }

    function handleFileData(data) {
        console.log(data)
    }

    return (
        <div className="graph-input">
            <ParametersForm onDataSubmit={handleSubmittedData} />
            <FileDataInput onFileDataSubmit={handleFileData} />
        </div>
    )
}