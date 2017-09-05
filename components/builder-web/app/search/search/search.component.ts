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
import { AppStore } from "../../AppStore";
import { filterPackagesBy, setPackagesSearchQuery } from "../../actions/index";
import { Subscription } from "rxjs/Subscription";

@Component({
    template: require("./search.component.html")
})

export class SearchComponent implements OnInit, OnDestroy {
    perPage: number = 50;
    query: string;
    searchBox: FormControl;
    origin: string;

    private sub: Subscription;

    constructor(private store: AppStore, private route: ActivatedRoute, private router: Router) {

        this.sub = route.params.subscribe(params => {
            this.origin = params["origin"];
            this.query = params["query"];
            this.fetch();
        });
    }

    ngOnInit() {
        this.searchBox = new FormControl(this.searchQuery);

        this.searchBox.valueChanges.debounceTime(400).distinctUntilChanged().
            subscribe(query => {
                if (query) {
                    this.search(query);
                } else {
                    this.store.dispatch(setPackagesSearchQuery(""));
                    this.fetch();
                }
            });
    }

    ngOnDestroy() {
        if (this.sub) {
            this.sub.unsubscribe();
        }
    }

    fetch() {
        if (this.query) {
            this.search(this.query);
        } else {
            this.fetchPackages();
        }
    }

    get packages() {
        return this.store.getState().packages.visible;
    }

    get searchQuery() {
        return this.store.getState().packages.searchQuery;
    }

    get totalCount() {
        return this.store.getState().packages.totalCount;
    }

    get ui() {
        return this.store.getState().packages.ui.visible;
    }

    fetchMorePackages() {
        this.store.dispatch(filterPackagesBy(
            this.packageParams(),
            this.searchQuery,
            true,
            this.store.getState().packages.nextRange)
        );
        return false;
    }

    packageParams() {
        return {
            origin: this.origin,
        };
    }

    private fetchPackages() {
        this.store.dispatch(filterPackagesBy(
            this.packageParams(),
            this.searchQuery,
            true)
        );
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
