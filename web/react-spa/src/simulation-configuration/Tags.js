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

import React, {useEffect, useState} from "react";
import Select from "react-select"
import {predefinedTags} from "./predefined-tags"
import {get} from "../common/apiCall";

function modelTags(tags) {
  return tags.map(tag => {
    return {value: tag.id, label: tag.name}
  })
}

export default function Tags() {

  const [tags, setTags] = useState([]);
  const [isLoading, setIsLoading] = useState(false);

  useEffect(() => {
    setIsLoading(true);

    get('/jobs/tags')
      .then(res => res.json())
      .then(setTags)
      .then(() => setIsLoading(false))
  }, []);

  return (
    <div className="input-control tags-wrap">
      <label className="font-weight-bold col-form-label-sm" htmlFor="tag-input">Tags</label>
      <Select
        options={modelTags(tags)}
        isLoading={isLoading}
        isMulti
        inputId="tag-input"
        name="tags"
        aria-label="tags"
      />
    </div>
  )
}
