import React from 'react';
import EpiSimulationParamForm from "./epiSimulationParamForm";
import VegaLine from './VegaLine';
function App() {
  return (
    <>
      <nav className="navbar navbar-expand-lg navbar-dark bg-dark">
        <a className="navbar-brand" href="/">EpiViz</a>
      </nav>
      <div className="container mt-4">
        <EpiSimulationParamForm />
        {/* <VegaLine></VegaLine> */}
      </div>
    </>
  );
}

export default App;
