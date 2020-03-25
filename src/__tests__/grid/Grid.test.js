import React from "react";
import Grid from "../../grid/Grid";
import renderer from 'react-test-renderer'

describe('Grid', function () {
    it('should have container with cells', function () {
        const gridSize = 5;
        const component = renderer.create(<Grid size={gridSize}/>);

        const tree = component.toJSON();

        expect(tree).toMatchSnapshot();
    });
});