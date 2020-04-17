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
const dbHandler = require("../db-handler")
const GridService = require("../../../db/services/GridService")
const {Grid} = require("../../../db/models/Grid")

describe('Grid Service', function () {
    beforeAll(async () => await dbHandler.connect());
    afterEach(async () => await dbHandler.clearDatabase());
    afterAll(async () => await dbHandler.closeDatabase());

    describe('saveGridLayout', function () {
        it('should save grid layout', async function () {
            let testSimulationId = 123;
            const gridLayoutToCreate = {
                simulation_id: testSimulationId,

                grid_size: 4,
                housing_area: {
                    start_offset: {x: 1, y: 1},
                    end_offset: {x: 2, y: 2},
                    iter_index: {x: 3, y: 3}
                },
                work_area: {
                    start_offset: {x: 4, y: 4},
                    end_offset: {x: 5, y: 5},
                    iter_index: {x: 6, y: 6}
                },
                transport_area: {
                    start_offset: {x: 7, y: 7},
                    end_offset: {x: 8, y: 8},
                    iter_index: {x: 9, y: 9}
                },
                hospital_area: {
                    start_offset: {x: 7, y: 7},
                    end_offset: {x: 8, y: 8},
                    iter_index: {x: 9, y: 9}
                },
                houses: [{
                    start_offset: {x: 7, y: 7},
                    end_offset: {x: 8, y: 8},
                    iter_index: {x: 9, y: 9}
                }],
                offices: [{
                    start_offset: {x: 7, y: 7},
                    end_offset: {x: 8, y: 8},
                    iter_index: {x: 9, y: 9}
                }]
            }

            await GridService.saveGridLayout(gridLayoutToCreate)

            const gridLayoutInDb = (await Grid.findOne({simulation_id: testSimulationId}, {_id: 0, __v:0}).exec()).toObject();

            expect(gridLayoutInDb).toEqual(gridLayoutToCreate)

        });
    });
});