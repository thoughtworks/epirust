/*!
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

import MiscellaneousConfig from "../../simulation-configuration/MiscellaneousConfig";
import React from "react";
import {fireEvent, render} from "@testing-library/react";

describe('MiscellaneousConfig', function () {
  it('should have grid-data enabler input switch', function () {
    const {getByLabelText} = render(<MiscellaneousConfig/>);

    expect(getByLabelText("Grid Visualization")).toBeInTheDocument();
  });

  it('should disable grid-data input when grid size is greater than 100', function () {
    const {getByLabelText} = render(<MiscellaneousConfig/>);

    fireEvent.change(getByLabelText("Grid Size"), { target: { value: 200 } });

    expect(getByLabelText("Grid Visualization").disabled).toBe(true);

  });

  it('should uncheck grid-data input when grid size is greater than 100', function () {
    const {getByLabelText} = render(<MiscellaneousConfig/>);

    fireEvent.change(getByLabelText("Grid Size"), { target: { value: 100 } });
    fireEvent.change(getByLabelText("Grid Visualization"), { target: { checked: true } });
    expect(getByLabelText("Grid Visualization").checked).toBe(true);

    fireEvent.change(getByLabelText("Grid Size"), { target: { value: 200 } });
    expect(getByLabelText("Grid Visualization").checked).toBe(false);
    expect(getByLabelText("Grid Visualization").disabled).toBe(true);
  });

  it('should not disable grid-data input field when grid-size is less than 101 and number of simulation is 1', function () {
    const {getByLabelText} = render(<MiscellaneousConfig/>);

    fireEvent.change(getByLabelText("Grid Size"), { target: { value: 10 } });
    fireEvent.change(getByLabelText("Number of simulations"), { target: { value: 1 } });

    expect(getByLabelText("Grid Visualization").disabled).toBe(false);
  });

  it('should disable grid-data input when number of simulations is greater than 1', function () {
    const {getByLabelText} = render(<MiscellaneousConfig/>);

    fireEvent.change(getByLabelText("Grid Size"), { target: { value: 100 } });
    fireEvent.change(getByLabelText("Number of simulations"), { target: { value: 2 } });

    expect(getByLabelText("Grid Visualization").disabled).toBe(true);

  });

  it('should uncheck grid-data input when number of simulations is greater than 100', function () {
    const {getByLabelText} = render(<MiscellaneousConfig/>);

    fireEvent.change(getByLabelText("Grid Size"), { target: { value: 100 } });
    fireEvent.change(getByLabelText("Grid Visualization"), { target: { checked: true } });
    expect(getByLabelText("Grid Visualization").checked).toBe(true);

    fireEvent.change(getByLabelText("Number of simulations"), { target: { value: 2 } });
    expect(getByLabelText("Grid Visualization").checked).toBe(false);
    expect(getByLabelText("Grid Visualization").disabled).toBe(true);
  });
});