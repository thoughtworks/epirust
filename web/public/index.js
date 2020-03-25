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

const spec = {
    $schema: 'https://vega.github.io/schema/vega-lite/v4.json',
    data: { name: 'table' },
    autosize: {
        "resize": true
    },
    width: window.innerWidth - 200,
    height: window.innerHeight - 200,
    encoding: { "x": { "field": "hour", "type": "quantitative" } },
    layer: [
        {
            encoding: {
                color: { field: 'category', type: 'nominal' },
                y: { field: 'agents', type: 'quantitative' }
            },
            layer: [
                { "mark": "line" },
                { "transform": [{ "filter": { "selection": "hover" } }], "mark": "point" }
            ]
        },
        {
            "transform": [{ "pivot": "category", "value": "agents", "groupby": ["hour"] }],
            "mark": "rule",
            "encoding": {
                "opacity": {
                    "condition": { "value": 0.3, "selection": "hover" },
                    "value": 0
                },
                "tooltip": [
                    { "field": "susceptible", "type": "quantitative" },
                    { "field": "infected", "type": "quantitative" },
                    { "field": "quarantined", "type": "quantitative" },
                    { "field": "recovered", "type": "quantitative" },
                    { "field": "deceased", "type": "quantitative" }
                ]
            },
            "selection": {
                "hover": {
                    "type": "single",
                    "fields": ["hour"],
                    "nearest": true,
                    "on": "mouseover",
                    "empty": "none",
                    "clear": "mouseout"
                }
            }
        }
    ]
};

let socket = null;
let renderResult = null;

vegaEmbed('#vis', spec, { defaultStyle: true })
    .then(function (result) {
        const view = result.view;
        renderResult = view;
    })
    .catch(console.warn);

function handleSubmitData(e) {
    //Clear laready existing socket and clean graph
    socket && socket.close()
    renderResult && renderResult.remove('table', function (d) { return true; }).run();


    e.preventDefault();
    let paramsData = {}
    new FormData(e.target).forEach(function (value, key) {
        if (["number_of_agents",
            "grid_size",
            "simulation_hrs",
            "public_transport_percentage",
            "working_percentage",
            "vaccinate_at",
            "vaccinate_percentage"].includes(key)) {
            value = Number(value);
        }
        paramsData[key] = value;
    });

    fetch("http://localhost:3000/simulation/init", {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(paramsData)
    })

    fetch("http://localhost:3000/simulation", {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        }
    }).then((res) => {
        startSocket()
    })
}

function startSocket() {
    // connect to simple echo server
    if (socket) {
        socket.close();
    }

    socket = io('http://localhost:3000/')

    let dataBuffer = [];

    socket.on('epidemicStats', function (messageRaw) {
        const message = JSON.parse(messageRaw);
        console.log(message.hour);

        //TODO: remove this duplication
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

        dataBuffer = [...dataBuffer, susceptibleData, infectedData, quarantinedData, recoveredData, deceasedData]
        // Use the Vega view api to insert data

        if (message.hour % 100 === 0) {
            let changeSet = vega.changeset().insert(dataBuffer);
            renderResult.change('table', changeSet).run();
            dataBuffer = [];
        }

    });

}
