// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import {AppStore} from "../AppStore";
import {Component} from "angular2/core";
import {Router, RouterLink} from "angular2/router";
import query from "../query";
import {requestRoute} from "../actions";

@Component({
  directives: [RouterLink],
  template: `
  <div class="bldr-packages">
    <h2>{{username}}</h2>
    <p>Packages owned by {{username}}</p>
    <ul class="bldr-packages-plan-list">
      <li *ngIf="packages.length === 0">
        You have no packages. Here's how to create one: &hellip;
      </li>
      <li class="bldr-packages-package" *ngFor="#package of packages">
        <a [routerLink]="['Package', { derivation: package.derivation,
                                       name: package.name,
                                       version: package.version,
                                       release: package.release }]">
          {{package.derivation}}/{{package.name}}
          /
          <small>{{package.version}}/{{package.release}}</small>
        </a>
      </li>
    </ul>
  </div>
  `
})

export class PackagesComponent {
  private packages;

  constructor(private store: AppStore) {
    this.packages = query(this.store.getState().packages).
      allMostRecentForDerivation("smith"). // The demo user
      toArray();
  }

  get username() {
    return this.store.getState().username;
  }

  ngOnInit() {
    if (!this.store.getState().isSignedIn) {
      this.store.dispatch(requestRoute(["Home"]));
    }
  }
}
