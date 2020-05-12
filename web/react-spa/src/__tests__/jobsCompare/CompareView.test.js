/*
 * EpiRust
 * Copyright (c) 2020  ThoughtWorks, Inc.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

import GraphUpdater from "../../jobsCompare/GraphUpdater";
import Dygraph from "dygraphs";
import {CompareView} from "../../jobsCompare/CompareView";
import React from "react";
import {render} from "@testing-library/react";
import selectEvent from "react-select-event";
import {wait} from "@testing-library/dom";
import userEvent from "@testing-library/user-event";
import {get} from "../../common/apiCall";
import {epiCurves} from "../../common/constants";

jest.mock('dygraphs');
jest.mock("../../jobsCompare/GraphUpdater");
jest.mock("../../common/apiCall");

describe('CompareView', () => {

  it('should unmount and render the graph again on new comparison', () => {

    const graphData = [{hour: 1, job1: null, job2: null}];
    const startFnMock = updateBuffer => jest.fn().mockImplementationOnce(() => updateBuffer(graphData));
    const startFnMockReturningEmpty = updateBuffer => jest.fn().mockImplementationOnce(() => updateBuffer([]));

    GraphUpdater
      .mockImplementationOnce((updateBuffer) => ({'start': startFnMock(updateBuffer), 'stop': jest.fn()}))
      .mockImplementationOnce((updateBuffer) => ({
        'start': startFnMockReturningEmpty(updateBuffer),
        'stop': jest.fn()
      }));

    const setVisibilitySpy = jest.fn();
    const updateOptionsSpy = jest.fn();

    const dygraphMockFn = Dygraph.mockImplementation(() => ({
      updateOptions: updateOptionsSpy,
      setVisibility: setVisibilitySpy
    }));

    const jobId1 = "111", jobId2 = "222";
    const job1 = {_id: jobId1, simulations: []}, job2 = {_id: jobId2, simulations: []};

    const {rerender} = render(<CompareView selectedJobs={{job1, job2}}/>);

    const job1Data = [[], [], [], [], []];
    const job2Data = [[], [], [], [], []];
    expect(dygraphMockFn).toHaveBeenCalledWith(expect.any(HTMLElement), [[1, ...job1Data, ...job2Data]], expectedGraphOptions);
    dygraphMockFn.mockClear();

    rerender(<CompareView selectedJobs={{job1, job2}}/>);
    expect(dygraphMockFn).not.toHaveBeenCalled();
  });

  it('should render comparing graph on updates from GraphUpdater', () => {

    const graphData = [{hour: 1, job1: null, job2: null}];
    const startFnMock = updateBuffer => jest.fn().mockImplementationOnce(() => updateBuffer(graphData));
    GraphUpdater.mockImplementation((updateBuffer) => ({'start': startFnMock(updateBuffer), 'stop': jest.fn()}));

    const setVisibilitySpy = jest.fn();
    const updateOptionsSpy = jest.fn();

    const dygraphMockFn = Dygraph.mockImplementation(() => ({
      updateOptions: updateOptionsSpy,
      setVisibility: setVisibilitySpy
    }));

    const jobId1 = "111", jobId2 = "222";
    const job1 = {_id: jobId1, simulations: []}, job2 = {_id: jobId2, simulations: []};

    render(<CompareView selectedJobs={{job1, job2}}/>);

    const job1Data = [[], [], [], [], []];
    const job2Data = [[], [], [], [], []];

    expect(dygraphMockFn).toHaveBeenCalledWith(expect.any(HTMLElement), [[1, ...job1Data, ...job2Data]], expectedGraphOptions);
    expect(GraphUpdater).toHaveBeenCalledWith(expect.any(Function), jobId1, jobId2);
  });

  it('should render annotations for selected interventions on the graph', async () => {
    get
      .mockResolvedValueOnce({
        json: jest.fn().mockResolvedValueOnce([{
          hour: 1,
          interventions: [{intervention: "build_new_hospital", data: {}}]
        }])
      })
      .mockResolvedValueOnce({
        json: jest.fn().mockResolvedValueOnce([{hour: 3, interventions: [{intervention: "vaccination", data: {}}]}])
      });

    const startFnMock = updateBuffer => jest.fn().mockImplementationOnce(() => updateBuffer(graphData));
    GraphUpdater.mockImplementation((updateBuffer) => ({'start': startFnMock(updateBuffer), 'stop': jest.fn()}));

    const setVisibilitySpy = jest.fn();
    const setAnnotationsSpy = jest.fn();
    Dygraph.mockImplementation(() => ({
      updateOptions: jest.fn(),
      setVisibility: setVisibilitySpy,
      setAnnotations: setAnnotationsSpy
    }));

    const jobId1 = "111";
    const jobId2 = "222";

    const job1Simulation = {_id: 'sim-111'};
    const job2Simulation = {_id: 'sim-222'};

    const job1 = {_id: jobId1, simulations: [job1Simulation]};
    const job2 = {_id: jobId2, simulations: [job2Simulation]};

    const {getByLabelText, getByText} = render(<CompareView selectedJobs={{job1, job2}}/>);

    await selectEvent.select(getByLabelText("Job 1:"), job1Simulation._id);
    await selectEvent.select(getByLabelText("Job 2:"), job2Simulation._id);

    userEvent.click(getByText("Show Interventions"));

    expect(setVisibilitySpy).toHaveBeenNthCalledWith(1, [true, true, true, true, true, true, true, true, true, true]);

    await wait(() => {
      expect(setAnnotationsSpy).toHaveBeenCalledWith([
        {
          "attachAtBottom": true,
          "cssClass": "annotation hospital",
          "series": "susceptible_job1",
          "shortText": "Build Hospitals",
          "text": "Build Hospitals at 1",
          "tickHeight": 40,
          "x": 1,
        },
        {
          "attachAtBottom": true,
          "cssClass": "annotation vaccination",
          "series": "susceptible_job1",
          "shortText": "Vaccination",
          "text": "Vaccination at 3",
          "tickHeight": 80,
          "x": 3,
        }
      ]);
    });
  });

  it('should render line graph for selected series', async () => {
    get
      .mockResolvedValueOnce({
        json: jest.fn().mockResolvedValueOnce([{
          hour: 1,
          interventions: [{intervention: "build_new_hospital", data: {}}]
        }])
      })
      .mockResolvedValueOnce({
        json: jest.fn().mockResolvedValueOnce([{hour: 3, interventions: [{intervention: "vaccination", data: {}}]}])
      });

    const startFnMock = updateBuffer => jest.fn().mockImplementationOnce(() => updateBuffer(graphData));
    GraphUpdater.mockImplementation((updateBuffer) => ({'start': startFnMock(updateBuffer), 'stop': jest.fn()}));

    const setVisibilitySpy = jest.fn();
    const setAnnotationsSpy = jest.fn();
    Dygraph.mockImplementation(() => ({
      updateOptions: jest.fn(),
      setVisibility: setVisibilitySpy,
      setAnnotations: setAnnotationsSpy
    }));

    const jobId1 = "111";
    const jobId2 = "222";

    const job1 = {_id: jobId1, simulations: []};
    const job2 = {_id: jobId2, simulations: []};

    const {getByLabelText} = render(<CompareView selectedJobs={{job1, job2}}/>);

    await selectEvent.select(getByLabelText("Select line-graphs:"), epiCurves[0]);

    expect(setVisibilitySpy).toHaveBeenNthCalledWith(1, [true, true, true, true, true, true, true, true, true, true]);

    await wait(() => {
      expect(setVisibilitySpy).toHaveBeenNthCalledWith(2, [true, false, false, false, false, true, false, false, false, false]);
    })
  });

  it('should render annotations for a series and change annotation when another series is selected', async () => {
    get
      .mockResolvedValueOnce({
        json: jest.fn().mockResolvedValueOnce([{
          hour: 1,
          interventions: [{intervention: "build_new_hospital", data: {}}]
        }])
      })
      .mockResolvedValueOnce({
        json: jest.fn().mockResolvedValueOnce([{hour: 3, interventions: [{intervention: "vaccination", data: {}}]}])
      });

    const startFnMock = updateBuffer => jest.fn().mockImplementationOnce(() => updateBuffer(graphData));
    GraphUpdater.mockImplementation((updateBuffer) => ({'start': startFnMock(updateBuffer), 'stop': jest.fn()}));

    const setVisibilitySpy = jest.fn();
    const setAnnotationsSpy = jest.fn();
    Dygraph.mockImplementation(() => ({
      updateOptions: jest.fn(),
      setVisibility: setVisibilitySpy,
      setAnnotations: setAnnotationsSpy
    }));

    const job1Simulation = {_id: 'sim-111'};
    const job2Simulation = {_id: 'sim-222'};

    const job1 = {_id: 'jobId1', simulations: [job1Simulation]};
    const job2 = {_id: 'jobId2', simulations: [job2Simulation]};

    const {getByLabelText, getByText} = render(<CompareView selectedJobs={{job1, job2}}/>);

    await selectEvent.select(getByLabelText("Job 1:"), job1Simulation._id);
    await selectEvent.select(getByLabelText("Job 2:"), job2Simulation._id);
    userEvent.click(getByText("Show Interventions"));

    expect(setVisibilitySpy).toHaveBeenNthCalledWith(1, [true, true, true, true, true, true, true, true, true, true]);

    await wait(() => {
      const [{series: annotation1Series}, {series: annotation2Series}] = setAnnotationsSpy.mock.calls[0][0];
      expect(annotation1Series).toBe("susceptible_job1");
      expect(annotation2Series).toBe("susceptible_job1");
    });

    await selectEvent.select(getByLabelText("Select line-graphs:"), epiCurves[1]);

    await wait(() => {
      const [{series: annotation1Series}, {series: annotation2Series}] = setAnnotationsSpy.mock.calls[1][0];
      expect(annotation1Series).toBe("infected_job1");
      expect(annotation2Series).toBe("infected_job1");
    })
  });

  const expectedGraphOptions = {
    "animatedZooms": true,
    "colors": ["#00a987", "#005b98", "#8900c1", "#e46b00", "#2c3e50", "#1abc9ccc", "#3498dbcc", "#9b59b6cc", "#e67e22cc", "#7f8c8d"],
    "errorBars": true,
    "labels": [
      "hour", "susceptible_job1", "infected_job1", "hospitalized_job1", "recovered_job1", "deceased_job1",
      "susceptible_job2", "infected_job2", "hospitalized_job2", "recovered_job2", "deceased_job2"
    ],
    "legend": "always",
    "rollPeriod": 24,
    "showRoller": true,
    "title": "Time Series Graph",
    "xlabel": "Hours",
    "ylabel": "Number of Agents"
  };
  const graphData = [{
    hour: 1,
    job1: {susceptible: 10, susceptible_std: 0},
    job2: {susceptible: 10, susceptible_std: 0}
  }];
});
