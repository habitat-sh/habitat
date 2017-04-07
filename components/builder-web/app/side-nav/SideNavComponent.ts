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

import { Component, Input, OnInit } from "@angular/core";

@Component({
    selector: "hab-side-nav",
    template: `
    <nav class="hab-side-nav">
        <h4>Dashboard</h4>
        <ul class="hab-side-nav--list">
            <li><a [class.active]='routeMatch("pkgs\/core")'
                   [routerLink]="['/pkgs', 'core']">
                Packages
            </a></li>
            <li *ngIf="isSignedIn"><a
                   [class.active]='routeMatch("origins")'
                   [routerLink]="['/origins']">Origins</a></li>
        </ul>
    </nav>`
})

export class SideNavComponent {
    @Input() isSignedIn;
    @Input() route: string;

    // Return true if a route matches what we're looking at.
    private routeMatch(s: string): boolean {
        return this.route.match(s) !== null;
    }
}
