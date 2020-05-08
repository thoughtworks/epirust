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
import PropTypes from "prop-types";

function modelTags(tags) {
  return tags.map(tag => {
    return {value: tag.id, label: tag.name}
  })
}

export default function TagsSelect({label, placeholder, onChange, tagIdsSelected = []}) {

  const [tags, setTags] = useState([]);
  const [isLoading, setIsLoading] = useState(false);

  useEffect(() => {
    setIsLoading(true);

    get('/jobs/tags')
      .then(res => res.json())
      .then(setTags)
      .then(() => setIsLoading(false))
  }, []);

  const tagIdToTag = tags.reduce((acc, tag) => {
    return {...acc, [tag.id]: tag};
  }, {});

  const valuesSelected = (!tags.length || !tagIdsSelected.length)
    ? []
    : tagIdsSelected.map(id => ({
      value: tagIdToTag[id].id,
      label: tagIdToTag[id].name
    }));

  return (
    <div className="input-control tags-wrap">
      <label className="font-weight-bold col-form-label-sm" htmlFor="tag-input">{label}</label>
      <Select
        {...(!valuesSelected.length ? {} : {value: valuesSelected})}
        options={modelTags(tags)}
        isLoading={isLoading}
        isMulti
        placeholder={placeholder}
        inputId="tag-input"
        name="tags"
        aria-label="tags"
        onChange={onChange}
        styles={{
          placeholder: (provided) => ({
            ...provided,
            fontSize: '14px'
          }),
          input: (provided) => ({
            ...provided,
            fontSize: '14px',
            margin: 0
          }),
          control: (provided) => ({
            ...provided,
            fontSize: '14px'
          }),
          option: (provided) => ({
            ...provided,
            padding: "6px 12px"
          })
        }}
      />
    </div>
  )
}

TagsSelect.propTypes = {
  label: PropTypes.string,
  placeholder: PropTypes.string,
  onChange: PropTypes.func,
};
