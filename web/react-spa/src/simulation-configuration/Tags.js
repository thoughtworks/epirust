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

import React from "react";
import Select from "react-select"
import {predefinedTags} from "./predefined-tags"

export default function Tags() {
  function transform() {
    return predefinedTags.map(tag => {
      return {value: tag.id, label: tag.name}
    })
  }

  return (
    <div className="input-control tags-wrap">
      <label className="font-weight-bold col-form-label-sm" htmlFor="tags">Tags</label>
      <Select options={transform()} isMulti name="tags" data-testid="tags" id="tags" aria-label="tags"/>
    </div>
  )
}
