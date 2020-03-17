import React from 'react';
import ParamterInputForm from './ParameterInputForm';
import { useState } from 'react';
import './app.scss';
import SocketAwareGraph from './SocketAwareGraph';

function App() {
  const [displayChartKey, setDisplayChartKey] = useState(null);

  function handleFormSubmit() {
    setDisplayChartKey(true);
  }

  return (
    <>
      <nav className="navbar navbar-expand-lg navbar-dark bg-dark">
        <a className="navbar-brand" href="/">EpiViz</a>
      </nav>
      <div className="container mt-4">
        <ParamterInputForm onSubmit={handleFormSubmit} />
        {displayChartKey && <SocketAwareGraph num={displayChartKey} />}
      </div>
    </>
  );
}

export default App;