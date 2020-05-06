import React from "react";
import {act, render} from "@testing-library/react";
import {JobsRefresher} from "../../jobs/JobsRefresher"
import {JobsView} from "../../jobs";
import {LOADING_STATES} from "../../common/constants";
import {MemoryRouter, Route} from "react-router-dom";

jest.mock("../../jobs/JobsRefresher")

describe('JobsView', () => {

  it('should start jobs refresher on mount', () => {
    const mockStart = jest.fn();
    JobsRefresher.mockImplementation(() => ({start: mockStart}))

    render(<MemoryRouter initialEntries={["/jobs/"]}><JobsView/></MemoryRouter>)

    expect(JobsRefresher).toHaveBeenCalledTimes(1)
    expect(mockStart).toHaveBeenCalledTimes(1)
  });

  it('should render components with updated state on refresh', () => {
    const mockStart = jest.fn();
    JobsRefresher.mockImplementation(() => ({start: mockStart}))

    const {container} = render(<MemoryRouter initialEntries={["/jobs/"]}>
      <Route path={"/jobs/:id?/:view?"}><JobsView/></Route>
    </MemoryRouter>)

    expect(container).toMatchSnapshot()

    const [updateJob, updateLoadingState] = JobsRefresher.mock.calls[0]
    act(() => {
      updateLoadingState(LOADING_STATES.FINISHED)
      updateJob([
        {
          _id: "123456dc",
          status: "finished",
          config: {tags: []}
        },
        {
          _id: "567890gv",
          status: "finished",
          config: {tags: []}
        }])
    });

    expect(container).toMatchSnapshot()
  });

  afterEach(() => {
    jest.clearAllMocks();
  });
});
