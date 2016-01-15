import {AppComponent} from "./app.component";
import appState from "./app-state";
import {ROUTER_PROVIDERS} from "angular2/router";
import {bootstrap}    from "angular2/platform/browser";

// Expose the app state on the window so we can inspect it in a console.
window["appState"] = appState;

bootstrap(AppComponent, [ROUTER_PROVIDERS]);
