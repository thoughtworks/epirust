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

    pushData(paramsData)
        .then(startSimulation)
        .then(startSocket)
}

function pushData(paramsData) {
    return fetch("http://localhost:3000/simulation/init", {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(paramsData)
    });
}

function startSimulation() {
    return fetch("http://localhost:3000/simulation", {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        }
    });
}

function startSocket() {
    if (socket) {
        socket.close();
    }

    socket = io('http://localhost:3000/');

    let dataBuffer = [];
    let graph;

    socket.on('epidemicStats', function (messageRaw) {
        const message = JSON.parse(messageRaw);
        console.log(message.hour);

        const { hour, susceptible, infected, quarantined, recovered, deceased } = message;

        dataBuffer.push([hour, susceptible, infected, quarantined, recovered, deceased]);

        if (message.hour % 100 === 0) {

            if (!graph) {
                graph = new Dygraph(document.getElementById("vis"), dataBuffer, {
                    labels: ["hour", "susceptible", "infected", "quarantined", "recovered", "deceased"]
                });
                return;
            }
            graph.updateOptions({ 'file': dataBuffer });
        }
    });
}
