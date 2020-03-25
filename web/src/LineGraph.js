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
import Dygraph from 'dygraphs';
import { useState, useEffect } from 'react';
import './LineGraph.scss'
import PropTypes from 'prop-types'

export default function Graph({ dataBuffer , enableExport = false}) {
    const [graph, setGraph] = useState(null);
    useEffect(() => {
        if (dataBuffer.length === 0) {
            graph && graph.destroy()
            setGraph();
            return
        }
        if (!graph) {
            let graphInstance = new Dygraph(document.getElementById("vis"), dataBuffer, {
                labels: ["hour", "susceptible", "infected", "quarantined", "recovered", "deceased"],
                legend: 'always',
                animatedZooms: true,
                title: 'Time Series Graph',
                ylabel: 'Number of Agents',
                xlabel: 'Hours',
                showRoller: true
            });

            setGraph(graphInstance)
        }
        else {
            graph.updateOptions({ 'file': dataBuffer });
        }
    }, [graph, dataBuffer])

    function handleExportClick() {
        let filename = "export.json";
        let contentType = "application/json;charset=utf-8;";

        var a = document.createElement('a');
        a.download = filename;
        a.href = 'data:' + contentType + ',' + encodeURIComponent(JSON.stringify(dataBuffer));
        a.target = '_blank';
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
    }

    return (
        <div className="graph-wrap">

            <div id="vis" data-testid="visualization"></div>

            <div className="graph-actions">
                <p className="action">Zoom: <span className='control'>click-drag</span></p>
                <p className="action">Pan: <span className='control'>shift-click-drag</span></p>
                <p className="action">Restore: <span className='control'>double-click</span></p>
                <button onClick={handleExportClick} className="btn btn-secondary" disabled={!enableExport}>Export graph</button>
            </div>

        </div>
    );
}

Graph.propTypes = {
    dataBuffer: PropTypes.array.isRequired,
    enableExport: PropTypes.bool
}

