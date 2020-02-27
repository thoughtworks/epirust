const epirust = require('../../services/epirust');

jest.mock('execa');

describe('epirust', () => {
    test('should exec epirust engine', () => {
        const execa = jest.fn();
        const epirustService = new epirust();

        epirustService.start(100);

        expect(epirustService.engine).toEqual("./external/epirust");
    //    TODO: Validate execa has been called - Jayanta
    });
});