import React from 'react';
import DataInputs from './DataInputs';
import { useState } from 'react';
import io from 'socket.io-client'
import SocketAwareGraph from './SocketAwareGraph'
import Graph from './LineGraph';

function TimeSeries() {
    const [socket, setSocket] = useState(null);
    const [fileData, setFileData] = useState([]);

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

    function handleFormSubmit() {
        if (socket) {
            socket.close()
            setSocket(null)
        }

        if (fileData.length > 1) {
            setFileData([])
        }

        startSimulation()
            .then(startSocket)
    }

    function handleFileData(data) {
        if (socket) {
            socket.close()
            setSocket(null);
        }
        setFileData(data)
    }

    return (
        <>
            <DataInputs onFileDataInput={handleFileData} onSubmit={handleFormSubmit} />
            {fileData.length > 1 ? <Graph dataBuffer={fileData} /> : <SocketAwareGraph socket={socket} />}
        </>
    );
}

export default TimeSeries;