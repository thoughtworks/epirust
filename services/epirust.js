const execa = require('execa');

module.exports = class EpirustService{

    start(numberOfAgents) {
        this.engine = './external/epirust';
        (async () => {
            try {
                const {stdout} = await execa(this.engine, [numberOfAgents]);
                console.log(stdout);
            } catch (error) {
                throw new Error("Failed spawning epirust engine - " + error);
            }
        })();
    }
};