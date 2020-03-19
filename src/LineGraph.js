import React from 'react';
import Dygraph from 'dygraphs';
import { useState, useEffect } from 'react';

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
                labels: ["hour", "susceptible", "infected", "quarantined", "recovered", "deceased"]
            });

            setGraph(graphInstance);
        }
        else {
            graph.updateOptions({ 'file': dataBuffer });
        }

    }, [graph, dataBuffer])

    return <div id="vis" data-testid="visualization" style={{ width: "70%", height: "600px" }}></div>;
}