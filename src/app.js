import React from 'react';
import ParamterInputForm from './ParameterInputForm';
import { useState } from 'react';
import './app.scss';

import { useEffect } from 'react';
import Dygraph from 'dygraphs';

import io from 'socket.io-client'

function App() {
  const [socket, setSocket] = useState(null);

  const [dataBuffer, setDataBuffer] = useState([]);
  const [graph, setGraph] = useState(null);

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

  useEffect(() => {

    if (dataBuffer.length === 0)
      return

    if (!graph) {
      let graphInstance = new Dygraph(document.getElementById("vis"), dataBuffer, {
        labels: ["hour", "susceptible", "infected", "quarantined", "recovered", "deceased"]
      });

      setGraph(graphInstance);
    }
    else {
      graph.updateOptions({ 'file': dataBuffer });
    }

  }, [graph, dataBuffer])

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
    if (socket)
      socket.close()

    if (graph) {
      setDataBuffer([]);
      graph.destroy()
      setGraph(null);
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
        <ParamterInputForm onSubmit={handleFormSubmit} />
        <div id="vis" style={{ width: "70%", height: "600px" }}></div>
      </div>
    </>
  );
}

export default App;