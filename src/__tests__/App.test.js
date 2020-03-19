import React from 'react';
import { render } from '@testing-library/react';
import App from '../App';

test('renders App', () => { //TODO: test child components are rendered, preferably convert to snapshot
  const { getByText , container} = render(<App />);
  expect(getByText('EpiViz')).toBeInTheDocument();
});
