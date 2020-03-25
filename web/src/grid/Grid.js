import React from "react";
import PropTypes from 'prop-types'
import Cell from "./Cell";
import {Color} from "./Color";
import "./Grid.scss"

export default function Grid({size}) {
    let color = new Color(75, 34, 23);
    let cellSize = 10;

    function repeat(element, times) {
        let _range = [];
        for (let i = 0; i < times; i++) {
            _range.push(element(i));
        }
        return _range;
    }

    const cell = (cellId) => <Cell key={cellId} cellId={cellId} size={cellSize} color={color}/>;
    const row = (rowId) => (
        <div className={`grid-row-${rowId}`} key={rowId}>
            {repeat(cell, size).map(x => x)}
        </div>
    );

    return (
        <div className="grid">
            <div className="row">
                <div className="col-8">
                    {repeat(row, size).map(x => x)}
                </div>
                <div className="col-4">
                    Config
                </div>
            </div>
        </div>
    );
}

Grid.propTypes = {
    size: PropTypes.number.isRequired
};