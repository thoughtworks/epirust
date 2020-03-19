import React from 'react';
import DataInputs from './DataInputs';
import { useState } from 'react';
import io from 'socket.io-client'
import './app.scss';
import SocketAwareGraph from './SocketAwareGraph'

function App() {
  const [socket, setSocket] = useState(null);

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

    startSimulation()
      .then(startSocket)
  }

  return (
    <>
      <nav className="navbar navbar-expand-lg navbar-dark bg-dark">
        <a className="navbar-brand" href="/">EpiViz</a>
      </nav>
      <div className="container mt-4">
        <DataInputs onSubmit={handleFormSubmit} />
        <SocketAwareGraph socket={socket} />
      </div>
    </>
  );
}

export default App;