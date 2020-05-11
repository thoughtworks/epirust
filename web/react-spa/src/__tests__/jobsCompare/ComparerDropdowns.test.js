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
import {render} from '@testing-library/react'
import ComparerDropdowns from "../../jobsCompare/ComparerDropdowns";
import userEvent from "@testing-library/user-event";

describe('Comparer Dropdowns', () => {
  const testJobs = [{_id: 'id1'}, {_id: 'id2'}];

  it('should render inputs to select jobs to compare', () => {
    const {asFragment} = render(<ComparerDropdowns jobs={testJobs}/>);
    expect(asFragment()).toMatchSnapshot()
  });

  it('should return selected jobs when compare button clicked', () => {
    const mockOnCompare = jest.fn()
    const {getByRole, getByLabelText} = render(<ComparerDropdowns jobs={testJobs} onCompare={mockOnCompare}/>);

    userEvent.selectOptions(getByLabelText('Job 1'), ['id1']);
    userEvent.selectOptions(getByLabelText('Job 2'), ['id2']);
    userEvent.click(getByRole('button', /Compare/));

    expect(mockOnCompare).toHaveBeenCalledTimes(1)
    expect(mockOnCompare).toHaveBeenCalledWith({job1: 'id1', job2: 'id2'})
  });

  it('should show error message if both the selected jobs are same on compare click', () => {
    const {queryByText, getByRole, getByLabelText} = render(<ComparerDropdowns jobs={testJobs}/>);

    userEvent.selectOptions(getByLabelText('Job 1'), ['id1']);
    userEvent.selectOptions(getByLabelText('Job 2'), ['id1']);
    userEvent.click(getByRole('button', /Compare/));

    expect(queryByText("Can't compare same jobs!")).toBeInTheDocument()
  });
});

