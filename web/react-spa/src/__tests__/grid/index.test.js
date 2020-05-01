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

import React from 'react'
import {act, render} from '@testing-library/react'
import GridPage from '../../grid/index'
import MockSocket from 'socket.io-mock'
import io from 'socket.io-client'

jest.mock('socket.io-client');

describe('Grid Page', () => {
    let mockSocket, closeSpy;
    beforeEach(() => {
        mockSocket = new MockSocket();
        io.mockImplementation(() => mockSocket);
        closeSpy = jest.fn();
        mockSocket.close = closeSpy;
    });

    it('should render GridPage component with Loading', () => {
        const { asFragment } = render(<GridPage jobId={"1542319876"} />);
        expect(asFragment()).toMatchSnapshot()
    });

    it('should render GridPage component consuming the data coming from socket', () => {
        const layoutDimensions = {
            "grid_size": 10,
            "housing_area": {
                "start_offset": {
                    "x": 0,
                    "y": 0
                },
                "end_offset": {
                    "x": 4,
                    "y": 10
                }
            },
            "work_area": {
                "start_offset": {
                    "x": 5,
                    "y": 0
                },
                "end_offset": {
                    "x": 7,
                    "y": 10
                }
            },
            "transport_area": {
                "start_offset": {
                    "x": 4,
                    "y": 0
                },
                "end_offset": {
                    "x": 5,
                    "y": 10
                }
            },
            "hospital_area": {
                "start_offset": {
                    "x": 7,
                    "y": 0
                },
                "end_offset": {
                    "x": 8,
                    "y": 10
                }
            },
            "houses": [
                {
                    "start_offset": {
                        "x": 0,
                        "y": 0
                    },
                    "end_offset": {
                        "x": 2,
                        "y": 2
                    },
                },
                {
                    "start_offset": {
                        "x": 2,
                        "y": 0
                    },
                    "end_offset": {
                        "x": 4,
                        "y": 2
                    },
                }],
            "offices": [
                {
                    "start_offset": {
                        "x": 5,
                        "y": 0
                    },
                    "end_offset": {
                        "x": 6,
                        "y": 1
                    },
                },
                {
                    "start_offset": {
                        "x": 6,
                        "y": 0
                    },
                    "end_offset": {
                        "x": 7,
                        "y": 1
                    },
                }]
        };

        const agentPositionMessage = {
            "hr": 1,
            "citizen_states": [
                { "id": 595, "state": "s", "location": { "x": 0, "y": 0 } },
                { "id": 238, "state": "s", "location": { "x": 37, "y": 66 } },
                { "id": 981, "state": "s", "location": { "x": 31, "y": 1 } }]
        };

        let emitSpy = jest.fn();

        act(() => {
            jest.mock('socket.io-client', () => { });
            mockSocket.emit = emitSpy;
        });

        const { asFragment } = render(<GridPage jobId={"1542319876"}/>);

        act(() => {
            mockSocket.socketClient.emit("gridData", layoutDimensions);
            mockSocket.socketClient.emit("gridData", agentPositionMessage);
        });

        expect(closeSpy).toHaveBeenCalledTimes(0);

        act(() => {
            mockSocket.socketClient.emit("gridData", { "simulation_ended": true });
        });

        expect(asFragment()).toMatchSnapshot();
        expect(emitSpy).toHaveBeenCalledWith("get", {jobId: "1542319876"});
        expect(closeSpy).toHaveBeenCalled()
    });

    it('should close the socket before unmounting the component', () => {
        render(<GridPage jobId={"1542319876"} />).unmount();

        expect(closeSpy).toHaveBeenCalledTimes(1);
        expect(closeSpy.mock.calls[0]).toHaveLength(0)
    });
});