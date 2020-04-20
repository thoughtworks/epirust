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

const {Grid, CitizenState} = require("../models/Grid")

async function saveGridLayout(gridLayout) {
    const grid = new Grid(gridLayout);
    await grid.save();
}

async function saveCitizenState(citizenState) {
    let query = {simulation_id: citizenState.simulation_id, hr: citizenState.hr};
    const updateQuery = CitizenState.updateOne(query, citizenState, {upsert: true});
    await updateQuery.exec()
}

function findSortedById(simulationId , skipBy) {
    const query = { simulation_id: simulationId };
    return Grid.find(query, {_id: 0, __v: 0})
        .sort({_id: 1})
        .skip(skipBy)
        .cursor();
}

module.exports = {saveGridLayout, saveCitizenState, findSortedById};
