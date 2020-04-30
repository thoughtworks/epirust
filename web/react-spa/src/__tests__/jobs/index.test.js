import {act, render} from "@testing-library/react";
import {JobsList} from "../../jobs";
import React from "react";
import {MemoryRouter} from "react-router-dom";

jest.mock("../../common/apiCall")
import {get} from "../../common/apiCall";

jest.useFakeTimers()

jest.mock('react-router-dom', () => {
  return {
    ...jest.requireActual('react-router-dom'),
    useParams: () => ({
      id: 123, view: "time-series"
    })
  }
});

describe('Jobs', () => {
  it('should render loader while fetching data of status', async () => {
    const {container} = render(
      <MemoryRouter>
        <JobsList/>
      </MemoryRouter>
    );

    expect(container.querySelector('#loader')).toBeInTheDocument()

    await act(async() => {await flushPromises()})

    expect(container.querySelector('#loader')).not.toBeInTheDocument();
  });

  it('should fetch simulation status from socket to show status on UI', async () => {
    const {asFragment} = await render(
      <MemoryRouter>
        <JobsList/>
      </MemoryRouter>
    );

    await act(async () => {
      await flushPromises()
    })

    expect(asFragment()).toMatchSnapshot();
  });

  beforeEach(() => {
    get.mockResolvedValueOnce({json: () => Promise.resolve([{_id: "ad1234", simulations: [{status: "finished"}]}])})
  });

  const flushPromises = () => new Promise(setImmediate);

  afterEach(() => {
    jest.clearAllMocks();
  });
});
