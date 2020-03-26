/*
 * EpiRust
 * Copyright (c) 2020  ThoughtWorks, Inc.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

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
