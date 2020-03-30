import React, { useState } from 'react';

export default function Interventions() {
    return (
        <fieldset>
            <legend>Interventions</legend>
            <small className="form-text text-muted">Only the active interventions will be taken into consideration</small>
            <VaccineInputs />
            <LockdownInputs />
            <HospitalSpreadInputs />
        </fieldset>
    );
}

function VaccineInputs() {
    const [showVaccine, setShowVaccine] = useState(true);

    function handleCheck(e) {
        setShowVaccine(e.target.checked)
    }

    return (
        <>
            <div className="custom-control custom-switch switch-right">
                <input onChange={handleCheck} type="checkbox" checked={showVaccine} className="custom-control-input" id="vaccine-switch" />
                <label className="custom-control-label col-form-label-sm" htmlFor="vaccine-switch">Vaccination</label>
            </div>

            {showVaccine && (
                <>
                    <div className="input-control">
                        <label className="col-form-label-sm" htmlFor="vaccinate_at">Vaccinate At</label>
                        <input type="number" name="vaccinate_at" className="form-control form-control-sm" id="vaccinate_at" aria-describedby="vaccinate_at" placeholder="Vaccinate At" defaultValue="5000" />
                    </div>
                    <div className="input-control">
                        <label className="col-form-label-sm" htmlFor="vaccinate_percentage">Vaccinate Percentage</label>
                        <input type="number" name="vaccinate_percentage" className="form-control form-control-sm" id="vaccinate_percentage" aria-describedby="vaccinate_percentage" placeholder="Vaccinate Percentage" defaultValue="0.2" step="any" />
                    </div>
                </>
            )}
        </>
    )
}

function LockdownInputs() {
    const [showLockdown, setShowLockdown] = useState(true);

    function handleCheck(e) {
        setShowLockdown(e.target.checked)
    }

    return (
        <>
            <div className="custom-control custom-switch switch-right">
                <input onChange={handleCheck} type="checkbox" checked={showLockdown} className="custom-control-input" id="lockdown-switch" />
                <label className="custom-control-label col-form-label-sm" htmlFor="lockdown-switch">Lockdown</label>
            </div>

            {showLockdown && (
                <>
                    <div className="input-control">
                        <label className="col-form-label-sm" htmlFor="lockdown_at_number_of_infections">Lockdown at number of infections</label>
                        <input type="number" name="lockdown_at_number_of_infections" className="form-control form-control-sm" id="lockdown_at_number_of_infections" aria-describedby="lockdown_at_number_of_infections" placeholder="Lockdown At(number of infections)" defaultValue="100" step="any" />
                    </div>
                    <div className="input-control">
                        <label className="col-form-label-sm" htmlFor="essential_workers_population">Essentials Workers Population</label>
                        <input type="number" name="essential_workers_population" className="form-control form-control-sm" id="essential_workers_population" aria-describedby="essential_workers_population" placeholder="Emergency Workers Population" defaultValue="0.1" step="any" />
                    </div>
                </>
            )}
        </>
    )
}

function HospitalSpreadInputs() {
    const [showHospitalSpread, setShowHospitalSpread] = useState(true);

    function handleCheck(e) {
        setShowHospitalSpread(e.target.checked)
    }

    return (
        <>
            <div className="custom-control custom-switch switch-right">
                <input onChange={handleCheck} type="checkbox" checked={showHospitalSpread} className="custom-control-input" id="hospital-switch" />
                <label className="custom-control-label col-form-label-sm" htmlFor="hospital-switch">Hospital Spread</label>
            </div>
            {showHospitalSpread && (
                <div className="input-control">
                    <label className="col-form-label-sm" htmlFor="hospital_spread_rate_threshold">Hospital Spread Rate Threshold</label>
                    <input type="number" name="hospital_spread_rate_threshold" className="form-control form-control-sm" id="hospital_spread_rate_threshold" aria-describedby="hospital_spread_rate_threshold" placeholder="Hospital Spread Rate Threshold" defaultValue="100" step="any" />
                </div>
            )}
        </>
    );
}

