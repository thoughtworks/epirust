/* global Plotly */

import React from 'react';
import io from 'socket.io-client';
import { useState } from 'react';
import { useRef } from 'react';
import Chart from 'chart.js';

const LineChartConfig = {
  type: 'scatter',
  data: {
    datasets: [{
      label: 'susceptible',
      data: [],
      borderColor: 'black',
    }, {
      label: 'infected',
      data: [],
      borderColor: 'red',
    }, {
      label: 'quarantined',
      data: [],
      borderColor: 'green',
    }, {
      label: 'recovered',
      data: [],
      borderColor: 'blue',
    }, {
      label: 'deceased',
      data: [],
      borderColor: 'yellow',
    }],
    options: {
      responsive: true,
      scales: {
        xAxes: [{
          display: 'true'
        }]
      }
    }
  }
}
LineChartConfig.data.datasets.forEach(function (dataset) {
  dataset.borderWidth = 1;
  dataset.showLine = true;
  dataset.lineTension = 0;
  dataset.fill = false;
  dataset.pointRadius = 1;
});

let iteration = 0;
let susceptible = [], infected = [], quarantined = [], recovered = [], deceased = [];
export default function () {
  const [socket, setSocket] = useState(null);
  const [chart, setChart] = useState(null);
  const chartCanvas = useRef();
  const stopSimulation = function (e) {
    e.preventDefault();
    socket.close();
  }

  const startSimulation = function (e) {
    e.preventDefault();
    const paramsData = {}
    new FormData(e.target).forEach(function (value, key) {
      paramsData[key] = value;
    });

    fetch("http://localhost:3000/simulation", {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(paramsData)
    }).then((res) => {
      chartCanvas.current.width = window.innerWidth;
      chartCanvas.current.height = window.innerHeight;
      setChart(new Chart(chartCanvas.current, LineChartConfig));
      setSocket(io('http://localhost:3000/'));
    })

  }

  const updateChart = (message) => {
    console.log(iteration++);
    message = JSON.parse(message);
    LineChartConfig.data.datasets[0].data.push({
      x: message.hour,
      y: message.susceptible
    });
    LineChartConfig.data.datasets[1].data.push({
      x: message.hour,
      y: message.infected
    });
    LineChartConfig.data.datasets[2].data.push({
      x: message.hour,
      y: message.quarantined
    });
    LineChartConfig.data.datasets[3].data.push({
      x: message.hour,
      y: message.recovered
    });
    LineChartConfig.data.datasets[4].data.push({
      x: message.hour,
      y: message.deceased
    });
    /* batching updates for performance */
    if (iteration % 1000 === 0) {
      chart.update();
    }
  }

  socket && socket.on('epidemicStats', function (message) {
    updateChart(message);
  });

  return (
    <>
      <canvas ref={chartCanvas} id="myChart" width="100vw" height="100vh"></canvas>
      {/* <div id="myDiv"></div> */}
      <form onSubmit={startSimulation}>
        <div className="form-row">
          <div className="col">
            <input type="number" name="numberOfAgents" className="form-control" id="numberOfAgents" aria-describedby="numberOfAgents" placeholder="Number of Agents" defaultValue="10000" />
          </div>
          <div className="col">
            <button type="submit" className="btn btn-primary">Start</button>
            <button type="button" className="btn btn-danger" onClick={stopSimulation}>Stop</button>
          </div>
        </div>
      </form>
    </>
  )
}