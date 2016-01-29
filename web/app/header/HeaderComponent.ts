// Copyright:: Copyright (c) 2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import {Component} from "angular2/core";
import {ROUTER_DIRECTIVES} from "angular2/router";
import {UserNavComponent} from "./user-nav/UserNavComponent";

@Component({
  directives: [ROUTER_DIRECTIVES, UserNavComponent],
  inputs: ["appName"],
  selector: "bldr-header",
  template: `
    <header class="bldr-header">
      <h1>{{appName}}</h1>
      <nav class="bldr-header-links">
        <ul>
          <li><a [ngClass]="{ active: onAllPackages }" [routerLink]="['Packages', { show: 'all' }]">All Packages</a></li>
          <li><a [ngClass]="{ active: onMyPackages }" [routerLink]="['Packages', { show: 'mine' }]">My Packages</a></li>
        </ul>
      </nav>
      <nav class="bldr-header-user">
        <user-nav></user-nav>
      </nav>
    </header>
  `,
})

export class HeaderComponent {
  // Ok I get that some of the state exists in the URL, but why aren't you using
  // something like RouteParams instead of window.location?
  //
  // Because I was having some trouble with those and this works.
  //
  // See also https://github.com/angular/angular/issues/4016
  get onAllPackages() {
    return window.location.pathname === "/packages" &&
      window.location.search.replace("?show=", "") !== "mine";
  }

  get onMyPackages() {
    return window.location.pathname === "/packages" &&
      window.location.search.replace("?show=", "") === "mine";
  }
}
