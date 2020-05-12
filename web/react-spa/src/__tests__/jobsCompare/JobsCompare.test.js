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

import React from "react";
import ShallowRenderer from 'react-test-renderer/shallow';
import {fireEvent, render} from '@testing-library/react'
import JobsCompare from "../../jobsCompare/JobsCompare";
import {get} from '../../common/apiCall'
import {flushPromises} from "../helper/promiseHelper";
import {act} from "react-dom/test-utils";
import GraphUpdater from '../../jobsCompare/GraphUpdater'
import {reduceStatus} from "../../jobs/JobTransformer";

jest.mock('../../common/apiCall')
jest.mock('../../jobsCompare/GraphUpdater')
jest.mock('../../jobs/JobTransformer')

describe('Jobs Compare', function () {
  const jobs = [
    {_id: '1', status: 'finished', simulations: [{_id: "345678iugf"}]},
    {_id: '2', status: 'finished', simulations: [{_id: "45678iugjj"}]}
  ];

  it('should render with default state', function () {
    const renderer = new ShallowRenderer();
    renderer.render(<JobsCompare/>)

    expect(renderer.getRenderOutput()).toMatchSnapshot()
  });

  it('should render with jobs fetched that are finished', async function () {
    const jobs = [
      {_id: 1, status: 'finished'},
      {_id: 2, status: 'running'},
      {_id: 3, status: 'in-queue'},
      {_id: 4, status: 'finished'},
      {_id: 5, status: 'failed'}
    ];
    get.mockResolvedValueOnce({json: jest.fn().mockResolvedValueOnce(jobs)})
    const {container} = render(<JobsCompare/>)

    await act(async () => {
      await flushPromises()
    })

    const dropdowns = container.querySelectorAll('select')
    expect(dropdowns).toMatchSnapshot()
  });

  it('should display failed message when fetching jobs has failed', async function () {
    get.mockRejectedValue()
    const {container} = render(<JobsCompare/>)

    await act(async () => {
      await flushPromises()
    })

    expect(container).toMatchSnapshot()
  });

  it('should start jobs fetcher on compare click', async () => {
    get.mockResolvedValueOnce({json: jest.fn().mockResolvedValueOnce(jobs)})
    const mockStart = jest.fn();
    GraphUpdater.mockImplementation(() => ({'start': mockStart, 'stop': jest.fn()}))
    const {container} = render(<JobsCompare/>)

    await act(async () => await flushPromises())

    const dropdowns = container.querySelectorAll('.form-control');
    fireEvent.change(dropdowns[0], {target: {value: '1'}})
    fireEvent.change(dropdowns[1], {target: {value: '2'}})
    fireEvent.click(container.querySelector('button'))

    expect(GraphUpdater).toHaveBeenCalledTimes(1)
    expect(GraphUpdater).toHaveBeenCalledWith(expect.any(Function), '1', '2')
    expect(mockStart).toHaveBeenCalledTimes(1)
  });

  it.skip('should update graph when graph updater updates value', async () => {
    get.mockResolvedValueOnce({json: jest.fn().mockResolvedValueOnce(jobs)})
    const mockStart = jest.fn();
    GraphUpdater.mockImplementation(() => ({'start': mockStart, 'stop': jest.fn()}))
    const {container} = render(<JobsCompare/>)

    await act(async () => await flushPromises())

    const dropdowns = container.querySelectorAll('.form-control');
    fireEvent.change(dropdowns[0], {target: {value: '1'}})
    fireEvent.change(dropdowns[1], {target: {value: '2'}})
    fireEvent.click(container.querySelector('button'))

    const updateFunction = GraphUpdater.mock.calls[0][0]
    act(() => {
      updateFunction([{hour: 1, job1: null, job2: null}])
    })
    expect(container).toMatchSnapshot()
  });

  beforeEach(() => {
    reduceStatus.mockImplementation(data => data)
  });

  afterEach(() => {
    jest.clearAllMocks();
  })
});
