/*
 * EpiRust
 * Copyright (c) 2020  ThoughtWorks, Inc.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

import React, { useState } from 'react'
import { diseases } from './diseases'

export default function DiseaseDynamics() {
    const disease = diseases["small_pox"];

    function handleDiseaseSelect(event) {
        const diseaseSelected = diseases[event.target.value]
        const diseaseDynamics = ["regular_transmission_start_day", "high_transmission_start_day", "last_day", "regular_transmission_rate", "high_transmission_rate", "death_rate"]

        diseaseDynamics.map(field => { 
            document.getElementById(field).value = diseaseSelected[field] 
        })
    }

    return (
        <fieldset>
            <legend>Disease Dynamics</legend>
            <div className="input-control">
                <label htmlFor="disease_name">Disease Name</label>
                <select data-testid="select-disease" name="disease_name" onChange={handleDiseaseSelect} className="form-control" id="disease_name" aria-describedby="disease_name" placeholder="Disease Name" defaultValue="small_pox">
                    {Object.keys(diseases).map((disease_key) => {
                        return (<option key= {disease_key} value={disease_key}>{diseases[disease_key].label}</option>)
                    })}
                </select>
            </div>

            <div className="input-control">
                <label htmlFor="regular_transmission_start_day">Regular Transmission Start Day</label>
                <input type="text" name="regular_transmission_start_day" className="form-control" id="regular_transmission_start_day" aria-describedby="regular_transmission_start_day" placeholder="Regular Transmission Start Day" defaultValue={disease["regular_transmission_start_day"]}/>
            </div>

            <div className="input-control">
                <label htmlFor="high_transmission_start_day">High Transmission Start Day</label>
                <input type="text" name="high_transmission_start_day" className="form-control" id="high_transmission_start_day" aria-describedby="high_transmission_start_day" placeholder="High Transmission Start Day" defaultValue={disease["high_transmission_start_day"]}/>
            </div>

            <div className="input-control">
                <label htmlFor="last_day">Last Day</label>
                <input type="text" name="last_day" className="form-control" id="last_day" aria-describedby="last_day" placeholder="Last Day" defaultValue={disease["last_day"]}/>
            </div>

            <div className="input-control">
                <label htmlFor="regular_transmission_rate">Regular Transmission Rate</label>
                <input type="text" name="regular_transmission_rate" className="form-control" id="regular_transmission_rate" aria-describedby="regular_transmission_rate" placeholder="Regular Transmission Rate" defaultValue={disease["regular_transmission_rate"]}/>
            </div>

            <div className="input-control">
                <label htmlFor="high_transmission_rate">High Transmission Rate</label>
                <input type="text" name="high_transmission_rate" className="form-control" id="high_transmission_rate" aria-describedby="high_transmission_rate" placeholder="High Transmission Rate" defaultValue={disease["high_transmission_rate"]}/>
            </div>

            <div className="input-control">
                <label htmlFor="death_rate">Death Rate</label>
                <input type="text" name="death_rate" className="form-control" id="death_rate" aria-describedby="death_rate" placeholder="Death Rate" defaultValue={disease["death_rate"]}/>
            </div>
        </fieldset>
    )
}
