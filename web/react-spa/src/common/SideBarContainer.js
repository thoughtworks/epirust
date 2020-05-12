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

import React, {useState} from "react";
import {faTimes} from "@fortawesome/free-solid-svg-icons";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";

export default function SideBarContainer({children}) {
  const [sidebarOpen, updateSideBarOpen] = useState(false);

  return (
    <div className="drawer-wrap">
      <button onClick={() => updateSideBarOpen(status => !status)} type="button" className="btn btn-primary" aria-label="Close">
        <span>Settings</span>
        <svg className="bi bi-gear-fill" width="1em" height="1em" viewBox="0 0 16 16" fill="currentColor"
             xmlns="http://www.w3.org/2000/svg">
          <path fillRule="evenodd"
                d="M9.405 1.05c-.413-1.4-2.397-1.4-2.81 0l-.1.34a1.464 1.464 0 01-2.105.872l-.31-.17c-1.283-.698-2.686.705-1.987 1.987l.169.311c.446.82.023 1.841-.872 2.105l-.34.1c-1.4.413-1.4 2.397 0 2.81l.34.1a1.464 1.464 0 01.872 2.105l-.17.31c-.698 1.283.705 2.686 1.987 1.987l.311-.169a1.464 1.464 0 012.105.872l.1.34c.413 1.4 2.397 1.4 2.81 0l.1-.34a1.464 1.464 0 012.105-.872l.31.17c1.283.698 2.686-.705 1.987-1.987l-.169-.311a1.464 1.464 0 01.872-2.105l.34-.1c1.4-.413 1.4-2.397 0-2.81l-.34-.1a1.464 1.464 0 01-.872-2.105l.17-.31c.698-1.283-.705-2.686-1.987-1.987l-.311.169a1.464 1.464 0 01-2.105-.872l-.1-.34zM8 10.93a2.929 2.929 0 100-5.86 2.929 2.929 0 000 5.858z"
                clipRule="evenodd"/>
        </svg>
      </button>
      <div className={`drawer ${sidebarOpen && "open"}`}>
        <FontAwesomeIcon onClick={() => updateSideBarOpen(status => !status)} icon={faTimes}/>
        {children}
      </div>
    </div>
  );
}
