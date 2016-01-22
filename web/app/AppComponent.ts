// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import {AppStore} from "./AppStore";
import {Component, Inject} from "angular2/core";
import {HomeComponent} from "./home/HomeComponent";
import {PackageComponent} from "./package/PackageComponent";
import {PackagesComponent} from "./packages/PackagesComponent";
import {Router, RouteConfig, ROUTER_DIRECTIVES} from "angular2/router";
import {SignInComponent} from "./sign-in/SignInComponent";
import {UserNavComponent} from "./user-nav/UserNavComponent";
import {routeChange} from "./actions";

@Component({
  directives: [ROUTER_DIRECTIVES, UserNavComponent],
  selector: "bldr",
  template: `
    <div class="bldr-container">
      <header class="bldr-header">
        <h1>{{appName}}</h1>
        <nav class="bldr-header-user">
          <user-nav></user-nav>
        </nav>
      </header>
      <section class="bldr-main">
        <router-outlet></router-outlet>
      </section>
      <footer class="bldr-footer">
        <p>&copy; {{now}} Chef Software, Inc. All Rights Reserved.</p>
      </footer>
    </div>
  `,
})

@RouteConfig([
  { path: "/", name: "Home", component: HomeComponent },
  { path: "/packages", name: "Packages", component: PackagesComponent },
  { path: "/packages/:derivation/:name/:version/:release", name: "Package",
    component: PackageComponent },
  { path: "/sign-in", name: "Sign In", component: SignInComponent },
])

export class AppComponent {
  constructor(private router: Router, private store: AppStore) {
    router.subscribe(value => store.dispatch(routeChange(value)));
    store.subscribe(state => {
      let requestedRoute = store.getState().requestedRoute;
      console.log("New state received ", state.toObject());

      if (requestedRoute) { router.navigate(requestedRoute); }
    });
  }

  get appName() {
    return this.store.getState().appName;
  }

  get now() {
    return this.store.getState().currentYear;
  }
}
