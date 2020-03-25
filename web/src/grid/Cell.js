import React from "react";
import PropTypes from 'prop-types'
import {Color} from "./Color";
import "./Cell.scss"

export default function Cell({cellId, color, size}) {
    return (
        <div className={`grid-cell-${cellId}`} style={{width: `${size}px`, height: `${size}px`}}>
            <div className="cell-body"
                 style={{background: `rgb(${color.r}, ${color.g}, ${color.b})`}}/>
        </div>
    );
}

Cell.propTypes = {
    cellId: PropTypes.number.isRequired,
    color: PropTypes.instanceOf(Color).isRequired,
    size: PropTypes.number.isRequired
};