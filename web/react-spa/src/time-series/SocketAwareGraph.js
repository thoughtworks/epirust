import React, { useState, useEffect } from 'react';
import Graph from './LineGraph';
import PropTypes from 'prop-types'

export default function SocketAwareGraph({ socket, simulationId }) {
    const [dataBuffer, setDataBuffer] = useState([]);
    const [simulationEnded, setSimulationEnded] = useState(false);
    useEffect(() => {
        if (!socket) {
            setDataBuffer([]);
            return
        }

        let buff = [];
        socket.emit('simulation_id', simulationId);

        socket.on('epidemicStats', function (messageRaw) {
            const message = messageRaw;

            let simulationEndedTemp = false;
            if ("simulation_ended" in message) {
                simulationEndedTemp = true;
                socket.close();
            }
            else {
                const { hour, susceptible, infected, quarantined, recovered, deceased } = message;
                const perHourStats = [hour, susceptible, infected, quarantined, recovered, deceased];
                buff.push(perHourStats);
            }
            if (message.hour % 100 === 0 || simulationEndedTemp) {
                setDataBuffer(buffer => {
                    let total = [...buffer, ...buff];
                    buff = [];
                    return total;
                });
            }
            if (simulationEndedTemp) {
                setSimulationEnded(true)
            }
        });

        return () => socket.close()
    }, [socket, simulationId]);

    return (
        <Graph
            dataBuffer={dataBuffer}
            enableExport={simulationEnded}
            labels={["hour", "susceptible", "infected", "quarantined", "recovered", "deceased"]}
        />
    )
}

SocketAwareGraph.propTypes = {
    socket: PropTypes.object,
    simulationId: PropTypes.number.isRequired,
};
