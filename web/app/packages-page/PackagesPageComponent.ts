// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import {Component, OnInit} from "angular2/core";
import {RouteParams, RouterLink} from "angular2/router";
import {AppStore} from "../AppStore";
import {PackageBreadcrumbsComponent} from "../PackageBreadcrumbsComponent";
import {filterPackagesBy, requestRoute} from "../actions/index";

@Component({
    directives: [PackageBreadcrumbsComponent, RouterLink],
    template: `
    <div class="hab-packages">
        <h2>
            <package-breadcrumbs [ident]="routeParams.params"
                [params]="routeParams.params">
            </package-breadcrumbs>
        </h2>
        <hr>
        <ul class="hab-packages-plan-list">
            <li *ngIf="packages.size === 0">
                No packages found. Here's how to create one: &hellip;
            </li>
            <li class="hab-packages-package" *ngFor="#package of packages">
                <a [routerLink]="['Package', { origin: package.origin,
                                               name: package.name,
                                               version: package.version,
                                               release: package.release }]">
                    {{package.origin}}
                    /
                    {{package.name}}
                    /
                    {{package.version}}
                    /
                    {{package.release}}

                    <span class="count" *ngIf="package.starCount">{{package.starCount}} ★</span>
                </a>
            </li>
        </ul>
    </div>`,
})

export class PackagesPageComponent implements OnInit {
    constructor(private store: AppStore, private routeParams: RouteParams) { }

    get packages() {
        return this.store.getState().packages.visible;
    }

    ngOnInit() {
        if (!this.store.getState().user.isSignedIn) {
            this.store.dispatch(requestRoute(["Home"]));
        }

        this.store.dispatch(filterPackagesBy(this.routeParams.params));
    }
}
