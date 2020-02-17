import Plotly from 'plotly.js-dist';
import Data from './simulation_100_000.csv';


let hours_data = Data.map(function(row){
  return row[0]
});
hours_data = hours_data.splice(1,hours_data.length);

let susceptible_data = Data.map(function(row){
  return row[1]
});
susceptible_data = susceptible_data.splice(1,susceptible_data.length);

let infected_data = Data.map(function(row){
    return row[2]
});
infected_data = infected_data.splice(1,infected_data.length);

let quarantined_data = Data.map(function(row){
  return row[3]
});
quarantined_data = quarantined_data.splice(1, quarantined_data.length);


let recovered_data = Data.map(function(row){
  return row[4]
});
recovered_data = recovered_data.splice(1, recovered_data.length);


let deceased_data = Data.map(function(row){
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

var data = [susceptible, infected, quarantined, recovered, deceased];
var layout = {
  xaxis: {
  },

  legend: {
    x: 1,
    y: 3.5
  }
};
Plotly.newPlot('myDiv', data, layout);
