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

const {SimulationCountsConsumer} = require('../../services/SimulationCountsConsumer');
jest.mock("../../services/kafka");
jest.mock("../../db/models/Simulation");
jest.mock("../../db/models/Count");
const {KafkaGroupConsumer} = require('../../services/kafka');
const {Simulation} = require('../../db/models/Simulation');
const Count = require('../../db/models/Count');

describe('Simulation Counts Consumer', () => {
    beforeEach(() => {
        jest.clearAllMocks()
    });

    it('should initiate the kafka group consumer with with correct config', () => {
        new SimulationCountsConsumer();
        let KafkaGroupConsumerConfig = ['localhost:9092', 'counts_updated', 'dev_server_consumer'];

        expect(KafkaGroupConsumer).toHaveBeenCalledTimes(1);
        expect(KafkaGroupConsumer.mock.calls[0]).toEqual(KafkaGroupConsumerConfig);
    });

    it('should updated the status of simulation to be finished', async () => {
        KafkaGroupConsumer.mockReturnValueOnce({consumerStream: [{value: '{"simulation_ended":true}', key: "123"}]});
        const execMock = jest.fn();
        Simulation.update.mockReturnValueOnce({exec: execMock});
        const simulationCountsConsumer = new SimulationCountsConsumer();

        await simulationCountsConsumer.start();

        expect(Simulation.update).toHaveBeenCalledTimes(1);
        expect(Simulation.update.mock.calls[0]).toEqual([
            {simulation_id: 123},
            {status: "finished", simulation_id: 123},
            {"upsert": true},
        ]);
        expect(execMock).toHaveBeenCalledTimes(1);
        expect(execMock.mock.calls[0]).toEqual([])
    });

    it('should store counts if not ended message', async () => {
        KafkaGroupConsumer.mockReturnValueOnce({consumerStream: [{value: '{"dummyKey":"dummyValue"}', key: "123"}]});
        let saveMock = jest.fn();
        Count.mockReturnValueOnce({save: saveMock});
        const simulationCountsConsumer = new SimulationCountsConsumer();

        await simulationCountsConsumer.start();

        expect(Count).toHaveBeenCalledTimes(1);
        expect(Count.mock.calls[0]).toEqual([{dummyKey: 'dummyValue', "simulation_id": 123}]);
        expect(saveMock).toHaveBeenCalledTimes(1);
        expect(saveMock.mock.calls[0]).toEqual([])
    });
});



