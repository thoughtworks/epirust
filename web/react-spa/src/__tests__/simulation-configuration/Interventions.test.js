import React from 'react'
import { render, fireEvent, prettyDOM } from '@testing-library/react'
import Interventions from '../../simulation-configuration/Interventions'

test('should toggle displaying the inputs on clicking the switch for vaccination intervention', () => {
    const { getByLabelText, container } = render(<Interventions />);

    expect(getByLabelText('Vaccination').checked).toBe(true)

    expect(getByLabelText('Vaccinate At')).toBeInTheDocument()
    expect(getByLabelText('Vaccinate Percentage')).toBeInTheDocument()

    fireEvent.click(getByLabelText('Vaccination'))

    expect(getByLabelText('Vaccination').checked).toBe(false);

    expect(container.querySelector('vaccinate_at')).not.toBeInTheDocument()
    expect(container.querySelector('vaccinate_percentage')).not.toBeInTheDocument()
})

test('should toggle displaying the inputs on clicking the switch for lockdown intervention', () => {
    const { getByLabelText, container } = render(<Interventions />);

    expect(getByLabelText('Lockdown').checked).toBe(true)

    expect(getByLabelText('Vaccinate At')).toBeInTheDocument()
    expect(getByLabelText('Vaccinate Percentage')).toBeInTheDocument()

    fireEvent.click(getByLabelText('Lockdown'))

    expect(getByLabelText('Lockdown').checked).toBe(false);

    expect(container.querySelector('lockdown_at_number_of_infections')).not.toBeInTheDocument()
    expect(container.querySelector('essential_workers_population')).not.toBeInTheDocument()
})

test('should toggle displaying the inputs on clicking the switch for hospital spread intervention', () => {
    const { getByLabelText, container } = render(<Interventions />);

    expect(getByLabelText('Hospital Spread').checked).toBe(true)
    expect(getByLabelText('Hospital Spread Rate Threshold')).toBeInTheDocument()

    fireEvent.click(getByLabelText('Hospital Spread'))

    expect(getByLabelText('Hospital Spread').checked).toBe(false);
    expect(container.querySelector('hospital_spread_rate_threshold')).not.toBeInTheDocument()
})

