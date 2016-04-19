// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {Component, OnInit} from "angular2/core";
import {RouterLink} from "angular2/router";

@Component({
    directives: [RouterLink],
    inputs: ["isSignedIn", "origin", "route"],
    selector: "hab-side-nav",
    template: `
    <nav class="hab-side-nav">
        <div class="switcher">
            <a *ngIf="isSignedIn && !origin.name"
               [routerLink]="['OriginCreate']">
                Add Origin
            </a>
            <span *ngIf="origin.name">{{origin.name}}</span>
        </div>
        <ul *ngIf="isSignedIn">
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
            <li *ngIf="isSignedIn">
                <a [class.active]='routeMatch("pkgs.+filter=mine")'
                   [routerLink]="['Packages', { filter: 'mine' }]">
                    My Packages
                </a>
            </li>
        </ul>
        <h4 *ngIf="isSignedIn">Organizations</h4>
        <ul *ngIf="isSignedIn">
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
