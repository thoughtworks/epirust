import React from 'react'
import { render, fireEvent, getByTestId } from '@testing-library/react'
import ParametersForm from './ParamtersForm'

test('renders ParameterInputForm with defaults', () => {
    const { getByText, getByLabelText } = render(<ParametersForm />)

    expect(getByLabelText('Number of Agents').value).toBe('10000')
    expect(getByLabelText('Disease Name').value).toBe('small_pox')
    expect(getByLabelText('Grid Size').value).toBe('250')
    expect(getByLabelText('Simulation Hours').value).toBe('10000')
    expect(getByLabelText('Public Transport Percentage').value).toBe('0.2')
    expect(getByLabelText('Working Percentage').value).toBe('0.7')
    expect(getByLabelText('Vaccinate At').value).toBe('5000')
    expect(getByLabelText('Vaccinate Percentage').value).toBe('0.2')
    expect(getByText('Start'))
})

test('invoke onsubmit handler passed on form submit', () => {
    const handleSubmitData = jest.fn()
    const { getByTestId } = render(<ParametersForm onDataSubmit={handleSubmitData} />)

    fireEvent.submit(getByTestId('simulationForm'))

    expect(handleSubmitData).toHaveBeenCalledTimes(1)
})

test('make API call on form submit', () => {
    const handleSubmitData = jest.fn()
    const { getByTestId } = render(<ParametersForm onDataSubmit={handleSubmitData} />)
    const expectedData = {
        "number_of_agents": 10000,
        "disease_name": "small_pox",
        "grid_size": 250,
        "simulation_hrs": 10000,
        "public_transport_percentage": 0.2,
        "working_percentage": 0.7,
        "vaccinate_at": 5000,
        "vaccinate_percentage": 0.2
    }

    fireEvent.submit(getByTestId('simulationForm'))
    expect(handleSubmitData).toHaveBeenCalledWith(expectedData)
})

