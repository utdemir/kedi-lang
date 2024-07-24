/* @refresh reload */
import { render } from 'solid-js/web'
import { Route, Router } from "@solidjs/router";

import "virtual:css-vars.css"
import './common.css'

import { lazy } from 'solid-js';

render(() =>
    <div class="container">
        <Router base={import.meta.env.BASE_URL}>
            <Route path="/" component={lazy(() => import("./pages/home"))} />
        </Router>
    </div>,
    document.getElementById('root')!
)
