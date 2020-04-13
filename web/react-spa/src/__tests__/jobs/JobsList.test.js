import { render } from "@testing-library/react";
import { JobsList } from "../../jobs/JobsList";
import React from "react";
import { MemoryRouter } from "react-router-dom";
import { act } from "react-dom/test-utils";

describe('JobsList', function () {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  it('should fetch simulations when id and view is not selected', async function () {
    const mockResponse = Promise.resolve([{ simulation_id: 1234, status: "finished" }]);
    const mockJson = jest.fn().mockReturnValueOnce(mockResponse);
    const mockPromise = { json: mockJson };

    jest.spyOn(global, 'fetch')
      .mockImplementation(() => Promise.resolve(mockPromise));

    jest.mock('react-router-dom', () => {
      return {
        ...jest.requireActual('react-router-dom'),
        useParams: () => ({})
      }
    });

    const { asFragment } = await render(<MemoryRouter><JobsList /></MemoryRouter>);

    expect(asFragment()).toMatchSnapshot();
    expect(fetch).toHaveBeenCalledTimes(1);
    expect(mockJson).toHaveBeenCalledTimes(1);
  });
});