import PropTypes from 'prop-types';
import React, { useEffect, useState } from 'react';
import Graph from './LineGraph';
import Loader from '../common/Loader'
import { parseAnnotations } from './utils';

export const BUFFER_SIZE_TO_RENDER = 100;
export default function SocketAwareGraph({ socket, jobId, includesMultipleSimulations = false }) {
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

                //TODO: write this better
                let perHourStats = []
                if (includesMultipleSimulations) {
                    const { hour,
                        susceptible, susceptible_std,
                        infected, infected_std,
                        quarantined, quarantined_std,
                        recovered, recovered_std,
                        deceased, deceased_std } = message;

                    perHourStats = [hour,
                        susceptible, susceptible_std,
                        infected, infected_std,
                        quarantined, quarantined_std,
                        recovered, recovered_std,
                        deceased, deceased_std
                    ];
                }
                else {
                    const { hour, susceptible, infected, quarantined, recovered, deceased } = message;
                    perHourStats = [hour, susceptible, infected, quarantined, recovered, deceased];
                }
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

    if (includesMultipleSimulations) {
        const arr = [...dataBuffer]
        arr.unshift(["hour", "susceptible", "infected", "quarantined", "recovered", "deceased"]);
        const csvFormattedData = arr.join("\n");
        return <Graph dataBuffer={csvFormattedData} enableExport={true} errorBars={true} />
    }

    return (
        <Graph
            annotations={annotations}
            dataBuffer={dataBuffer}
            enableExport={simulationEnded}
            labels={["hour", "susceptible", "infected", "quarantined", "recovered", "deceased"]}
        />
    )
}

SocketAwareGraph.propTypes = {
    socket: PropTypes.object,
    jobId: PropTypes.string.isRequired,
};
