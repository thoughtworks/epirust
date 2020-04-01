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


import Graph from "./LineGraph";
import SocketAwareGraph from "./SocketAwareGraph";
import React, {useState} from "react";
import io from "socket.io-client";
import FileDataInput from "./FileDataInput";

function TimeSeries() {
  const [socket, setSocket] = useState(null);
  const [fileData, setFileData] = useState([]);

  function startSocket() {
    if (socket) {
      socket.close();
      setSocket(null);
    }

    setSocket(io('http://localhost:3000/counts'));

  }

  function handleFileData(data) {
    if (socket) {
      socket.close();
      setSocket(null);
    }
    setFileData(data)
  }

  return (
    <>
      <FileDataInput onFileDataSubmit={handleFileData} />
      {fileData.length > 1 ? <Graph dataBuffer={fileData}/> : <SocketAwareGraph socket={socket}/>}
      <button className="btn btn-primary" onClick={startSocket}>Show Time Series</button>
    </>
  )
}

export default TimeSeries;