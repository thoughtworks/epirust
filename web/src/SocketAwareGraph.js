/*
 * EpiRust
 * Copyright (c) 2020  ThoughtWorks, Inc.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

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
            const message = JSON.parse(messageRaw);

            let simulationEndedTemp = false
            if ("simulation_ended" in message) {
                simulationEndedTemp = true;
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
