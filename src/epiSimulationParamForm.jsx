import React, { useRef } from 'react';
import io from 'socket.io-client';
import Plotly from 'plotly.js-dist';

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
      socket.on('epidemicStats', function(message) {
        // var susceptible = {
        //   x: message.hours_data,
        //   y: message.susceptible_data,
        //   type: 'scatter',
        //   name: 'susceptible'
        // };
        // var infected = {
        //   x: message.hours_data,
        //   y: message.infected_data,
        //   type: 'scatter',
        //   name: 'infected'
        // };

        // var quarantined = {
        //   x: message.hours_data,
        //   y: message.quarantined_data,
        //   type: 'scatter',
        //   name: 'quarantined'
        // };

        // var recovered = {
        //   x: message.hours_data,
        //   y: message.recovered_data,
        //   type: 'scatter',
        //   name: 'recovered'
        // };

        // var deceased = {
        //   x: message.hours_data,
        //   y: message.deceased_data,
        //   type: 'scatter',
        //   name: 'deceased'
        // };

        // var patients = [susceptible, infected, quarantined, recovered, deceased];
        // var layout = {
        //   xaxis: {
        //   },

        //   legend: {
        //     x: 1,
        //     y: 3.5
        //   }
        // };
        // Plotly.newPlot('myDiv', patients, layout);
        console.log(message);
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