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

import React from 'react';
import DataInputs from './DataInputs';
import { useState } from 'react';
import io from 'socket.io-client'
import './app.scss';
import SocketAwareGraph from './SocketAwareGraph'
import Graph from './LineGraph';

function App() {
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
      <nav className="navbar navbar-expand-lg navbar-dark bg-dark">
        <a className="navbar-brand" href="/">EpiViz</a>
      </nav>
      <div className="container-fluid mt-4">
        <DataInputs onFileDataInput={handleFileData} onSubmit={handleFormSubmit} />
        {fileData.length > 1 ? <Graph dataBuffer={fileData} /> : <SocketAwareGraph socket={socket} />}
      </div>
    </>
  );
}

export default App;
