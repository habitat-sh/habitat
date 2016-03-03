// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import {AppStore} from "../AppStore";
import {Component, OnInit} from "angular2/core";
import {RouteParams, RouterLink} from "angular2/router";
import {filterPackagesBy, requestRoute} from "../actions/index";

@Component({
    directives: [RouterLink],
    template: `
    <div class="bldr-packages">
        <h2>
            <span *ngIf="filter === 'mine'">My Packages</span>
            <span *ngIf="showAll">All Packages</span>
            <span *ngIf="origin">{{origin}}</span>
            <span *ngIf="name">
                <a [routerLink]="['Packages']">*</a>
                /
                {{name}}
            </span>
        </h2>
        <ul class="bldr-packages-plan-list">
            <li *ngIf="packages.size === 0">
                No packages found. Here's how to create one: &hellip;
            </li>
            <li class="bldr-packages-package" *ngFor="#package of packages">
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

                    <span class="stars" *ngIf="package.starCount">{{package.starCount}}</span>
                </a>
            </li>
        </ul>
    </div>`,
})

export class PackagesPageComponent implements OnInit {
    constructor(private store: AppStore, private routeParams: RouteParams) { }

    get origin() {
        return this.routeParams.params["origin"];
    }

    get filter() {
        return this.routeParams.params["filter"];
    }

    get name() {
        return this.routeParams.params["name"];
    }

    get packages() {
        return this.store.getState().packages.visible;
    }

    get showAll() {
        return Object.keys(this.routeParams.params).length === 0;
    }

    ngOnInit() {
        if (!this.store.getState().user.isSignedIn) {
            this.store.dispatch(requestRoute(["Home"]));
        }

        this.store.dispatch(filterPackagesBy(this.routeParams.params));
    }
}
