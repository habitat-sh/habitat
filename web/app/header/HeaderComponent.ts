// Copyright:: Copyright (c) 2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import {Component} from "angular2/core";
import {RouterLink} from "angular2/router";
import {UserNavComponent} from "./user-nav/UserNavComponent";
import {url} from "../util";

@Component({
    directives: [RouterLink, UserNavComponent],
    inputs: ["appName", "route"],
    selector: "bldr-header",
    template: `
    <header class="bldr-header">
        <h1>{{appName}}</h1>
        <nav class="bldr-header-links">
            <ul>
                <li><a [ngClass]="{ active: onExplore }" [routerLink]="['Explore']">Explore</a></li>
                <li><a [ngClass]="{ active: onAllPackages }" [routerLink]="['Packages']">All Packages</a></li>
                <li><a [ngClass]="{ active: onMyPackages }" [routerLink]="['Packages', { filter: 'mine' }]">My Packages</a></li>
            </ul>
        </nav>
        <nav class="bldr-header-user">
            <user-nav></user-nav>
        </nav>
    </header>`,
})

export class HeaderComponent {
    private route;

    // Methods to figure out which page we're on
    get onAllPackages() {
        const route = url(this.route);
        return route.pathname === "/pkgs" &&
            route.search.replace("?filter=", "") !== "mine";
    }

    get onExplore() {
        const route = url(this.route);
        return route.pathname === "/explore";
    }

    get onMyPackages() {
        const route = url(this.route);
        return route.pathname === "/pkgs" &&
            route.search.replace("?filter=", "") === "mine";
    }
}
