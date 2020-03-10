import React, { useState } from 'react'
import { VegaLite } from 'react-vega'
import io from 'socket.io-client';

const spec = {
    $schema: 'https://vega.github.io/schema/vega-lite/v4.json',
    width: 400,
    mark: 'line',
    data: { name: 'table' },
    encoding: {
        x: { field: 'hour', type: 'quantitative', scale: { zero: false } },
        y: { field: 'agents', type: 'quantitative' },
        category: { field: 'category', type: 'nominal' }
    }
};

export default function VegaLine() {

    const [data, setData] = useState({
        table: [
            {
                hour: 1,
                agents: 5,
                category: 'susceptible'
            },
            {
                hour: 2,
                agents: 6,
                category: 'infected'
            },
            {
                hour: 3,
                agents: 15,
                category: 'quarantined'
            },
            {
                hour: 4,
                agents: 25,
                category: 'recovered'
            },
            {
                hour: 5,
                agents: 50,
                category: 'deceased'
            },
        ]
    })
    const [socket, setSocket] = useState(null)

    const stopSimulation = function (e) {
        e.preventDefault();
        socket.close();
    }

    const startSimulation = function (e) {
        e.preventDefault();
        const paramsData = {}
        new FormData(e.target).forEach(function (value, key) {
            paramsData[key] = value;
        });

        // fetch("http://localhost:3000/simulation", {
        //     method: 'POST',
        //     headers: {
        //         'Content-Type': 'application/json'
        //     },
        //     body: JSON.stringify(paramsData)
        // }).then((res) => {
        //     setSocket(io('http://localhost:3000/'));
        // })

    }

    socket && socket.on('epidemicStats', function (messageRaw) {
        const message = JSON.parse(messageRaw);
        console.log(message.hour);

        let susceptibleData = {
            hour: message.hour,
            agents: message.susceptible,
            category: 'susceptible'
        }
        let infectedData = {
            hour: message.hour,
            agents: message.infected,
            category: 'infected'
        }
        let quarantinedData = {
            hour: message.hour,
            agents: message.quarantined,
            category: 'quarantined'
        }
        let recoveredData = {
            hour: message.hour,
            agents: message.recovered,
            category: 'recovered'
        }
        let deceasedData = {
            hour: message.hour,
            agents: message.deceased,
            category: 'deceased'
        }

        setData({ ...data.name, values: [...data.values, susceptibleData, infectedData, quarantinedData, recoveredData, deceasedData] });
    });

    return (
        <>
            <VegaLite spec={spec} data={data} />

            <form onSubmit={startSimulation}>
                <div className="form-row">
                    <div className="col">
                        <input type="number" name="numberOfAgents" className="form-control" id="numberOfAgents" aria-describedby="numberOfAgents" placeholder="Number of Agents" defaultValue="10000" />
                    </div>
                    <div className="col">
                        <button type="submit" className="btn btn-primary">Start</button>
                        <button type="button" className="btn btn-danger" onClick={stopSimulation}>Stop</button>
                    </div>
                </div>
            </form>
        </>
    )
}