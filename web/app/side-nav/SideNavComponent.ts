// Copyright:: Copyright (c) 2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import {Component, OnInit} from "angular2/core";
import {RouterLink} from "angular2/router";

@Component({
    directives: [RouterLink],
    inputs: ["route"],
    selector: "hab-side-nav",
    template: `
    <nav class="hab-side-nav">
        <ul>
            <li><a [class.active]='routeMatch("projects")'
                   [routerLink]="['Projects']">Projects</a></li>
        </ul>
        <hr>
        <h4>Public Packages</h4>
        <ul>
            <li><a [class.active]='routeMatch("explore")'
                   [routerLink]="['Explore']">Explore</a></li>
            <li><a [class.active]='routeMatch("pkgs$")'
                   [routerLink]="['Packages']">All Packages</a></li>
            <li><a [class.active]='routeMatch("pkgs.+filter=mine")'
                   [routerLink]="['Packages', { filter: 'mine' }]">My Packages</a></li>
        </ul>
        <h4>Linked Accounts</h4>
        <ul>
            <li><a [class.active]='routeMatch("linked-accounts")'
                   [routerLink]='["LinkedAccounts"]'>Manage Accounts</a></li>
            <li><a [class.active]='routeMatch("scm-repos")'
                   [routerLink]="['SCMRepos']">GitHub Repos</a></li>
        </ul>
        <h4>Organizations</h4>
        <ul>
            <li><a [class.active]='routeMatch("orgs")'
                   [routerLink]="['Organizations']">Manage Orgs</a></li>
        </ul>
    </nav>`
})

export class SideNavComponent {
    private route: string;

    // Return true if a route matches what we're looking at.
    private routeMatch(s: string): boolean {
        return this.route.match(s) !== null;
    }
}
