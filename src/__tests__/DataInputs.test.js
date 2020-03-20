import React from 'react'
import { render, fireEvent, getByTestId } from '@testing-library/react'
import DataInputs from '../DataInputs'

jest.useFakeTimers();

test('invoke submit form data to server on submit', () => {
    jest.spyOn(global, 'fetch')
        .mockImplementation(() => Promise.resolve())

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

    const onSubmitSpy = jest.fn()
    const { getByTestId } = render(<DataInputs onSubmit={onSubmitSpy} onFileDataInput={jest.fn()}/>)

    fireEvent.submit(getByTestId('simulationForm'))

    expect(onSubmitSpy).toHaveBeenCalledWith()

    expect(global.fetch).toHaveBeenCalledTimes(1)
    expect(global.fetch.mock.calls[0][0]).toBe("http://localhost:3000/simulation/init")
    expect(global.fetch.mock.calls[0][1].method).toBe("POST")
    expect(global.fetch.mock.calls[0][1].body).toBe(JSON.stringify(expectedBody))
})

test('invoke onsubmit handler passed on a file upload', () => {
    const fileBlobObject = {}

    jest.spyOn(window, 'FileReader')
        .mockImplementation(function () {
            this.readAsBinaryString = function () {
                setTimeout(this.onloadend, 0, {
                    target: {
                        readyState: 2,
                        result: "[[1,2,3,4,5],[1,2,3,4,5]]"
                    }
                });
            };
        })
    window.FileReader.DONE = 2

    const onFileDataInputSpy = jest.fn(()=>"ABCBD")
    const { getByTestId } = render(<DataInputs onSubmit={jest.fn()} onFileDataInput={onFileDataInputSpy} />)

    fireEvent.change(getByTestId('import-input'), {
        target: {
            files: [
                { size: 100, slice: () => fileBlobObject }
            ]
        }
    })

    jest.runAllTimers();


    expect(window.FileReader).toHaveBeenCalled()
    expect(onFileDataInputSpy).toHaveBeenCalledTimes(1)
})