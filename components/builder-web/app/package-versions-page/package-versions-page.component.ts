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

import { Component, OnInit, OnDestroy } from "@angular/core";
import { RouterLink, ActivatedRoute } from "@angular/router";
import { AppStore } from "../AppStore";
import { fetchLatestPackage, fetchPackageVersions } from "../actions/index";
import { Subscription } from "rxjs/Subscription";

@Component({
    template: require("./package-versions-page.component.html")
})

export class PackageVersionsPageComponent implements OnInit, OnDestroy {
    loadVersions: Function;

    private name: string;
    private origin: string;
    private sub: Subscription;

    constructor(private route: ActivatedRoute, private store: AppStore) {
        this.sub = route.params.subscribe(params => {
            this.name = params["name"];
            this.origin = params["origin"];
            this.fetchVersions();
            this.fetchLatest();
        });
    }

    ngOnInit() {
        this.loadVersions = this.fetchVersions.bind(this);
    }

    ngOnDestroy() {
        this.sub.unsubscribe();
    }

    get ui() {
        return this.store.getState().packages.ui.versions;
    }

    get versions() {
        return this.store.getState().packages.versions || [];
    }

    get latestPackage() {
        return this.store.getState().packages.current;
    }

    private fetchVersions() {
        this.store.dispatch(fetchPackageVersions(this.origin, this.name));
    }

    private fetchLatest() {
        this.store.dispatch(fetchLatestPackage(this.origin, this.name));
    }
}
