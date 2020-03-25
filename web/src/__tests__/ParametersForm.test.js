import React from 'react'
import { render, fireEvent } from '@testing-library/react'
import ParametersForm from '../ParamtersForm'
import renderer from 'react-test-renderer'

test('should render ParametersForm with defaults', () => {
    const component = renderer.create(<ParametersForm onDataSubmit={jest.fn()} />)
    let tree = component.toJSON()

    expect(tree).toMatchSnapshot()
})

test('invoke onsubmit handler passed on form submit', () => {
    const handleSubmitData = jest.fn()
    const { getByTestId } = render(<ParametersForm onDataSubmit={handleSubmitData} />)

    fireEvent.submit(getByTestId('simulationForm'))

    expect(handleSubmitData).toHaveBeenCalledTimes(1)
})

test('invoke handler on form submit', () => {
    const handleSubmitData = jest.fn()
    const { getByTestId } = render(<ParametersForm onDataSubmit={handleSubmitData} />)
    const expectedData = {
        "death_rate": 0.2,
        "disease_name": "small_pox",
        "emergency_workers_population": 0.1,
        "grid_size": 250,
        "high_transmission_rate": 0.5,
        "high_transmission_start_day": 16,
        "hospital_spread_rate_threshold": 100,
        "last_day": 22,
        "lockdown_at_number_of_infections": 100,
        "number_of_agents": 10000,
        "public_transport_percentage": 0.2,
        "regular_transmission_rate": 0.05,
        "regular_transmission_start_day": 10,
        "simulation_hrs": 10000,
        "vaccinate_at": 5000,
        "vaccinate_percentage": 0.2,
        "working_percentage": 0.7,
    }

    fireEvent.submit(getByTestId('simulationForm'))
    expect(handleSubmitData).toHaveBeenCalledWith(expectedData)
})

