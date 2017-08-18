// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
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

import { Component, OnDestroy } from "@angular/core";
import { ActivatedRoute } from "@angular/router";
import { FeatureFlags } from "../Privilege";
import { AppStore } from "../AppStore";
import { Package } from "../records/Package";
import { Origin } from "../records/Origin";
import { isPackage, isSignedIn } from "../util";
import { fetchBuild, fetchBuilds, fetchPackage, setProjectHint, requestRoute } from "../actions/index";
import { BuilderApiClient } from "../BuilderApiClient";
import { Subscription } from "rxjs/Subscription";

@Component({
    template: `
    <div class="hab-package page-title">
        <h2>
            <hab-package-breadcrumbs [ident]="package.ident"></hab-package-breadcrumbs>
        </h2>
        <h4 *ngIf="releaseParam">{{ releaseParam }}</h4>
    </div>
    <div *ngIf="!ui.loading && !ui.exists">
      <p>
          Failed to load package.
          <span *ngIf="ui.errorMessage">
              Error: {{ui.errorMessage}}
          </span>
      </p>
    </div>
    <hab-tabs>
        <hab-tab *ngIf="build" tabTitle="Build Output">
            <div class="page-body has-sidebar">
                <hab-build [build]="build" stream="false"></hab-build>
            </div>
        </hab-tab>
        <hab-tab tabTitle="Manifest">
            <div class="page-body has-sidebar">
                <hab-package-info [package]="package"></hab-package-info>
            </div>
        </hab-tab>
    </hab-tabs>
    `,
})

export class PackagePageComponent implements OnDestroy {
    originParam: string;
    nameParam: string;
    releaseParam: string;
    versionParam: string;

    private fetched: boolean;
    private sub: Subscription;

    constructor(private route: ActivatedRoute, private store: AppStore) {

        this.sub = route.params.subscribe(params => {
            this.originParam = params["origin"];
            this.nameParam = params["name"];
            this.versionParam = params["version"];
            this.releaseParam = params["release"];
            this.fetchPackage();
            this.fetchBuilds();
        });
    }

    ngOnDestroy() {
        this.sub.unsubscribe();
    }

    get build() {
        let b = this.store.getState().builds.visible.find((b) => {
            return b.release === this.package.ident.release;
        });

        if (b && !this.fetched) {
            this.store.dispatch(fetchBuild(b.id, this.token));
            this.fetched = true;
        }

        return b;
    }

    get features() {
        return this.store.getState().users.current.flags;
    }

    // Initially set up the package to be whatever comes from the params,
    // so we can query for its versions and releases. In ngOnInit, we'll
    // populate more data by dispatching setCurrentPackage.
    get package() {
        const currentPackageFromState = this.store.getState().packages.current;

        // Use the currentPackage from the state if it's the same package we want
        // here.
        if (isPackage(currentPackageFromState || {}, { ident: this.packageParams() })) {
            return currentPackageFromState;
        } else {
            return Package({ ident: this.packageParams() });
        }
    }

    get origin() {
        return this.package.ident.origin;
    }

    get projectId() {
        return `${this.package.ident.origin}/${this.package.ident.name}`;
    }

    get project() {
        return this.store.getState().projects.added.find(proj => { return proj["id"] === this.projectId; });
    }

    get token() {
        return this.store.getState().gitHub.authToken;
    }

    get ui() {
        return this.store.getState().packages.ui.current;
    }

    get memberOfOrigin() {
        return this.store.getState().origins.mine.includes(Origin({name: this.package.ident.origin}));
    }

    createProject() {
        this.store.dispatch(setProjectHint({
            originName: this.package.ident.origin,
            packageName: this.package.ident.name
        }));
        this.store.dispatch(requestRoute(["/projects", "create"]));
    }

    private packageParams() {
        return {
            origin: this.originParam,
            name: this.nameParam,
            version: this.versionParam,
            release: this.releaseParam
        };
    }

    private fetchBuilds() {
        this.store.dispatch(
            fetchBuilds(this.package.ident.origin, this.package.ident.name, this.token)
        );

    }

    private fetchPackage () {
        this.store.dispatch(fetchPackage(this.package));
    }
}
