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
import {fireEvent, render} from '@testing-library/react'
import JobsDropdown from "../../jobsCompare/JobsDropdown";

describe('Jobs DropDown', () => {
  it('should render dropdown with job id list', function () {
    const jobs = [{_id: "id1"}, {_id: "id2"}]
    const {container} = render(<JobsDropdown jobs={jobs} label="job 1"/>);

    expect(container).toMatchSnapshot()
  });

  it('should not break when dropdown is updated', function () {
    const jobs = [{_id: "id1"}, {_id: "id2"}]
    const {container} = render(<JobsDropdown jobs={jobs} label="job 1"/>);

    expect(() => fireEvent.change(container.querySelector('.form-control'), {target: {value: 'id1'}}))
      .not.toThrow(Error)
  });
});

