import renderer from 'react-test-renderer'
import Cell from "../../grid/Cell";
import {Color} from "../../grid/Color";
import React from "react";


describe('Cell', function () {
    it('should render with specified color and size', function () {
        const testColor = new Color(10, 11, 12);
        const component = renderer.create(<Cell color={testColor} size={10} cellId={1}/>);

        const tree = component.toJSON();

        expect(tree).toMatchSnapshot();
    });
});