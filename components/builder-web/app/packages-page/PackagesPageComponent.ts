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

import {Control} from "angular2/common";
import {Component, OnInit} from "angular2/core";
import {RouteParams, RouterLink} from "angular2/router";
import {AppStore} from "../AppStore";
import {PackageBreadcrumbsComponent} from "../PackageBreadcrumbsComponent";
import {SpinnerComponent} from "../SpinnerComponent";
import {filterPackagesBy, setPackagesSearchQuery} from "../actions/index";
import {requireSignIn} from "../util";

@Component({
    directives: [PackageBreadcrumbsComponent, RouterLink, SpinnerComponent],
    template: `
    <div class="hab-packages">
        <div class="page-title">
            <h2>Search Packages</h2>
            <h4>
                <span *ngIf="searchQuery || routeParams.params['query']">Search Results</span>
                <package-breadcrumbs
                    *ngIf="!searchQuery"
                    [ident]="routeParams.params"
                    [params]="routeParams.params">
                </package-breadcrumbs>
            </h4>
            <hab-spinner [isSpinning]="ui.loading" [onClick]="spinnerFetchPackages">
            </hab-spinner>
        </div>
        <div class="page-body">
            <input type="search" autofocus
                [ngFormControl]="searchBox"
                placeholder="Search Packages&hellip;">
            <ul class="hab-packages-plan-list">
                <div *ngIf="(!ui.exists || packages.size === 0) && !ui.loading">
                    <p>
                        No packages found.
                        <span *ngIf="ui.errorMessage">
                            Error: {{ui.errorMessage}}
                        </span>
                    </p>
                </div>
                <li class="hab-packages-package" *ngFor="#package of packages">
                    <a [routerLink]="['Package', { origin: package.origin,
                                                   name: package.name,
                                                   version: package.version,
                                                   release: package.release }]">
                        <div class="item-title">
                            <h3>{{package.origin}} / {{package.name}}</h3>
                            <h4>{{package.version}} / {{package.release}}</h4>
                        </div>
                        <div class="item-info">
                            <span class="count" *ngIf="package.starCount">
                                <!-- TODO: import octicons -->
                                <span class="octicon octicon-star"></span>
                                {{package.starCount}}
                            </span>
                            <img src="/node_modules/octicons/svg/chevron-right.svg" />
                        </div>
                    </a>
                </li>
            </ul>
            <div *ngIf="packages.size < totalCount">
                Showing {{packages.size}} of {{totalCount}} packages.
                <a href="#" (click)="fetchMorePackages()">
                    Load
                    {{(totalCount - packages.size) > perPage ? perPage : totalCount - packages.size }}
                    more</a>.
            </div>
        </div>
    </div>`,
})

export class PackagesPageComponent implements OnInit {
    private perPage: number = 50;
    private searchBox: Control;
    private spinnerFetchPackages: Function;

    constructor(private store: AppStore, private routeParams: RouteParams) {
        this.spinnerFetchPackages = this.fetchPackages.bind(this);
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

    public ngOnInit() {
        if ("query" in this.routeParams.params) {
            this.search(this.routeParams.params["query"]);
        } else {
            this.fetchPackages();
        }

        this.searchBox = new Control(this.searchQuery);

        this.searchBox.valueChanges.debounceTime(400).distinctUntilChanged().
            subscribe(query => this.search(query));
    }

    private fetchPackages() {
        this.store.dispatch(filterPackagesBy(this.routeParams.params,
            this.searchQuery));
    }

    private fetchMorePackages() {
        this.store.dispatch(filterPackagesBy(this.routeParams.params,
            this.searchQuery,
            this.store.getState().packages.nextRange));
        return false;
    }

    private search(query) {
        this.store.dispatch(setPackagesSearchQuery(query));
        this.fetchPackages();
        return false;
    }
}
