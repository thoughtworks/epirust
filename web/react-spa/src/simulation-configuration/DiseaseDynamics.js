import React from 'react'
import { diseases } from './diseases'

export default function DiseaseDynamics() {
    const disease = diseases["small_pox"];

    function handleDiseaseSelect(event) {
        const diseaseSelected = diseases[event.target.value]
        const diseaseDynamics = ["regular_transmission_start_day", "high_transmission_start_day", "last_day", "regular_transmission_rate", "high_transmission_rate", "death_rate"]

        diseaseDynamics.forEach(field => {
            document.getElementById(field).value = diseaseSelected[field]
        })
    }

    return (
        <fieldset>
            <legend>Disease Dynamics</legend>
            <div className="input-control">
                <label className="col-form-label-sm" htmlFor="disease_name">Disease Name</label>
                <select data-testid="select-disease" name="disease_name" onChange={handleDiseaseSelect} className="form-control form-control-sm" id="disease_name" aria-describedby="disease_name" placeholder="Disease Name" defaultValue="small_pox">
                    {Object.keys(diseases).map((disease_key) => {
                        return (<option key={disease_key} value={disease_key}>{diseases[disease_key].label}</option>)
                    })}
                </select>
            </div>

            <div className="input-control">
                <label className="col-form-label-sm" htmlFor="regular_transmission_start_day">Regular Transmission Start Day</label>
                <input type="text" name="regular_transmission_start_day" className="form-control form-control-sm" id="regular_transmission_start_day" aria-describedby="regular_transmission_start_day" placeholder="Regular Transmission Start Day" defaultValue={disease["regular_transmission_start_day"]} />
            </div>

            <div className="input-control">
                <label className="col-form-label-sm" htmlFor="high_transmission_start_day">High Transmission Start Day</label>
                <input type="text" name="high_transmission_start_day" className="form-control form-control-sm" id="high_transmission_start_day" aria-describedby="high_transmission_start_day" placeholder="High Transmission Start Day" defaultValue={disease["high_transmission_start_day"]} />
            </div>

            <div className="input-control">
                <label className="col-form-label-sm" htmlFor="last_day">Last Day</label>
                <input type="text" name="last_day" className="form-control form-control-sm" id="last_day" aria-describedby="last_day" placeholder="Last Day" defaultValue={disease["last_day"]} />
            </div>

            <div className="input-control">
                <label className="col-form-label-sm" htmlFor="regular_transmission_rate">Regular Transmission Rate</label>
                <input type="text" name="regular_transmission_rate" className="form-control form-control-sm" id="regular_transmission_rate" aria-describedby="regular_transmission_rate" placeholder="Regular Transmission Rate" defaultValue={disease["regular_transmission_rate"]} />
            </div>

            <div className="input-control">
                <label className="col-form-label-sm" htmlFor="high_transmission_rate">High Transmission Rate</label>
                <input type="text" name="high_transmission_rate" className="form-control form-control-sm" id="high_transmission_rate" aria-describedby="high_transmission_rate" placeholder="High Transmission Rate" defaultValue={disease["high_transmission_rate"]} />
            </div>

            <div className="input-control">
                <label className="col-form-label-sm" htmlFor="death_rate">Death Rate</label>
                <input type="text" name="death_rate" className="form-control form-control-sm" id="death_rate" aria-describedby="death_rate" placeholder="Death Rate" defaultValue={disease["death_rate"]} />
            </div>
        </fieldset>
    )
}
