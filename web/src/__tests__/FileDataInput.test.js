import React from 'react'
import FileDataInput from '../FileDataInput'
import renderer from 'react-test-renderer' 

test('should render FileDataInput', () => {
    const component = renderer.create(<FileDataInput onFileDataSubmit={jest.fn()}/>)
    let tree = component.toJSON()

    expect(tree).toMatchSnapshot()
})

