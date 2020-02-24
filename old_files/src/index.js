import Plotly from 'plotly.js-dist';
import Data from './simulation_100_000.csv';

const button = document.getElementById("stop");

let new_data = [];

const stream = new ReadableStream({
  start(controller) {
    let interval = setInterval(() => {
      let data = Data.shift() 

        // Add the string to the stream
        controller.enqueue(data);

        // show it on the screen
        new_data.push(data)

        let hours_data = new_data.map(function(row){
          return row[0]
        });
        hours_data = hours_data.splice(1,hours_data.length);

        let susceptible_data = new_data.map(function(row){
          return row[1]
        });
        susceptible_data = susceptible_data.splice(1,susceptible_data.length);

        let infected_data = new_data.map(function(row){
          return row[2]
        });
        infected_data = infected_data.splice(1,infected_data.length);

        let quarantined_data = new_data.map(function(row){
          return row[3]
        });
        quarantined_data = quarantined_data.splice(1, quarantined_data.length);


        let recovered_data = new_data.map(function(row){
          return row[4]
        });
        recovered_data = recovered_data.splice(1, recovered_data.length);


        let deceased_data = new_data.map(function(row){
          return row[5]
        });
        deceased_data = deceased_data.splice(1, deceased_data.length);


        var susceptible = {
          x: hours_data,
          y: susceptible_data,
          type: 'scatter',
          name: 'susceptible'
        };
        var infected = {
          x: hours_data,
          y: infected_data,
          type: 'scatter',
          name: 'infected'
        };

        var quarantined = {
          x: hours_data,
          y: quarantined_data,
          type: 'scatter',
          name: 'quarantined'
        };

        var recovered = {
          x: hours_data,
          y: recovered_data,
          type: 'scatter',
          name: 'recovered'
        };

        var deceased = {
          x: hours_data,
          y: deceased_data,
          type: 'scatter',
          name: 'deceased'
        };

        var patients = [susceptible, infected, quarantined, recovered, deceased];
        var layout = {
          xaxis: {
          },

          legend: {
            x: 1,
            y: 3.5
          }
        };
        Plotly.newPlot('myDiv', patients, layout);

      }, 100);
    button.addEventListener('click', function() {
      clearInterval(interval);
      readStream();
      controller.close();
    });
  },
  pull(controller) {
      // We don't really need a pull in this example
  },
  cancel() {
      // This is called if the reader cancels,
      // so we should stop generating strings
    clearInterval(interval);
  }
});


function readStream() {
  const reader = stream.getReader();
    // read() returns a promise that resolves
    // when a value has been received
    reader.read().then(function processItem({ done, value }) {
      // Result objects contain two properties:
      // done  - true if the stream has already given you all its data.
      // value - some data. Always undefined when done is true.
      if (done) {
        console.log("Stream complete");
        return;
      }
      // Read some more, and call this function again
      return reader.read().then(processItem);
    });
  }
