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
            < input
                type="file"
                id="import-input"
                data-testid="import-input"
                name="import"
                onChange={handleFileUpload}
                accept="application/json" >
            </input >
            <br />
            <small className="form-text text-muted">You can export a data file when a graph is plotted, for importing later.</small>
        </form>
    )
}

FileDataInput.propTypes = {
    onFileDataSubmit: PropTypes.func.isRequired
}