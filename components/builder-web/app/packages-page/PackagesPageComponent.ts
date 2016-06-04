// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {Component, OnInit} from "angular2/core";
import {RouteParams, RouterLink} from "angular2/router";
import {AppStore} from "../AppStore";
import {PackageBreadcrumbsComponent} from "../PackageBreadcrumbsComponent";
import {SpinnerComponent} from "../SpinnerComponent";
import {filterPackagesBy} from "../actions/index";
import {requireSignIn} from "../util";

@Component({
    directives: [PackageBreadcrumbsComponent, RouterLink, SpinnerComponent],
    template: `
    <div class="hab-packages">
        <div class="page-title">
            <h2>
                <hab-spinner [isSpinning]="ui.loading" [onClick]="spinnerFetchPackages">
                </hab-spinner>
                <package-breadcrumbs [ident]="routeParams.params"
                    [params]="routeParams.params">
                </package-breadcrumbs>
            </h2>
        </div>
        <div class="page-body">
            <ul class="hab-packages-plan-list">
                <div *ngIf="(!ui.exists || packages.size === 0) && !ui.loading">
                    <p>
                        Failed to load packages.
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
            <div *ngIf="packages.size <= totalCount">
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
    private spinnerFetchPackages: Function;

    constructor(private store: AppStore, private routeParams: RouteParams) {
        this.spinnerFetchPackages = this.fetchPackages.bind(this);
    }

    get packages() {
        return this.store.getState().packages.visible;
    }

    get totalCount() {
        return this.store.getState().packages.totalCount;
    }

    get ui() {
        return this.store.getState().packages.ui.visible;
    }

    public ngOnInit() {
        if (this.routeParams.params["filter"] === "mine") {
            requireSignIn(this);
        }

        this.fetchPackages();
    }

    private fetchPackages() {
        this.store.dispatch(filterPackagesBy(this.routeParams.params));
    }

    private fetchMorePackages() {
        this.store.dispatch(filterPackagesBy(this.routeParams.params,
            this.store.getState().packages.nextRange));
        return false;
    }
}
