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

import TagsSelect from '../../simulation-configuration/TagsSelect'
import {render} from '@testing-library/react'
import React from 'react'
import selectEvent from 'react-select-event'
import {get} from "../../common/apiCall";
import {wait} from "@testing-library/dom";

jest.mock("../../common/apiCall")

describe("TagsSelect", () => {

  test('should render component with predefined tags', () => {
    get.mockResolvedValueOnce({
      json: jest.fn().mockResolvedValueOnce([
        {id: 'tagId1', name: "Smallpox"}
      ])
    });

    const {asFragment} = render(<TagsSelect placeholder={"placeholderText"} label={"Tags"}/>);
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
        <TagsSelect placeholder={"placeholderText"} label={"Tags"}/>
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

  test('should behave as a controlled component when passed values from outside', async () => {
    const smallPoxId = 'smallPoxId', sarsId = 'sarsId', covidId = 'covidId';

    get.mockResolvedValueOnce({
      json: jest.fn().mockResolvedValueOnce([
        {id: smallPoxId, name: "Smallpox"},
        {id: sarsId, name: "SARS"},
        {id: covidId, name: "COVID-19"}
      ])
    });

    const onChangeSpy = jest.fn();

    const {getByTestId, getByLabelText, rerender} = render(
      <form data-testid={'test-form'}>
        <TagsSelect
          placeholder={"placeholderText"}
          label={"Tags"}
          tagIdsSelected={[smallPoxId, sarsId]}
          onChange={onChangeSpy}
        />
      </form>
    );

    await wait(() => {
      expect(getByTestId('test-form')).toHaveFormValues({tags: [smallPoxId, sarsId]});
    });


    //selecting a value on a controlled component does have not effect on the selection, it has to come as value prop to Select
    await selectEvent.select(getByLabelText('Tags'), 'COVID-19');

    expect(getByTestId('test-form')).toHaveFormValues({tags: [smallPoxId, sarsId]});
    expect(onChangeSpy).toHaveBeenCalledWith(
      [{label: "Smallpox", value: smallPoxId}, {label: "SARS", value: sarsId}, {label: "COVID-19", value: covidId}],
      {
        action: "select-option",
        name: "tags",
        option: {"label": "COVID-19", "value": covidId}
      });


    //it is expected that it gets re-rendered with the expected values later
    rerender(
      <form data-testid={'test-form'}>
        <TagsSelect
          placeholder={"placeholderText"}
          label={"Tags"}
          tagIdsSelected={[smallPoxId, sarsId, covidId]}
          onChange={onChangeSpy}
        />
      </form>
    );

    expect(getByTestId('test-form')).toHaveFormValues({tags: [smallPoxId, sarsId, covidId]});
  })
});
