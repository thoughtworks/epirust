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
import PropTypes from 'prop-types'

export default function FileDataInput({ onFileDataSubmit }) {

    function handleFileUpload(e) {
        const file = e.target.files[0];

        var fileStart = 0;
        var fileEnd = file.size;

        var reader = new FileReader();

        // If we use onloadend, we need to check the readyState.
        reader.onloadend = function (evt) {
            if (evt.target.readyState === FileReader.DONE) {
                let fileData = JSON.parse(evt.target.result)
                return onFileDataSubmit(fileData);
            }
        };

        reader.readAsBinaryString(file.slice(fileStart, fileEnd));
    }

    return (
        <form className="file-input">
            <label htmlFor="import-input">Choose a file to import data from:</label>
            < input type="file"
                id="import-input"
                data-testid="import-input"
                name="import"
                onChange={handleFileUpload}
                accept="application/json" >
            </input >
            <br />
            <small>You can export a data file when a graph is plotted, for importing later.</small>
        </form>
    )
}

FileDataInput.propTypes = {
    onFileDataSubmit: PropTypes.func.isRequired
}
