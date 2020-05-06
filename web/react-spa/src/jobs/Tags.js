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

import React from "react";
import PropTypes from "prop-types";

export default function Tags({tags = [], isSmall = false}) {
  const classes = ['tags', ...(isSmall ? ['small'] : [])].join(' ');
  return (
    <ul data-testid="tags" className={classes}>
      {tags.map(tag =>
        <li key={tag.id} data-testid={`tag-${tag.id}`}>{tag.name}</li>
      )}
    </ul>
  );
}

const TagType = PropTypes.shape({id: PropTypes.string, name: PropTypes.string});
Tags.propTypes = {
  tags: PropTypes.arrayOf(TagType).isRequired,
  isSmall: PropTypes.bool
};
