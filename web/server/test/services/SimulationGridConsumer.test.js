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

const {SimulationGridConsumer} = require('../../services/SimulationGridConsumer');
jest.mock("../../services/kafka");
jest.mock("../../db/models/Simulation");
jest.mock("../../db/models/Grid");
const {KafkaGroupConsumer} = require('../../services/kafka');
const {Simulation} = require('../../db/models/Simulation');
const {Grid, CitizenState} = require('../../db/models/Grid');

describe('Simulation Grid Consumer', () => {
    beforeEach(() => {
        KafkaGroupConsumer.mockClear();
    });

    it('should initiate the kafka group consumer with with correct config', () => {
        new SimulationGridConsumer();
        let KafkaGroupConsumerConfig = ['localhost:9092', 'citizen_states_updated', 'dev_server_consumer'];

        expect(KafkaGroupConsumer).toHaveBeenCalledTimes(1);
        expect(KafkaGroupConsumer.mock.calls[0]).toEqual(KafkaGroupConsumerConfig);
    });

    it('should store simulation ended when the messages arrives', async () => {
        KafkaGroupConsumer.mockReturnValueOnce({consumerStream: [{value: '{"simulation_ended":true}', key: "123"}]});
        let execMock = jest.fn();
        Simulation.updateOne.mockReturnValueOnce({exec: execMock});
        const simulationConsumerGrid = new SimulationGridConsumer();

        await simulationConsumerGrid.start();

        expect(Simulation.updateOne).toHaveBeenCalledTimes(1);
        expect(Simulation.updateOne.mock.calls[0]).toEqual([
            {simulation_id: 123},
            {"grid_consumption_finished": true, "simulation_id": 123},
            {"upsert": true},
        ]);
        expect(execMock).toHaveBeenCalledTimes(1);
        expect(execMock.mock.calls[0]).toEqual([])
    });

    it('should store grid layout when grid layout message is sent', async () => {
        KafkaGroupConsumer.mockReturnValueOnce({consumerStream: [{value: '{"grid_size":2}', key: "123"}]});
        let saveMock = jest.fn();
        Grid.mockReturnValueOnce({save: saveMock});
        const simulationConsumerGrid = new SimulationGridConsumer();

        await simulationConsumerGrid.start();

        expect(Grid).toHaveBeenCalledTimes(1);
        expect(Grid.mock.calls[0]).toEqual([{"grid_size":2, "simulation_id": 123}]);
        expect(saveMock).toHaveBeenCalledTimes(1);
        expect(saveMock.mock.calls[0]).toEqual([])
    });

    it('should store citizen state when citizen state message is sent', async () => {
        KafkaGroupConsumer.mockReturnValueOnce({consumerStream: [{value: '{"dummy_key":1, "hr":1}', key: "123"}]});
        const execMock = jest.fn();
        CitizenState.updateOne.mockReturnValueOnce({exec: execMock});
        const simulationConsumerGrid = new SimulationGridConsumer();

        await simulationConsumerGrid.start();

        expect(CitizenState.updateOne).toHaveBeenCalledTimes(1);
        expect(CitizenState.updateOne).toHaveBeenCalledWith(
            {hr:1, simulation_id: 123},
            {dummy_key:1, hr:1, simulation_id: 123},
            {upsert: true}
        );
        expect(execMock).toHaveBeenCalledTimes(1);
        expect(execMock.mock.calls[0]).toEqual([])
    });
});



