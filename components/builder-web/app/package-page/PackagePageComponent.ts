// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

import {Component, OnInit} from "angular2/core";
import {RouteParams, RouterLink} from "angular2/router";
import {AppStore} from "../AppStore";
import {Package} from "../records/Package";
import {PackageBreadcrumbsComponent} from "../PackageBreadcrumbsComponent";
import {PackageListComponent} from "./PackageListComponent";
import {SpinnerComponent} from "../SpinnerComponent";
import {isPackage, packageString} from "../util";
import {fetchPackage} from "../actions/index";

@Component({
    directives: [PackageBreadcrumbsComponent, PackageListComponent, RouterLink,
        SpinnerComponent],
    template: `
    <div class="hab-package page-title">
        <h2>Package</h2>
        <h4>
            <package-breadcrumbs [ident]="package.ident">
            </package-breadcrumbs>
        </h4>
        <hab-spinner [isSpinning]="ui.loading" [onClick]="spinnerFetchPackage">
        </hab-spinner>
    </div>
    <div *ngIf="!ui.loading" class="page-body has-sidebar">
        <div class="page-body--main">
            <div *ngIf="!ui.exists && !ui.loading">
                <p>
                    Failed to load package.
                    <span *ngIf="ui.errorMessage">
                        Error: {{ui.errorMessage}}
                    </span>
                </p>
            </div>
            <div *ngIf="ui.exists && !ui.loading">
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
                    <pre>{{package.config}}</pre>
                </div>
            </div>
        </div>
        <div class="page-body--sidebar">
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
    private spinnerFetchPackage: Function;

    constructor(private routeParams: RouteParams, private store: AppStore) {
        this.spinnerFetchPackage = this.fetchPackage.bind(this);
    }

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

    get ui() {
        return this.store.getState().packages.ui.current;
    }

    public ngOnInit() {
        this.fetchPackage();
    }

    private fetchPackage () {
        this.store.dispatch(fetchPackage(this.package));
    }

    private packageString(params) { return packageString(params); }
}
