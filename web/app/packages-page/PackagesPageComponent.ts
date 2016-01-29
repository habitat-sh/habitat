// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import {AppStore} from "../AppStore";
import {Component, OnInit} from "angular2/core";
import {RouteParams, RouterLink} from "angular2/router";
import {filterPackagesBy, requestRoute} from "../actions";

@Component({
  directives: [RouterLink],
  template: `
  <div class="bldr-packages">
    <h2 *ngIf="show === 'mine'">My Packages</h2>
    <h2 *ngIf="show === 'all'">All Packages</h2>
    <h2 *ngIf="show === 'derivation'">{{derivation}}</h2>
    <ul class="bldr-packages-plan-list">
      <li *ngIf="packages.length === 0">
        No packages found. Here's how to create one: &hellip;
      </li>
      <li class="bldr-packages-package" *ngFor="#package of packages">
        <a [routerLink]="['Package', { derivation: package.derivation,
                                       name: package.name,
                                       version: package.version,
                                       release: package.release }]">
          {{package.derivation}}
          /
          {{package.name}}
          /
          {{package.version}}
          /
          {{package.release}}
        </a>
      </li>
    </ul>
  </div>
  `
})

export class PackagesPageComponent implements OnInit {
  constructor(private store: AppStore, private routeParams: RouteParams) {}

  get derivation() {
    return this.routeParams.params["derivation"];
  }

  get packages() {
    return this.store.getState().visiblePackages;
  }

  get show() {
    if (this.routeParams.params["show"] === "mine") {
      return "mine";
    } else if (this.routeParams.params["derivation"]) {
      return "derivation";
    } else {
      return "all";
    }
  }

  get username() {
    return this.store.getState().username;
  }

  ngOnInit() {
    if (!this.store.getState().isSignedIn) {
      this.store.dispatch(requestRoute(["Home"]));
    }

    this.store.dispatch(filterPackagesBy(this.show,
                                         this.routeParams.params["derivation"]));
  }
}
