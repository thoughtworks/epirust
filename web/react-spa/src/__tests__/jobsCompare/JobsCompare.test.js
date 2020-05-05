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
import {render} from '@testing-library/react'
import JobsCompare from "../../jobsCompare/JobsCompare";
import {get} from '../../common/apiCall'
import {flushPromises} from "../helper/promiseHelper";
import {act} from "react-dom/test-utils";

jest.mock('../../common/apiCall')

describe('Jobs Compare', function () {
  it('should render with default state', function () {
    const renderer = new ShallowRenderer();
    renderer.render(<JobsCompare/>)

    expect(renderer.getRenderOutput()).toMatchSnapshot()
  });

  it('should render with jobs fetched', async function () {
    const jobs = [{_id: 1}, {_id: 2}];
    get.mockResolvedValueOnce({json: jest.fn().mockResolvedValueOnce(jobs)})
    const {container} = render(<JobsCompare/>)

    await act(async () => {
      await flushPromises()
    })

    expect(container).toMatchSnapshot()
  });

  it('should display failed message when fetching jobs has failed', async function () {
    get.mockRejectedValue()
    const {container} = render(<JobsCompare/>)

    await act(async () => {
      await flushPromises()
    })

    expect(container).toMatchSnapshot()
  });
});