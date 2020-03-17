import React from 'react';
import { useEffect } from 'react';
import { useState } from 'react';
import io from 'socket.io-client'
import Dygraph from 'dygraphs';

export default function SocketAwareGraph() {

    const [socket, setSocket] = useState(null);
    const [dataBuffer, setDataBuffer] = useState([]);

    useEffect(() => {
        function startSimulation() {
            return fetch("http://localhost:3000/simulation", {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                }
            });
        }

        function startSocket() {
            setSocket(io('http://localhost:3000/'));
        }

        startSimulation()
            .then(startSocket)

    }, []);

    useEffect(() => {
        if (!socket)
            return;

        let buff = [];

        socket.on('epidemicStats', function (messageRaw) {
            const message = JSON.parse(messageRaw);
            const { hour, susceptible, infected, quarantined, recovered, deceased } = message;

            buff.push([hour, susceptible, infected, quarantined, recovered, deceased]);

            if (hour % 100 === 0) {
                setDataBuffer(buffer => {
                    let total = [...buffer, ...buff]
                    buff = [];
                    return total;
                });
            }
        });
    }, [socket])


    return (
        <LineGraph data={dataBuffer} />
    )
}

function LineGraph({ data }) {

    const [graph, setGraph] = useState(null);

    useEffect(() => {

        if (!graph) {
            let graphInstance = new Dygraph(document.getElementById("vis"), data, {
                labels: ["hour", "susceptible", "infected", "quarantined", "recovered", "deceased"]
            });

            setGraph(graphInstance);
        }
        else {
            graph.updateOptions({ 'file': data });
        }

    }, [graph, data])

    return (
        <div id="vis" style={{ width: "70%", height: "600px" }}></div>
    )
}
