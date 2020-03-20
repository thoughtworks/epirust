import React from 'react'
import { render, fireEvent } from '@testing-library/react'
import ParametersForm from '../ParamtersForm'
import renderer from 'react-test-renderer'

test('should render ParametersForm with defaults', () => {
    const component = renderer.create(<ParametersForm onDataSubmit={jest.fn()}/>)
    let tree = component.toJSON()

    expect(tree).toMatchSnapshot()
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

