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

import renderer from 'react-test-renderer'
import Cell from "../../grid/Cell";
import {Color} from "../../grid/Color";
import React from "react";


describe('Cell', function () {
    it('should render with specified color and size', function () {
        const testColor = new Color(10, 11, 12);
        const component = renderer.create(<Cell color={testColor} size={10} cellId={1}/>);

        const tree = component.toJSON();

        expect(tree).toMatchSnapshot();
    });
});
