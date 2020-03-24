import DiseaseDynamics from '../DiseaseDynamics'
import { render, fireEvent } from '@testing-library/react'
import React from 'react'

test('should update disease dynamics for a disease selected', () => {
    const { getByPlaceholderText, getByTestId } = render(<DiseaseDynamics />);

    fireEvent.change(getByTestId("select-disease"), {target: {value: "sars"}})

    expect(getByPlaceholderText("Regular Transmission Start Day").value).toBe("5")
    expect(getByPlaceholderText("High Transmission Start Day").value).toBe("10")
    expect(getByPlaceholderText("Last Day").value).toBe("23")
    expect(getByPlaceholderText("Regular Transmission Rate").value).toBe("0.025")
    expect(getByPlaceholderText("High Transmission Rate").value).toBe("0.25")
    expect(getByPlaceholderText("Death Rate").value).toBe("0.1")
})