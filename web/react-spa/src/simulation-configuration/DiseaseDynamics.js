import React, {useState} from 'react'
import {diseases} from './diseases'

export default function DiseaseDynamics() {
  const [diseaseSelected, setDiseaseSelected] = useState("small_pox");
  const disease = diseases[diseaseSelected];
  const lastDayVariantsClass = disease["last_day"] === disease["asymptomatic_last_day"] ? "d-none" : "";
  return <fieldset>
    <legend>Disease Dynamics</legend>
    <div className="input-control">
      <label className="col-form-label-sm" htmlFor="disease_name">Disease Name</label>
      <select data-testid="select-disease" name="disease_name"
              onChange={event => setDiseaseSelected(event.target.value)} className="form-control form-control-sm"
              id="disease_name" aria-describedby="disease_name"
              placeholder="Disease Name" value={diseaseSelected}>
        {Object.keys(diseases).map((disease_key) => {
          return (<option key={disease_key} value={disease_key}>{diseases[disease_key].label}</option>)
        })}
      </select>
    </div>

    <div className="input-control">
      <label className="col-form-label-sm" htmlFor="regular_transmission_start_day">Regular Transmission Start
        Day</label>
      <input type="number" step="any" name="regular_transmission_start_day" className="form-control form-control-sm"
             id="regular_transmission_start_day" aria-describedby="regular_transmission_start_day"
             placeholder="Regular Transmission Start Day"
             value={disease["regular_transmission_start_day"]}/>
    </div>

    <div className="input-control">
      <label className="col-form-label-sm" htmlFor="high_transmission_start_day">High Transmission Start Day</label>
      <input type="number" step="any" name="high_transmission_start_day" className="form-control form-control-sm"
             id="high_transmission_start_day" aria-describedby="high_transmission_start_day"
             placeholder="High Transmission Start Day"
             value={disease["high_transmission_start_day"]}/>
    </div>

    <div className="input-control">
      <label className="col-form-label-sm" htmlFor="last_day">Last Day</label>
      <input type="number" step="any" name="last_day" className="form-control form-control-sm" id="last_day"
             aria-describedby="last_day" placeholder="Last Day" value={disease["last_day"]}/>
    </div>

    <div className={`input-control ${lastDayVariantsClass}`}>
      <label className="col-form-label-sm" htmlFor="asymptomatic_last_day">Asymptomatic Last Day</label>
      <input type="number" step="any" name="asymptomatic_last_day" className="form-control form-control-sm"
             id="asymptomatic_last_day" aria-describedby="asymptomatic_last_day"
             placeholder="Asymptomatic Last Day" value={disease["asymptomatic_last_day"]}/>
    </div>
    <div className={`input-control ${lastDayVariantsClass}`}>
      <label className="col-form-label-sm" htmlFor="mild_infected_last_day">Mild Infected Last Day</label>
      <input type="number" step="any" name="mild_infected_last_day" className="form-control form-control-sm"
             id="mild_infected_last_day" aria-describedby="mild_infected_last_day"
             placeholder="Mild Infected Last Day" value={disease["mild_infected_last_day"]}/>
    </div>

    <div className="input-control">
      <label className="col-form-label-sm" htmlFor="regular_transmission_rate">Regular Transmission Rate</label>
      <input type="number" step="any" min="0" max="1" name="regular_transmission_rate"
             className="form-control form-control-sm" id="regular_transmission_rate"
             aria-describedby="regular_transmission_rate" placeholder="Regular Transmission Rate"
             value={disease["regular_transmission_rate"]}/>
    </div>

    <div className="input-control">
      <label className="col-form-label-sm" htmlFor="high_transmission_rate">High Transmission Rate</label>
      <input type="number" step="any" min="0" max="1" name="high_transmission_rate"
             className="form-control form-control-sm" id="high_transmission_rate"
             aria-describedby="high_transmission_rate" placeholder="High Transmission Rate"
             value={disease["high_transmission_rate"]}/>
    </div>

    <div className="input-control">
      <label className="col-form-label-sm" htmlFor="death_rate">Death Rate</label>
      <input type="number" step="any" min="0" max="1" name="death_rate" className="form-control form-control-sm"
             id="death_rate" aria-describedby="death_rate" placeholder="Death Rate"
             value={disease["death_rate"]}/>
    </div>

    <div className="input-control d-none">
      <label className="col-form-label-sm" htmlFor="percentage_asymptomatic_population">Asymptomatic
        percentage</label>
      <input min="0" max="1" type="number" step="any" name="percentage_asymptomatic_population"
             className="form-control form-control-sm" id="percentage_asymptomatic_population"
             aria-describedby="percentage_asymptomatic_population" placeholder="Asymptomatic percentage"
             value={disease["percentage_asymptomatic_population"]}/>
    </div>

    <div className="input-control d-none">
      <label className="col-form-label-sm" htmlFor="percentage_severe_infected_population">% Severe Infected
        Population</label>
      <input min="0" max="1" type="number" step="any" name="percentage_severe_infected_population"
             className="form-control form-control-sm" id="percentage_severe_infected_population"
             aria-describedby="percentage_severe_infected_population"
             placeholder="Percentage Severe Infected Population"
             value={disease["percentage_severe_infected_population"]}/>
    </div>

    <div className="input-control d-none">
      <label className="col-form-label-sm" htmlFor="exposed_duration">Exposed Duration</label>
      <input type="number" step="any" name="exposed_duration" className="form-control form-control-sm"
             id="exposed_duration" aria-describedby="exposed_duration" placeholder="Exposed Duration"
             value={disease["exposed_duration"]}/>
    </div>

    <div className="input-control d-none">
      <label className="col-form-label-sm" htmlFor="pre_symptomatic_duration">Pre symptomatic Duration</label>
      <input type="number" step="any" name="pre_symptomatic_duration" className="form-control form-control-sm"
             id="pre_symptomatic_duration" aria-describedby="pre_symptomatic_duration"
             placeholder="Pre symptomatic Duration" value={disease["pre_symptomatic_duration"]}/>
    </div>
  </fieldset>
}
