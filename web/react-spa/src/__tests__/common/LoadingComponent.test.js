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
import {LoadingComponent} from "../../common/LoadingComponent";
import {LOADING_STATES} from "../../common/constants";
import React from "react";
import ShallowRenderer from 'react-test-renderer/shallow';


describe('LoadingComponent', () => {
  it('should render loader if the state is loading', () => {
    const renderer = new ShallowRenderer()
    renderer.render(<LoadingComponent loadingState={LOADING_STATES.LOADING}>
      <div>Test child component</div>
    </LoadingComponent>)

    expect(renderer.getRenderOutput()).toMatchSnapshot()
  });

  it('should render error message if the state is failed', () => {
    const renderer = new ShallowRenderer()
    renderer.render(<LoadingComponent loadingState={LOADING_STATES.FAILED}>
      <div>Test child component</div>
    </LoadingComponent>)

    expect(renderer.getRenderOutput()).toMatchSnapshot()
  });

  it('should render child component if the state is finished', () => {
    const renderer = new ShallowRenderer()
    renderer.render(<LoadingComponent loadingState={LOADING_STATES.FINISHED}>
      <div>Test child component</div>
    </LoadingComponent>)

    expect(renderer.getRenderOutput()).toMatchSnapshot()
  });
});
