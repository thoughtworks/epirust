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
import Tags from "../../jobs/Tags";
import {render} from "@testing-library/react";

describe('Tags', () => {

  test('should render tags associated with the job', async () => {
    let tagId = "111", tagName = "tagName";
    let tagId1 = "112", tagName1 = "tagName";

    const {getByTestId} = render(
      <Tags tags={[
        {id: tagId, name: tagName},
        {id: tagId1, name: tagName1}
      ]}
      />);

    expect(getByTestId(`tag-${tagId}`)).toHaveTextContent(tagName);
    expect(getByTestId(`tag-${tagId1}`)).toHaveTextContent(tagName1)
  });


  test('should render tags as small', async () => {
    let tagId = "111", tagName = "tagName";

    const {getByTestId} = render(
      <Tags
        isSmall
        tags={[
          {id: tagId, name: tagName}
        ]}
      />);

    expect(getByTestId(`tags`)).toHaveClass('small');
  });
});
