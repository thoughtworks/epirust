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
const {Grid, CitizenState} = require("../../../db/models/Grid")
const {mockObjectId} = require('../../helpers');

describe('Grid Service', function () {
    beforeAll(async () => await dbHandler.connect());
    afterEach(async () => await dbHandler.clearDatabase());
    afterAll(async () => await dbHandler.closeDatabase());

    describe('saveGridLayout', function () {
        it('should save grid layout', async function () {
            const testSimulationId = mockObjectId();
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

            const gridLayoutInDb = (await Grid.findOne({}, {_id: 0, __v:0}).exec()).toObject();

            expect(gridLayoutInDb).toEqual(gridLayoutToCreate)

        });
    });

    describe('saveCitizenState', function () {
        it('should save a new citizen state if it does not exist in db', async function () {
            const testSimulationId = mockObjectId();
            const citizenStateToCreate = {
                simulation_id: testSimulationId,
                hr: 2,
                citizen_states: [{
                    citizen_id: "123",
                    state: 's',
                    location: {
                        x: 3,
                        y: 4
                    }
                }]
            }
            await GridService.saveCitizenState({...citizenStateToCreate})

            const citizenState = (await CitizenState.findOne({}, {_id: 0, __v:0}).exec()).toObject();

            expect(citizenState).toEqual(citizenStateToCreate)

        });

        it('should update citizen state if it already exist in db', async function () {
            const testSimulationId = mockObjectId();
            await new CitizenState({simulation_id: testSimulationId, hr: 2}).save()

            const citizenStateToCreate = {
                simulation_id: testSimulationId,
                hr: 2,
                citizen_states: [{
                    citizen_id: "123",
                    state: 's',
                    location: {
                        x: 3,
                        y: 4
                    }
                }]
            }
            await GridService.saveCitizenState({...citizenStateToCreate})

            const citizenStateInDb = (await CitizenState.findOne({}, {_id: 0, __v:0}).exec()).toObject();
            const numberOfCitizenStateInDb = await CitizenState.countDocuments().exec();

            expect(citizenStateInDb).toEqual(citizenStateToCreate)
            expect(numberOfCitizenStateInDb).toEqual(1)
        });
    });

    describe('findSortedById', function () {
        it('should sort documents by id and skip by 1 document', async function () {
            const simulationId = mockObjectId();
            await new Grid({simulation_id: simulationId}).save()
            await new Grid({simulation_id: simulationId, grid_size: 3}).save()

            const cursor = GridService.findSortedById(simulationId, 1);

            let documents = []
            for await (const doc of cursor) {
                documents = documents.concat(doc.toObject())
            }
            expect(documents).toHaveLength(1);
            expect(documents[0]).toEqual({
                simulation_id: simulationId,
                grid_size: 3,
                houses: [],
                offices: []
            });
        });
    });
});