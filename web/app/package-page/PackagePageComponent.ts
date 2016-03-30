// Copyright:: Copyright (c) 2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import {AppStore} from "../AppStore";
import {Component, OnInit} from "angular2/core";
import {Package} from "../records/Package";
import {PackageListComponent} from "./PackageListComponent";
import {RouteParams, RouterLink} from "angular2/router";
import {isPackage, packageString} from "../util";
import {fetchPackage} from "../actions/index";

@Component({
    directives: [PackageListComponent, RouterLink],
    template: `
    <div>
        <div *ngIf="!package" class="hab-package">
            <h2>Not Found</h2>
            <p>{{packageString(package)}} does not exist.</p>
            <p>Here's how you would make it: &hellip;</p>
        </div>
        <div *ngIf="package" class="hab-package">
            <h2>
                <a [routerLink]="['PackagesForOrigin', { origin: package.ident.origin }]">
                    {{package.ident.origin}}
                </a>
                /
                {{package.ident.name}}
                /
                {{package.ident.version}}
                /
                {{package.ident.release}}
            </h2>
            <div class="hab-package-info">
                <dl>
                    <dt>Version</dt>
                    <dd>{{package.ident.version}}</dd>
                    <dt>Release</dt>
                    <dd>{{package.ident.release}}</dd>
                    <dt>Checksum</dt>
                    <dd>{{package.checksum}}</dd>
                    <dt *ngIf="package.exposes.length > 0">Exposed Ports</dt>
                    <dd *ngIf="package.exposes.length > 0">
                        <span *ngFor="#port of package.exposes">{{port}} </span>
                    </dd>
                </dl>
            </div>
            <div class="hab-package-manifest">
                <h3>Manifest</h3>
                <div class="manifest" [innerHTML]="package.manifest"></div>
            </div>
            <div class="hab-package-config" *ngIf="package.config">
                <h3>Configuration</h3>
                <pre> {{package.config}}</pre>
            </div>
            <div class="hab-package-deps-build">
                <h3>Dependencies</h3>
                <package-list [currentPackage]="package"
                              [packages]="package.deps"></package-list>
            </div>
            <div class="hab-package-deps-runtime">
                <h3>Transitive Dependencies</h3>
                <package-list [currentPackage]="package"
                              [packages]="package.tdeps"></package-list>
            </div>
      </div>
  </div>`,
})

export class PackagePageComponent implements OnInit {
    private markdown;

    constructor(private routeParams: RouteParams, private store: AppStore) { }

    // Initially set up the package to be whatever comes from the params,
    // so we can query for its versions and releases. In ngOnInit, we'll
    // populate more data by dispatching setCurrentPackage.
    get package() {
        const currentPackageFromState = this.store.getState().packages.current;
        const params = this.routeParams.params;

        // Use the currentPackage from the state if it's the same package we want
        // here.
        if (isPackage(currentPackageFromState || {}, { ident: params })) {
            return currentPackageFromState;
        } else {
            return Package({ ident: params });
        }
    }

    ngOnInit() {
        this.store.dispatch(fetchPackage(this.package));
    }

    packageString(params) { return packageString(params); }
}
