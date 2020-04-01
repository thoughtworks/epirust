import React, { useState, useEffect } from 'react';
import Graph from './LineGraph';
import PropTypes from 'prop-types'

export default function SocketAwareGraph({ socket }) {
    const [dataBuffer, setDataBuffer] = useState([]);
    const [simulationEnded, setSimulationEnded] = useState(false);
    useEffect(() => {

        if (!socket) {
            setDataBuffer([])
            return
        }

        let buff = [];

        socket.on('epidemicStats', function (messageRaw) {
            const message = messageRaw;

            let simulationEndedTemp = false
            if ("simulation_ended" in message) {
                simulationEndedTemp = true;
                socket.close();
            }
            else {
                const { hour, susceptible, infected, quarantined, recovered, deceased } = message;
                buff.push([hour, susceptible, infected, quarantined, recovered, deceased]);
            }
            if (message.hour % 100 === 0 || simulationEndedTemp) {
                setDataBuffer(buffer => {
                    let total = [...buffer, ...buff]
                    buff = [];
                    return total;
                });
            }
            if(simulationEndedTemp) {
                setSimulationEnded(true)
            }
        });
    }, [socket])

    return <Graph dataBuffer={dataBuffer} enableExport={simulationEnded}/>
}

SocketAwareGraph.propTypes = {
    socket: PropTypes.object
}