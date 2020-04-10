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


import SocketAwareGraph from "./SocketAwareGraph";
import RestfulGraph from "./RestfulGraph";
import React, { useEffect, useState } from "react";
import io from "socket.io-client";
import config from "../config";
import { transformTimeSeriesMessages, transformTimeSeriesDeviationMessages } from './utils'

export function TimeSeries({ simulationId }) {
  const [socket, setSocket] = useState(null);

  useEffect(() => {
    const socketInstance = io(`${config.API_HOST}/${'counts'}`);
    setSocket(socketInstance);

    return () => {
      socketInstance.close();
    }
  }, []);
  return (
    <SocketAwareGraph
      simulationId={simulationId}
      socket={socket}
      transformFn={transformTimeSeriesMessages}
    />
  )
}

export function TimeSeriesDeviation({ simulationId }) {
  return (
    <RestfulGraph
      simulationId={simulationId}
      apiPath={`${config.API_HOST}/simulation/${simulationId}/time-series-deviation`}
      transformFn={transformTimeSeriesDeviationMessages}
    />
  )
}