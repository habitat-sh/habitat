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
    inputs: ["isSignedIn", "route"],
    selector: "hab-side-nav",
    template: `
    <nav class="hab-side-nav">
        <h4>Dashboard</h4>
        <ul class="hab-side-nav--list">
            <li><a [class.active]='routeMatch("pkgs\/core")'
                   [routerLink]="['PackagesForOrigin', { origin: 'core' }]">
                Packages
            </a></li>
            <li *ngIf="isSignedIn"><a
                   [class.active]='routeMatch("origins")'
                   [routerLink]="['Origins']">Origins</a></li>
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
