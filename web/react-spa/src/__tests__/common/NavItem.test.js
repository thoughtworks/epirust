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

import {NavItem} from "../../common/NavItem";
import React from "react";
import {render} from "@testing-library/react";
import {MemoryRouter} from "react-router-dom";

describe('NavItem', function () {
  it('should render activated', function () {
    const {container} = render(<MemoryRouter initialEntries={['/test/']}><NavItem name="Test" linksTo="/test"/></MemoryRouter>);

    expect(container).toMatchSnapshot();
  });

  it('should not render activated', function () {
    const {container} = render(<MemoryRouter initialEntries={['/abc/']}><NavItem name="Test" linksTo="/test"/></MemoryRouter>);

    expect(container).toMatchSnapshot();
  });
});