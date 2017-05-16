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
import config from "../config";

@Component({
    selector: "hab-side-nav",
    template: `
    <nav class="hab-side-nav">
        <h4>Depot</h4>
        <ul class="hab-side-nav--list">
            <li *ngIf="isSignedIn">
                <a [routerLink]="['/']"
                    routerLinkActive="active"
                    [routerLinkActiveOptions]="{exact: true}">Dashboard</a>
            </li>
            <li *ngIf="isSignedIn">
                <a [routerLink]="['/origins']"
                    routerLinkActive="active">My Origins</a>
            </li>
            <li>
                <a [routerLink]="['/pkgs']"
                    routerLinkActive="active">Search Packages</a>
            </li>
        </ul>
        <h4>Quick Links</h4>
        <ul class="hab-side-nav--list">
            <li>
                <a href="{{ config['docs_url'] }}">Habitat Docs</a>
            </li>
            <li>
                <a href="{{ config['tutorials_url'] }}/getting-started/overview">Getting Started</a>
            </li>
            <li>
                <a href="{{ config['www_url'] }}/about">Why Habitat?</a>
            </li>
        </ul>
    </nav>`
})
export class SideNavComponent {
    @Input() isSignedIn;

    get config() {
        return config;
    }
}
