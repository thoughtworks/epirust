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
import ShallowRenderer from 'react-test-renderer/shallow';
import {ComparerConfig} from "../../jobsCompare/ComparerConfig";
import React from "react";

describe('Comparer Config', () => {
  it('should render Select for time-series filer and form for selecting interventions', () => {
    const renderer = new ShallowRenderer()
    const selectedJobs = {
      job1: {simulations: [{_id:"4567uhg"}, {_id: "4567uyhghjn"}]},
      job2: {simulations: [{_id:"456776tgh"}]}
    }
    renderer.render(<ComparerConfig selectedJobs={selectedJobs}/>)

    expect(renderer.getRenderOutput()).toMatchSnapshot()
  });
});
