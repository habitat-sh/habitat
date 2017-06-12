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

import { FormControl } from "@angular/forms";
import { Component, OnInit, OnDestroy } from "@angular/core";
import { ActivatedRoute, Router } from "@angular/router";
import { AppStore } from "../AppStore";
import { clearBuilds, fetchBuilds, fetchPackageVersions, filterPackagesBy,
         scheduleBuild, setPackagesSearchQuery } from "../actions/index";
import { requireSignIn } from "../util";
import { Subscription } from "rxjs/Subscription";

@Component({
    template: require("./packages-page.component.html")
})

export class PackagesPageComponent implements OnInit, OnDestroy {
    perPage: number = 50;
    query: string;
    searchBox: FormControl;
    spinnerFetchPackages: Function;
    name: string;
    origin: string;
    version: string;

    private sub: Subscription;

    constructor(private store: AppStore, private route: ActivatedRoute, private router: Router) {
        this.spinnerFetchPackages = this.fetchPackages.bind(this);

        this.sub = route.params.subscribe(params => {
            this.name = params["name"];
            this.origin = params["origin"];
            this.version = params["version"];
            this.query = params["query"];
            this.fetch();
        });
    }

    ngOnInit() {
        this.searchBox = new FormControl(this.searchQuery);

        this.searchBox.valueChanges.debounceTime(400).distinctUntilChanged().
            subscribe(query => this.search(query));
    }

    ngOnDestroy() {
        this.store.dispatch(clearBuilds());

        if (this.sub) {
            this.sub.unsubscribe();
        }
    }

    get activeBuild() {
        for (let i = 0; i < this.builds.size; i++ ) {
            let build = this.builds.get(i);

            if (build["state"] === "Dispatched" || build["state"] === "Pending") {
                return build;
            }
        }

        return null;
    }

    fetch() {
        this.store.dispatch(clearBuilds());

        if (this.query) {
            this.search(this.query);
        } else {
            this.fetchBuilds();

            if (this.name && !this.version) {
                this.fetchVersions();
            } else {
                this.fetchPackages();
            }
        }
    }

    get builds() {
        return this.store.getState().builds.visible;
    }

    get iCanRequestABuild() {
        let isMember = !!this.store.getState().origins.mine.find(o => o.name === "core");

        if (this.origin === "core" && isMember && this.layout === "versions") {
            return true;
        }

        return false;
    }

    get layout() {
        let s = "origin";

        if (this.version) {
            s = "builds";
        }
        else if (this.name) {
            s = "versions";
        }

        return s;
    }

    get packages() {
        return this.store.getState().packages.visible;
    }

    get searchQuery() {
        return this.store.getState().packages.searchQuery;
    }

    get showBreadcrumbs() {
        return this.origin && this.name;
    }

    get showSearch() {
        return (this.origin && !this.name || this.query);
    }

    get subtitle() {
        return this.version ? "builds" : "package";
    }

    get totalCount() {
        return this.store.getState().packages.totalCount;
    }

    get ui() {
        return this.store.getState().packages.ui.visible;
    }

    get versions() {
        return this.store.getState().packages.versions;
    }

    fetchMorePackages() {
        this.store.dispatch(filterPackagesBy(
            this.packageParams(),
            this.searchQuery,
            this.distinct(),
            this.store.getState().packages.nextRange)
        );
        return false;
    }

    packageParams() {
        return {
            name: this.name,
            origin: this.origin,
            version: this.version
        };
    }

    requestNewBuild() {
        let token = this.store.getState().gitHub.authToken;
        this.store.dispatch(scheduleBuild(this.origin, this.name, token));
    }

    private fetchBuilds() {
        if (this.origin && this.name) {
            let token = this.store.getState().gitHub.authToken;

            if (token) {
                this.store.dispatch(fetchBuilds(this.origin, this.name, token));
            }
        }
    }

    private fetchPackages() {
        this.store.dispatch(filterPackagesBy(
            this.packageParams(),
            this.searchQuery,
            this.distinct())
        );
    }

    private fetchVersions() {
        this.store.dispatch(fetchPackageVersions(this.origin, this.name));
    }

    private distinct() {
        return !this.version;
    }

    private search(query) {
        this.store.dispatch(setPackagesSearchQuery(query));

        if (query === "") {
            this.router.navigate(["/pkgs"]);
        }
        else {
            this.fetchPackages();
        }

        return false;
    }
}
