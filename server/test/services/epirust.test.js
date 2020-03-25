jest.mock('execa');

const execa = require('execa');

const epirust = require('../../services/epirust');

describe('epirust', () => {
    it('should exec epirust engine', () => {
        const epirustService = new epirust();

        epirustService.start(100);

        expect(execa.mock.calls.length).toEqual(1);
        expect(execa.mock.calls[0]).toEqual(["./external/epirust", [100]]);
        expect(epirustService.engine).toEqual("./external/epirust");
    });
});