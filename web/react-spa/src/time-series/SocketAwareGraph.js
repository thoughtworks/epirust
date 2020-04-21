import PropTypes from 'prop-types';
import React, { useEffect, useState } from 'react';
import { Interventions } from '../grid/constants';
import Graph from './LineGraph';
import Loader from '../common/Loader'

export default function SocketAwareGraph({ socket, simulationId }) {
    const [dataBuffer, setDataBuffer] = useState([]);
    const [simulationEnded, setSimulationEnded] = useState(false);
    const [annotations, updateAnnotations] = useState([]);

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

                if ('interventions' in message) {
                    updateAnnotations(annotations =>
                        [...annotations, ...parseAnnotations(message.interventions, hour)]
                    )
                }
            }
            if (buff.length === 100 || simulationEndedTemp) {
                setDataBuffer(buffer => [...buffer, ...buff]);
                buff = [];
            }
            if (simulationEndedTemp) {
                setSimulationEnded(true)
            }
        });

        return () => socket.close()
    }, [socket, simulationId]);

    if (!dataBuffer.length)
        return <Loader />

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
    simulationId: PropTypes.number.isRequired,
};

function parseAnnotations(interventions, hour) {

    const InterventionToClassNames = {
        [Interventions.LOCKDOWN]: "lockdown",
        [Interventions.BUILD_NEW_HOSPITAL]: "hospital",
        [Interventions.VACCINATION]: "vaccination"
    }

    function getLabel(interventionObj) {
        switch (interventionObj.intervention) {

            case Interventions.LOCKDOWN:
                return interventionObj.data.status === Interventions.status.LOCKDOWN_START
                    ? "Lockdown start"
                    : "Lockdown end"

            case Interventions.BUILD_NEW_HOSPITAL:
                return "Build Hospitals"

            case Interventions.VACCINATION:
                return "Vaccination"

            default:
                return "Unknown"
        }
    }

    return interventions.map(i => {
        const className = InterventionToClassNames[i.intervention];
        return { x: hour, label: getLabel(i), className }
    })
}

