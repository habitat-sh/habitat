// Copyright:: Copyright (c) 2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import {AppStore} from "./AppStore";
import {Component, Inject} from "angular2/core";
import {HeaderComponent} from "./header/HeaderComponent";
import {HomeComponent} from "./home/HomeComponent";
import {PackageComponent} from "./package/PackageComponent";
import {PackagesComponent} from "./packages/PackagesComponent";
import {Router, RouteConfig, ROUTER_DIRECTIVES} from "angular2/router";
import {SignInComponent} from "./sign-in/SignInComponent";
import {routeChange} from "./actions";

@Component({
  directives: [ROUTER_DIRECTIVES, HeaderComponent],
  selector: "bldr",
  template: `
    <div class="bldr-container">
      <bldr-header [appName]="state.appName"></bldr-header>
      <section class="bldr-main">
        <router-outlet></router-outlet>
      </section>
      <footer class="bldr-footer">
        <p>&copy; {{state.currentYear}} Chef Software, Inc. All Rights Reserved.</p>
      </footer>
    </div>
  `,
})

@RouteConfig([
  { path: "/", name: "Home", component: HomeComponent },
  { path: "/packages", name: "Packages", component: PackagesComponent },
  { path: "/packages/:derivation", name: "PackagesForDerivation", component: PackagesComponent },
  { path: "/packages/:derivation/:name/:version/:release", name: "Package",
    component: PackageComponent },
  { path: "/sign-in", name: "Sign In", component: SignInComponent },
])

export class AppComponent {
  private state;

  constructor(private router: Router, private store: AppStore) {
    this.state = store.getState();

    router.subscribe(value => store.dispatch(routeChange(value)));

    store.subscribe(state => {
      let requestedRoute = store.getState().requestedRoute;
      console.log("New state received ", state.toObject());

      if (requestedRoute) { router.navigate(requestedRoute); }
    });
  }
}
