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
