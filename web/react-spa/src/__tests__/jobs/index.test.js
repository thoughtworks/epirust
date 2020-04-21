import { render, act } from "@testing-library/react";
import { JobsList } from "../../jobs";
import React from "react";
import MockSocket from 'socket.io-mock'


import io from 'socket.io-client'
jest.mock('socket.io-client')

jest.useFakeTimers()

jest.mock('react-router-dom', () => {
  return {
    ...jest.requireActual('react-router-dom'),
    useParams: () => ({
      id: 123, view: "time-series"
    })
  }
});

const flushPromises = () => new Promise(setImmediate);

import { MemoryRouter } from "react-router-dom";

afterEach(() => {
  jest.clearAllMocks();
});

test('should fetch simulation status from API to show status on UI', async function () {
  const mockResponse = Promise.resolve([{ simulation_id: 1234, status: "finished" }]);

  const mockPromise = { json: jest.fn().mockReturnValueOnce(mockResponse) };
  jest.spyOn(global, 'fetch').mockResolvedValue(mockPromise)

  io.mockImplementation(() => jest.fn().mockReturnValueOnce({
    close: jest.fn(),
    on: jest.fn()
  }))

  const { asFragment } = await render(
    <MemoryRouter>
      <JobsList />
    </MemoryRouter>
  );

  await act(async () => {
    await flushPromises()
  })

  expect(fetch).toHaveBeenCalledTimes(1);
  expect(fetch).toHaveBeenCalledWith('http://localhost:3000/simulation/');

  expect(asFragment()).toMatchSnapshot();
});


test('should fetch simulation status from socket messages to update status on UI', async function () {
  const simulationId = 1234;
  const mockResponse = Promise.resolve([{ simulation_id: simulationId, status: "in-queue" }]);

  const mockPromise = { json: jest.fn().mockReturnValueOnce(mockResponse) };
  jest.spyOn(global, 'fetch').mockResolvedValue(mockPromise)

  const socket = new MockSocket();
  socket.socketClient.close = jest.fn()
  io.mockImplementation(() => socket.socketClient)

  const { getByTestId } = await render(
    <MemoryRouter>
      <JobsList />
    </MemoryRouter>
  );

  await act(async () => {
    await flushPromises()
  })

  expect(getByTestId(`job-status-${simulationId}`)).toHaveTextContent("In-Queue")

  act(() => {
    socket.emit('jobStatus', [{ simulation_id: simulationId, status: "running" }])
    jest.runAllTimers();
  })

  expect(getByTestId(`job-status-${simulationId}`)).toHaveTextContent("In-Progress")

  act(() => {
    socket.emit('jobStatus', [{ simulation_id: simulationId, status: "finished" }])
    jest.runAllTimers();
  })
  expect(getByTestId(`job-status-${simulationId}`)).toHaveTextContent("Finished")

  expect(io).toHaveBeenCalledTimes(1);
  expect(io).toHaveBeenCalledWith("http://localhost:3000/job-status");
});