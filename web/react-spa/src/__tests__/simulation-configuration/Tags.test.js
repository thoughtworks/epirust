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

import Tags from '../../simulation-configuration/Tags'
import {render} from '@testing-library/react'
import React from 'react'
import selectEvent from 'react-select-event'
import {get} from "../../common/apiCall";

jest.mock("../../common/apiCall")

describe("Tags", () => {

  test('should render component with predefined tags', () => {
    get.mockResolvedValueOnce({
      json: jest.fn().mockResolvedValueOnce([
        {id: 'tagId1', name: "Smallpox"}
      ])
    });

    const {asFragment} = render(<Tags/>);
    expect(asFragment()).toMatchSnapshot();
  });

  test('should render component with predefined tags', async () => {
    get.mockResolvedValueOnce({
      json: jest.fn().mockResolvedValueOnce([
        {id: 'smallPoxId', name: "Smallpox"},
        {id: 'sarsId', name: "SARS"},
        {id: 'covidId', name: "COVID-19"}
      ])
    });

    const {getByTestId, getByLabelText} = render(
      <form data-testid={'test-form'}>
        <Tags/>
      </form>
    );

    expect(getByTestId('test-form')).toHaveFormValues({tags: ''}); // empty select

    await selectEvent.select(getByLabelText('Tags'), ['Smallpox', 'SARS']);
    expect(getByTestId('test-form')).toHaveFormValues({tags: ['smallPoxId', 'sarsId']});

    await selectEvent.select(getByLabelText('Tags'), 'COVID-19');
    expect(getByTestId('test-form')).toHaveFormValues({
      tags: ['smallPoxId', 'sarsId', 'covidId'],
    })
  })
});
