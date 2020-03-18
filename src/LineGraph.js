import React from 'react';
import Dygraph from 'dygraphs';
import { useState, useEffect } from 'react';

export default function Graph({ dataBuffer }) {
    const [graph, setGraph] = useState(null);

    useEffect(() => {
        if (dataBuffer.length === 0) {
            graph && graph.destroy()
            setGraph(null);
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

    return <div id="vis" style={{ width: "70%", height: "600px" }}></div>;
}