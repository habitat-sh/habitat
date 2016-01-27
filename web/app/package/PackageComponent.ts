// Copyright:: Copyright (c) 2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import {AppStore} from "../AppStore";
import {Component} from "angular2/core";
import {RouteParams, RouterLink} from "angular2/router";
import {PackageListComponent} from "./PackageListComponent";
import {packageString} from "../util";
import query from "../query";

@Component({
  directives: [PackageListComponent, RouterLink],
  template: `
  <div>
    <div *ngIf="!package" class="bldr-package">
      <h2>Not Found</h2>
      <p>{{currentPackageIdString}} does not exist.</p>
      <p>Here's how you would make it: &hellip;</p>
    </div>
    <div *ngIf="package" class="bldr-package">
      <h2>
        <a [routerLink]="['Packages', { derivation: package.derivation }]">{{package.derivation}}</a>
        /
        {{package.name}}
        /
        {{package.version}}
        /
        {{package.release}}
      </h2>
      <div class="bldr-package-info">
        <dl>
          <dt>Maintainer</dt>
          <dd>{{package.maintainer}}</dd>
          <dt>License</dt>
          <dd>{{package.license}}</dd>
          <dt>Source URL</dt>
          <dd><a href="{{package.source}}">{{package.source}}</a></dd>
        </dl>
      </div>
      <div class="bldr-package-version-info">
        <dl>
          <dt>Version</dt>
          <dd>{{package.version}}</dd>
          <dt>Release</dt>
          <dd>{{package.release}}</dd>
          <dt>SHA</dt>
          <dd>{{package.sha}}</dd>
        </dl>
      </div>
      <div class="bldr-package-versions">
        <h3>Available Versions</h3>
        <package-list [packages]="versions"></package-list>
      </div>
      <div class="bldr-package-releases">
        <h3>Releases <small>of version {{package.version}}</small></h3>
        <package-list [packages]="releases"></package-list>
      </div>
      <div class="bldr-package-deps">
        <div class="bldr-package-deps-build">
          <h3>Build Dependencies</h3>
          <package-list [packages]="package.buildDependencies"></package-list>
        </div>
        <div class="bldr-package-deps-runtime">
          <h3>Runtime Dependencies</h3>
          <package-list [packages]="package.dependencies"></package-list>
        </div>
      </div>
    </div>
  </div>
  `,
})

export class PackageComponent {
  private currentPackageParams;
  private allPackages;
  private releases;
  private versions;

  constructor (private routeParams: RouteParams, private store: AppStore) {
    this.allPackages = this.store.getState().packages;
    this.currentPackageParams = this.routeParams.params;
    this.releases = query(this.allPackages).
      allReleasesForPackageVersion(this.package).
      toArray();
    this.versions = query(this.allPackages).
      allVersionsForPackage(this.package).
      toArray();
    console.log(this.versions);
  }

  get currentPackageIdString() {
    return packageString(this.currentPackageParams);
  }

  get package () {
    return query(this.allPackages).
      fromParams(this.currentPackageParams).
      first();
  }

  packageString(pkg) {
    return packageString(pkg);
  }
}
