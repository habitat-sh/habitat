// Copyright:: Copyright (c) 2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import {AppStore} from "../AppStore";
import {Component, OnInit} from "angular2/core";
import {PackageListComponent} from "./PackageListComponent";
import {RouteParams, RouterLink} from "angular2/router";
import {isPackage, packageString} from "../util";
import {fetchPackage} from "../actions/index";

@Component({
    directives: [PackageListComponent, RouterLink],
    template: `
    <div>
        <div *ngIf="!package" class="bldr-package">
            <h2>Not Found</h2>
            <p>{{packageString(package)}} does not exist.</p>
            <p>Here's how you would make it: &hellip;</p>
        </div>
        <div *ngIf="package" class="bldr-package">
            <h2>
                <a [routerLink]="['PackagesForOrigin', { origin: package.origin }]">{{package.origin}}</a>
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
                <package-list [currentPackage]="package"
                              [packages]="package.versions"></package-list>
            </div>
            <div class="bldr-package-releases">
                <h3>Releases <small>of version {{package.version}}</small></h3>
                <package-list [currentPackage]="package"
                              [packages]="package.releases"></package-list>
            </div>
            <div class="bldr-package-deps-build">
                <h3>Build Dependencies</h3>
                <package-list [currentPackage]="package"
                              [packages]="package.buildDependencies"></package-list>
            </div>
            <div class="bldr-package-deps-runtime">
                <h3>Runtime Dependencies</h3>
                <package-list [currentPackage]="package"
                              [packages]="package.dependencies"></package-list>
            </div>
      </div>
  </div>`,
})

export class PackagePageComponent implements OnInit {
    constructor(private routeParams: RouteParams, private store: AppStore) { }

    // Initially set up the package to be whatever comes from the params,
    // so we can query for its versions and releases. In ngOnInit, we'll
    // populate more data by dispatching setCurrentPackage.
    get package() {
        const currentPackageFromState = this.store.getState().currentPackage;
        const params = this.routeParams.params;

        // Use the currentPackage from the state if it's the same package we want
        // here.
        if (isPackage(currentPackageFromState || {}, params)) {
            return currentPackageFromState;
        } else {
            return params;
        }
    }

    ngOnInit() {
        this.store.dispatch(fetchPackage(this.package));
    }

    packageString(params) { return packageString(params); }
}
