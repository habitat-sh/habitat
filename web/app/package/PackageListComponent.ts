// Copyright:: Copyright (c) 2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import {Component} from "angular2/core";
import {RouterLink, RouteParams} from "angular2/router";
import {isPackage, packageString} from "../util";

@Component({
  inputs: ["packages"],
  directives: [RouterLink],
  selector: "package-list",
  template: `
  <ul class="bldr-package-list">
    <li *ngIf="packages.length === 0">None</li>
    <li *ngFor="#pkg of packages">
      <a [ngClass]="{ active: isPackage(currentPackage, pkg) }" [routerLink]="['Package', { derivation: pkg.derivation,
                                     name: pkg.name,
                                     version: pkg.version,
                                     release: pkg.release }]">
        {{packageString(pkg)}}
      </a>
    </li>
  </ul>
  `
})

export class PackageListComponent {
  constructor(private routeParams: RouteParams) {}
  private isPackage(x, y) { return isPackage(x, y); }
  private packageString(pkg) { return packageString(pkg); }
  get currentPackage() { return this.routeParams.params; }
}
