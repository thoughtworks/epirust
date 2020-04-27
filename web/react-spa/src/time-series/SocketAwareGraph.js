import PropTypes from 'prop-types';
import React, { useEffect, useState } from 'react';
import Graph from './LineGraph';
import Loader from '../common/Loader'
import { parseAnnotations } from './utils';

export const BUFFER_SIZE_TO_RENDER = 100;
export default function SocketAwareGraph({ socket, jobId }) {
    const [dataBuffer, setDataBuffer] = useState([]);
    const [simulationEnded, setSimulationEnded] = useState(false);
    const [annotations, updateAnnotations] = useState([]);

    useEffect(() => {
        if (!socket) {
            setDataBuffer([]);
            return
        }

        let buff = [];
        socket.emit('get', { jobId });

        socket.on('epidemicStats', function (messageRaw) {
            const message = messageRaw;

            let simulationEndedTemp = false;
            if ("simulation_ended" in message) {
                simulationEndedTemp = true;
                socket.close();
            }
            else {
                const { hour,
                    susceptible, susceptible_std,
                    infected, infected_std,
                    quarantined, quarantined_std,
                    recovered, recovered_std,
                    deceased, deceased_std } = message;

                const perHourStats = [hour,
                    susceptible, susceptible_std,
                    infected, infected_std,
                    quarantined, quarantined_std,
                    recovered, recovered_std,
                    deceased, deceased_std
                ];

                buff.push(perHourStats);

                if ('interventions' in message) {
                    updateAnnotations(annotations =>
                        [...annotations, ...parseAnnotations(message.interventions, message.hour)]
                    )
                }
            }

            if (buff.length === BUFFER_SIZE_TO_RENDER || simulationEndedTemp) {
                setDataBuffer(buffer => [...buffer, ...buff]);
                buff = [];
            }
            if (simulationEndedTemp) {
                setSimulationEnded(true)
            }
        });

        return () => socket.close()
    }, [socket, jobId]);

    if (!dataBuffer.length)
        return <Loader />

    const arr = [...dataBuffer]
    arr.unshift(["hour", "susceptible", "infected", "quarantined", "recovered", "deceased"]);
    const csvFormattedData = arr.join("\n");
    return (
        <Graph
            annotations={annotations}
            dataBuffer={csvFormattedData}
            enableExport={simulationEnded}
            errorBars={true} />
    )
}

SocketAwareGraph.propTypes = {
    socket: PropTypes.object,
    jobId: PropTypes.string.isRequired,
};
