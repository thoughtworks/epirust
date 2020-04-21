import React from 'react';

export default function Loader() {
    return (
        <div id="loader" data-testid="loader" className="multi-spinners">
            <div className="spinner-grow text-dark" />
            <div className="spinner-grow text-dark" />
            <div className="spinner-grow text-dark" />
            <div className="spinner-grow text-dark" />
            <div className="spinner-grow text-dark" />
            <div className="spinner-grow text-dark" />
        </div>
    );
}