import React from 'react';
import Dygraph from 'dygraphs';
import { useState, useEffect } from 'react';
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

