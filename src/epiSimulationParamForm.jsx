/* global Plotly */

import React, { useRef } from 'react';
import io from 'socket.io-client';

export default function() {
  const graphHolder = useRef();
  const startSimulation = function(e) {
    e.preventDefault();
    const paramsData = {}
    new FormData(e.target).forEach(function(value, key){
      paramsData[key] = value;
    });
    fetch("http://localhost:3000/simulation", {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(paramsData)
    }).then((res) => {
      let socket = null;
      if(!socket) {
        socket = io('http://localhost:3000/');
      }
      res.json().then((json) => console.log(json));
      let hours = [], susceptible = [], infected = [], quarantined = [], recovered = [], deceased = [];
      socket.on('epidemicStats', function(message) {
        var patients;  
        message = JSON.parse(message);
        hours.push(message.hour);
        susceptible.push(message.susceptible);
        infected.push(message.infected);
        quarantined.push(message.quarantined);
        recovered.push(message.recovered);
        deceased.push(message.deceased);
        var susceptibleData = {
          x: hours,
          y: susceptible,
          type: 'scatter',
          name: 'susceptible'
        };
        // var infectedData = {
        //   x: hours,
        //   y: infected,
        //   type: 'scatter',
        //   name: 'infected'
        // };
        // var quarantinedData = {
        //   x: hours,
        //   y: quarantined,
        //   type: 'scatter',
        //   name: 'quarantined'
        // };
        // var recoveredData = {
        //   x: hours,
        //   y: recovered,
        //   type: 'scatter',
        //   name: 'recovered'
        // };

        // var deceasedData = {
        //   x: hours,
        //   y: deceased,
        //   type: 'scatter',
        //   name: 'deceased'
        // };
        patients = [susceptibleData];
        var layout = {
          xaxis: {
          },
          legend: {
            x: 1,
            y: 3.5
          }
        };
        if(hours.length === 1){
          Plotly.newPlot('myDiv', patients, layout);
        } else {
          Plotly.restyle('myDiv', 'y', [susceptible]);
          Plotly.restyle('myDiv', 'x', [hours]);
        }
        console.log(patients);
      });
    })
  }
    return (
      <>
        <div ref={graphHolder} id="myDiv"></div>
        <form onSubmit={startSimulation}>
          <div className="form-row">
            <div className="col">
              <input type="number" name="numberOfAgents" className="form-control" id="numberOfAgents" aria-describedby="numberOfAgents" placeholder="Number of Agents" defaultValue="10000"/>
            </div>
            <div className="col">
              <button type="submit" className="btn btn-primary">Submit</button>
            </div>
          </div>
          {/* <div className="form-group">
            <label htmlFor="simulationLifeTime">Simulation Life Time</label>
            <input type="number" name="simulationLifeTime" className="form-control" id="simulationLifeTime" />
          </div>
          <div className="form-group">
            <label htmlFor="vaccinationTime">Vaccination Time</label>
            <input type="number" name="vaccinationTime" className="form-control" id="vaccinationTime" />
          </div>
          <div className="form-group">
            <label htmlFor="vaccinationPercentage">Vaccination Percentage</label>
            <input type="number" name="vaccinationPercentage" className="form-control" id="vaccinationPercentage" step="0.1"/>
          </div>
          <div className="form-group">
            <label htmlFor="publicTransportPercentage">Public Transport Percentage</label>
            <input type="number" name="publicTransportPercentage" className="form-control" id="publicTransportPercentage" step="0.1"/>
          </div>
          <div className="form-group">
            <label htmlFor="workingPercentage">Working Percentage</label>
            <input type="number" name="workingPercentage" className="form-control" id="workingPercentage" step="0.1"/>
          </div> */}
        </form>
      </>
    )
}