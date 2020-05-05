import PropTypes from 'prop-types';
import React, { useEffect, useState } from 'react';
import Graph from './LineGraph';
import Loader from '../common/Loader'
import { parseAnnotations } from './utils';
import io from "socket.io-client";
import config from "../config";

export const BUFFER_SIZE_TO_RENDER = 100;
export default function SocketAwareGraph({ jobId }) {
    const [socket, setSocket] = useState(null);
    const [dataBuffer, setDataBuffer] = useState([]);
    const [simulationEnded, setSimulationEnded] = useState(false);
    const [annotations, updateAnnotations] = useState([]);

    useEffect(() => {
        const socketInstance = io(`${config.API_HOST}/${'counts'}`);
        setSocket(socketInstance);

        return () => {
            socketInstance.close();
        }
    }, []);

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
                    hospitalized, hospitalized_std,
                    recovered, recovered_std,
                    deceased, deceased_std } = message;

                const perHourStats = [hour,
                    susceptible, susceptible_std,
                    infected, infected_std,
                    hospitalized, hospitalized_std,
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
                setDataBuffer(buffer => {                
                    const temp = [...buffer, ...buff]
                    buff = [];
                    return temp;
                });
            }
            if (simulationEndedTemp) {
                setSimulationEnded(true)
            }
        });

        return () => socket.close()
    }, [socket, jobId]);

    if (!dataBuffer.length)
        return <Loader />

    return (
        <Graph
            annotations={annotations}
            dataBuffer={convertToCsv(dataBuffer)}
            enableExport={simulationEnded}
            errorBars={true} />
    )
}

SocketAwareGraph.propTypes = {
    socket: PropTypes.object,
    jobId: PropTypes.string.isRequired,
};

export function convertToCsv(dataBuffer) {
    const labels = ["hour", "susceptible", "infected", "hospitalized", "recovered", "deceased"];
    return [labels, ...dataBuffer].join("\n");
}