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

const {GridMessageHandler} = require('../../services/GridMessageHandler');
jest.mock("../../services/kafka");
jest.mock("../../db/services/SimulationService");
jest.mock("../../db/services/GridService");
const SimulationService = require('../../db/services/SimulationService');
const GridService = require('../../db/services/GridService');
const {mockObjectId} = require('../helpers');

describe('Simulation Grid Consumer', () => {
    it('should store simulation ended when the messages arrives', async () => {
        const simulationId = mockObjectId();
        const message = {value: '{"simulation_ended":true}', key: simulationId};
        const simulationConsumerGrid = new GridMessageHandler();

        await simulationConsumerGrid.handleMessage(message)

        expect(SimulationService.markGridConsumptionFinished).toHaveBeenCalledTimes(1);
        expect(SimulationService.markGridConsumptionFinished).toHaveBeenCalledWith(simulationId);
    });

    it('should store grid layout when grid layout message is sent', async () => {
        const simulationId = mockObjectId();
        const message = {value: '{"grid_size":2}', key: simulationId};
        const simulationConsumerGrid = new GridMessageHandler();

        await simulationConsumerGrid.handleMessage(message);

        expect(GridService.saveGridLayout).toHaveBeenCalledTimes(1);
        expect(GridService.saveGridLayout).toHaveBeenCalledWith({"grid_size": 2, "simulation_id": simulationId});
    });

    it('should store citizen state when citizen state message is sent', async () => {
        const simulationId = mockObjectId();
        const message = {value: '{"dummy_key":1, "hr":1}', key: simulationId};
        const simulationConsumerGrid = new GridMessageHandler();

        await simulationConsumerGrid.handleMessage(message);

        expect(GridService.saveCitizenState).toHaveBeenCalledTimes(1);
        expect(GridService.saveCitizenState).toHaveBeenCalledWith({dummy_key: 1, hr: 1, simulation_id: simulationId});
    });
});
