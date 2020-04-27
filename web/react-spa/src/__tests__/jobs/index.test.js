import {act, render} from "@testing-library/react";
import {JobsList} from "../../jobs";
import React from "react";
import MockSocket from 'socket.io-mock'
import {MemoryRouter} from "react-router-dom";

jest.mock("../../jobs/JobTransformer")
import {reduceStatus} from "../../jobs/JobTransformer";

jest.mock('socket.io-client')
import io from 'socket.io-client'

jest.useFakeTimers()

jest.mock('react-router-dom', () => {
  return {
    ...jest.requireActual('react-router-dom'),
    useParams: () => ({
      id: 123, view: "time-series"
    })
  }
});

const jobId = 'ad1234';
const job = {_id: jobId, status: "finished", config: {}};

describe('Jobs', () => {
  let socket;

  it('should render loader while fetching data of status', async () => {
    const {container} = render(
      <MemoryRouter>
        <JobsList/>
      </MemoryRouter>
    );

    expect(container.querySelector('#loader')).toBeInTheDocument()

    await act(async () => {socket.emit('jobStatus', [job])})

    expect(container.querySelector('#loader')).not.toBeInTheDocument();
  });

  it('should fetch simulation status from socket to show status on UI', async () => {
    const {asFragment} = await render(
      <MemoryRouter>
        <JobsList/>
      </MemoryRouter>
    );

    await act(async () => {
      socket.emit('jobStatus', [job])
    })

    expect(asFragment()).toMatchSnapshot();
  });

  it('should fetch simulation status from socket messages to update status on UI', async () => {
    const {getByTestId} = await render(
      <MemoryRouter>
        <JobsList/>
      </MemoryRouter>
    );

    await act(async () => {
      socket.emit('jobStatus', [{...job, status: 'in-queue'}])
    })

    expect(getByTestId(`job-status-${jobId}`)).toHaveTextContent("In-Queue")

    act(() => {
      socket.emit('jobStatus', [{_id: jobId, status: "running"}])
      jest.runAllTimers();
    })

    expect(getByTestId(`job-status-${jobId}`)).toHaveTextContent("In-Progress")

    act(() => {
      socket.emit('jobStatus', [{_id: jobId, status: "finished"}])
      jest.runAllTimers();
    })
    expect(getByTestId(`job-status-${jobId}`)).toHaveTextContent("Finished")

    expect(io).toHaveBeenCalledTimes(1);
    expect(io).toHaveBeenCalledWith("http://localhost:3000/job-status");
  });

  beforeEach(() => {
    socket = new MockSocket();
    socket.socketClient.close = jest.fn()
    io.mockImplementation(() => socket.socketClient)

    reduceStatus.mockImplementation((x) => (x));
  });

  afterEach(() => {
    jest.clearAllMocks();
  });
});
