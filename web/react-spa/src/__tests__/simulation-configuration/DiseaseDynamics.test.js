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

import DiseaseDynamics from '../../simulation-configuration/DiseaseDynamics'
import { render, fireEvent } from '@testing-library/react'
import React from 'react'

test('should update disease dynamics for a disease selected', () => {
    const { getByPlaceholderText, getByTestId } = render(<DiseaseDynamics />);

    fireEvent.change(getByTestId("select-disease"), {target: {value: "sars"}});

    expect(getByPlaceholderText("Regular Transmission Start Day").value).toBe("5");
    expect(getByPlaceholderText("High Transmission Start Day").value).toBe("10");
    expect(getByPlaceholderText("Last Day").value).toBe("23");
    expect(getByPlaceholderText("Asymptomatic Last Day").parentElement).toHaveClass("d-none");
    expect(getByPlaceholderText("Mild Infected Last Day").parentElement).toHaveClass("d-none");
    expect(getByPlaceholderText("Regular Transmission Rate").value).toBe("0.025");
    expect(getByPlaceholderText("High Transmission Rate").value).toBe("0.25");
    expect(getByPlaceholderText("Death Rate").value).toBe("0.1");
});

test('should display last day variants for covid', () => {
    const { getByPlaceholderText, getByTestId } = render(<DiseaseDynamics />);

    fireEvent.change(getByTestId("select-disease"), {target: {value: "covid_19"}});

    expect(getByPlaceholderText("Asymptomatic Last Day").parentElement).not.toHaveClass("d-none");
    expect(getByPlaceholderText("Asymptomatic Last Day").value).toBe("9");
    expect(getByPlaceholderText("Mild Infected Last Day").parentElement).not.toHaveClass("d-none");
    expect(getByPlaceholderText("Mild Infected Last Day").value).toBe("12");

});
