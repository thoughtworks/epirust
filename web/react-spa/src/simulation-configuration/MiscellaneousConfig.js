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

export default function MiscellaneousConfig() {
  const disableGrid = (gs) => gs > 100;
  const handleSizeChange = (e) => updateGridSize(e.target.value);

  const [gridSize, updateGridSize] = useState(250);
  const [gridDisable, updateGridDisabled] = useState(disableGrid(gridSize));
  const [gridChecked, updateGridChecked] = useState(false);

  const handleCheckedChange = (e) => updateGridChecked(!gridChecked);

  useEffect(() => {
    updateGridDisabled(disableGrid(gridSize));

    if(gridSize > 100)
      updateGridChecked(false);
  }, [gridSize]);

  return (
    <fieldset>
      <div className="input-control">
        <label className="col-form-label-sm" htmlFor="grid_size">Grid Size</label>
        <input onChange={handleSizeChange} type="number" name="grid_size" className="form-control form-control-sm" id="grid_size" aria-describedby="grid_size" placeholder="Grid Size" defaultValue={gridSize} />
      </div>


      <div className="input-control">
        <label className="col-form-label-sm" htmlFor="simulation_hrs">Simulation Hours</label>
        <input type="number" name="simulation_hrs" className="form-control form-control-sm" id="simulation_hrs" aria-describedby="simulation_hrs" placeholder="Simulation Hours" defaultValue="10000" />
      </div>

      <div className="custom-control custom-switch switch-right">
        <input type="checkbox" name="generate_grid_vis" onChange={handleCheckedChange} checked={gridChecked} disabled={gridDisable} className="custom-control-input" id="generate_grid_vis" />
        <label className="custom-control-label col-form-label-sm" htmlFor="generate_grid_vis">Grid Visualization</label>
      </div>
      <small className="form-text text-muted">Allowed when grid size less than 101 </small>
    </fieldset>
  )
}
