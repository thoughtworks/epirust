import React, { useState, useEffect } from 'react';
import Graph from './LineGraph';
import PropTypes from 'prop-types'

export default function SocketAwareGraph({ socket }) {
    const [dataBuffer, setDataBuffer] = useState([]);

    useEffect(() => {

        if (!socket) {
            setDataBuffer([])
            return
        }

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

    return <Graph dataBuffer={dataBuffer} />
}

SocketAwareGraph.propTypes = {
    socket: PropTypes.object
}