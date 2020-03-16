let socket = null;
let renderResult = null;

function handleSubmitData(e) {
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

function handleResetSimulation(e) {
    socket && socket.close()
    renderResult.remove('table', function (d) { return true; }).run();
}

function startSocket() {
    // connect to simple echo server
    if (socket) {
        socket.close();
    }

    socket = io('http://localhost:3000/');

    let dataBuffer = [];
    let g;

    socket.on('epidemicStats', function (messageRaw) {
        const message = JSON.parse(messageRaw);
        console.log(message.hour);
        dataBuffer.push([message.hour, message.susceptible, message.infected, message.quarantined, message.recovered,
            message.deceased]);

        if (message.hour % 100 === 0) {
            if (g) {
                g.updateOptions( { 'file': dataBuffer } );
            } else {
                g = new Dygraph(document.getElementById("vis"), dataBuffer, {
                    labels: ["hour", "susceptible", "infected", "quarantined", "recovered", "deceased"]
                });
            }
        }

    });

}
