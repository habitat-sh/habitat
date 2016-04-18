// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {Component, OnInit} from "angular2/core";
import {RouterLink} from "angular2/router";
import {OriginPickerComponent} from "./OriginPickerComponent";

@Component({
    directives: [OriginPickerComponent, RouterLink],
    inputs: ["fetchMyOrigins", "isSignedIn", "isOriginPickerOpen", "myOrigins",
        "origin", "route", "setCurrentOrigin", "toggleOriginPicker"],
    selector: "hab-side-nav",
    template: `
    <nav class="hab-side-nav">
        <hab-origin-picker [fetchMyOrigins]="fetchMyOrigins"
                           [isSignedIn]="isSignedIn"
                           [isOpen]="isOriginPickerOpen"
                           [myOrigins]="myOrigins"
                           [currentOrigin]="origin"
                           [setCurrentOrigin]="setCurrentOrigin"
                           [toggleOriginPicker]="toggleOriginPicker">
        </hab-origin-picker>
        <ul class="hab-side-nav--list" *ngIf="isSignedIn">
            <li><a [class.active]='routeMatch("projects")'
                   [routerLink]="['Projects']">Projects</a></li>
        </ul>
        <hr>
        <h4>Public Packages</h4>
        <ul class="hab-side-nav--list">
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
        <ul class="hab-side-nav--list" *ngIf="isSignedIn">
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
