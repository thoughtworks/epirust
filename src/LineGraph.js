import React from 'react';
import Dygraph from 'dygraphs';
import { useState, useEffect } from 'react';
import './LineGraph.scss'

export default function Graph({ dataBuffer }) {
    const [graph, setGraph] = useState();

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

            setGraph(graphInstance);
        }
        else {
            graph.updateOptions({ 'file': dataBuffer });
        }

    }, [graph, dataBuffer])

    return (
        <>
            <p className="graph-actions">
                <span className="action">Zoom: <span className='control'>click-drag</span></span>
                <span className="action">Pan: <span className='control'>shift-click-drag</span></span>
                <span className="action">Restore: <span className='control'>double-click</span></span>
            </p>
            <div id="vis" data-testid="visualization" style={{ margin: "0 auto", width: "80%", height: "600px" }}></div>
        </>
    );
}