import React from 'react';
import Dygraph from 'dygraphs';
import { useState, useEffect } from 'react';
import PropTypes from 'prop-types'
import { modelAnnotation } from './utils';

export default function Graph({ dataBuffer, enableExport = false, annotations = [], dygraphsOptions = {}, visibility }) {
    const [graph, setGraph] = useState(null);

    useEffect(() => {
        if (dataBuffer.length === 0) {
            graph && graph.destroy()
            setGraph();
            return
        }
        if (!graph) {
            let options = {
                legend: 'always',
                animatedZooms: true,
                title: 'Time Series Graph',
                ylabel: 'Number of Agents',
                xlabel: 'Hours',
                showRoller: true,
                errorBars: true,
                rollPeriod: 24,
                ...dygraphsOptions
            }
            const visualisationDiv = document.getElementById("vis");
            const graphInstance = new Dygraph(visualisationDiv, dataBuffer, options);
            setGraph(graphInstance)
        }
        else {
            graph.updateOptions({ 'file': dataBuffer });
        }
    }, [graph, dataBuffer, dygraphsOptions])

    useEffect(() => {
        if (!graph)
            return

        graph.setVisibility(visibility)

    }, [graph, visibility])


    useEffect(() => {
        if (!graph || !annotations.length)
            return

        graph.setAnnotations(annotations.map(modelAnnotation))

    }, [graph, annotations])

    function handleExportClick() {
        let filename = "export.json";
        let contentType = "application/json;charset=utf-8;";

        const a = document.createElement('a');
        a.download = filename;
        a.href = 'data:' + contentType + ',' + encodeURIComponent(JSON.stringify(dataBuffer));
        a.target = '_blank';
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
    }

    return (
        <div className="graph-wrap">

            <div className="graph-actions">
                <p className="action">Zoom: <code className='control'>click-drag</code></p>
                <p className="action">Pan: <code className='control'>shift-click-drag</code></p>
                <p className="action">Restore: <code className='control'>double-click</code></p>
                <button onClick={handleExportClick} className="btn btn-sm btn-secondary" disabled={!enableExport}>Export graph data</button>
            </div>

            <div id="vis" data-testid="visualization" />

        </div>
    );
}

Graph.propTypes = {
    enableExport: PropTypes.bool,
    annotations: PropTypes.arrayOf(PropTypes.object),
    dataBuffer: PropTypes.oneOfType([PropTypes.string, PropTypes.array])
}
