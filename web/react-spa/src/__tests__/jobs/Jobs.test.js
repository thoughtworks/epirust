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
import {Jobs} from "../../jobs/Jobs";
import React from "react";

describe('Jobs', () => {
  const testJobs = [
    {_id: "12344", status: "finished"},
    {_id: "12345", status: "finished", config: {key: "value"}},
    {_id: "12346", status: "finished"}
  ]

  it('should render jobs list and jobs details', () => {
    const renderer = new ShallowRenderer()
    renderer.render(<Jobs jobs={testJobs} activeJob={testJobs[1]}/>)

    expect(renderer.getRenderOutput()).toMatchSnapshot()
  });

  it('should render invalid job message if active job is invalid', () => {
    const renderer = new ShallowRenderer()
    renderer.render(<Jobs jobs={testJobs} activeJob={null}/>)

    expect(renderer.getRenderOutput()).toMatchSnapshot()
  });
});
