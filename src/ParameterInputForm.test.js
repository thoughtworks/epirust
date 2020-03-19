import React from 'react'
import { render, fireEvent, getByTestId } from '@testing-library/react'
import ParameterInputForm from './ParameterInputForm'

test('renders ParameterInputForm with defaults', () => {
    const { getByText, getByLabelText } = render(<ParameterInputForm />)

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
    const { getByTestId } = render(<ParameterInputForm onSubmit={handleSubmitData} />)

    fireEvent.submit(getByTestId('simulationForm'))

    expect(handleSubmitData).toHaveBeenCalledTimes(1)
})

test('make API call on form submit', () => {
    jest.spyOn(global, 'fetch')
        .mockImplementation(() => Promise.resolve())
    const handleSubmitData = jest.fn()
    const { getByText, getByTestId } = render(<ParameterInputForm onSubmit={handleSubmitData} />)
    const expectedBody = {
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

    expect(global.fetch).toHaveBeenCalledTimes(1)
    expect(global.fetch.mock.calls[0][0]).toBe("http://localhost:3000/simulation/init")
    expect(global.fetch.mock.calls[0][1].method).toBe("POST")
    expect(global.fetch.mock.calls[0][1].body)
        .toBe(JSON.stringify(expectedBody))
})

