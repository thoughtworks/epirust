import React from 'react';

import { Link, useRouteMatch } from "react-router-dom";
import { Routes } from './App';

export default function Header() {

    function NavItem({ name, linksTo, activeOnExactMatch = false }) {
        let match = useRouteMatch({
            path: linksTo,
            exact: activeOnExactMatch
        });

        return (
            <li className={`nav-item ${match ? "active" : ""}`}>
                <Link className="nav-link" to={linksTo}>{name}</Link>
            </li>
        );
    }

    return (
        <nav className="navbar navbar-expand-lg navbar-dark bg-dark">
            <a className="navbar-brand" href="/">EpiViz</a>
            <button className="navbar-toggler" type="button" data-toggle="collapse" data-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
                <span className="navbar-toggler-icon" />
            </button>
            <div className="collapse navbar-collapse" id="navbarSupportedContent">
                <ul className="navbar-nav mr-auto">

                    <NavItem name={"Home"} linksTo={Routes.HOME} activeOnExactMatch={true} />
                    <NavItem name={"Time Series"} linksTo={Routes.TIME_SERIES} />
                    <NavItem name={"Grid Visualization"} linksTo={Routes.GRID} />
                    <NavItem name={"Jobs"} linksTo={Routes.JOBS} />

                </ul>
            </div>
        </nav>
    );
}
