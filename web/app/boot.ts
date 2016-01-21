import "angular2/bundles/angular2-polyfills"
import {AppComponent} from "./AppComponent";
import {AppStore} from "./AppStore";
import {ROUTER_PROVIDERS} from "angular2/router";
import {bootstrap} from "angular2/platform/browser";

bootstrap(AppComponent, [
  AppStore,
  ROUTER_PROVIDERS,
]);
