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

import React from 'react'
import {render} from '@testing-library/react'
import GridLegend from "../../grid/GridLegend";

describe('Grid Legend', function () {
    it('should render counts for each state', function () {
        const {container} = render(<GridLegend susceptible={12} exposed={1} infected={13} recovered={14} deceased={15}/>);

        const counts = container.querySelectorAll('.agents .count');
        expect(counts).toHaveLength(5);
        expect(counts[0].textContent).toEqual("12");
        expect(counts[1].textContent).toEqual("1");
        expect(counts[2].textContent).toEqual("13");
        expect(counts[3].textContent).toEqual("14");
        expect(counts[4].textContent).toEqual("15");
    });
});

