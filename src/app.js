import React from 'react';
import ParamterInputForm from './ParameterInputForm';
import { useState, useEffect } from 'react';
import io from 'socket.io-client'
import './app.scss';
import Graph from './LineGraph';

function App() {
  const [socket, setSocket] = useState(null);
  const [dataBuffer, setDataBuffer] = useState(null);

  useEffect(() => {

    if (!socket)
      return

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
    setDataBuffer([]);
  }

  function handleFormSubmit() {
    if (socket)
      socket.close()

    setDataBuffer(null);

    startSimulation()
      .then(startSocket)
  }

  return (
    <>
      <nav className="navbar navbar-expand-lg navbar-dark bg-dark">
        <a className="navbar-brand" href="/">EpiViz</a>
      </nav>
      <div className="container mt-4">
        <ParamterInputForm onSubmit={handleFormSubmit} />
        <Graph dataBuffer={dataBuffer} />
      </div>
    </>
  );
}

export default App;