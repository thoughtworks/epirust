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

import {Link, useRouteMatch} from "react-router-dom";
import React from "react";

export function NavItem({ name, linksTo, activeOnExactMatch = false }) {
  const match = useRouteMatch({
    path: linksTo,
    exact: activeOnExactMatch
  });

  return (
    <li className={`nav-item`}>
      <Link className={`nav-link ${match ? "active" : ""}`} to={linksTo}>{name}</Link>
    </li>
  );
}