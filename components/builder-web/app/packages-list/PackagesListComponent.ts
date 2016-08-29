// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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

import {Component, Input} from "@angular/core";
import {List} from "immutable";
import {RouterLink} from "@angular/router";

@Component({
    selector: "hab-packages-list",
    directives: [RouterLink],
    template: `
    <ul class="hab-packages-plan-list">
        <div *ngIf="noPackages">
            <p>
                No packages found.
                <span *ngIf="errorMessage">
                    Error: {{errorMessage}}
                </span>
            </p>
        </div>
        <li class="hab-packages-package" *ngFor="let pkg of packages">
            <a [routerLink]="['/pkgs', pkg.origin,
                                      pkg.name,
                                      pkg.version,
                                      pkg.release]">
                <div class="item-title">
                    <h3>{{pkg.origin}} / {{pkg.name}}</h3>
                    <h4>{{pkg.version}} / {{pkg.release}}</h4>
                </div>
                <div class="item-info">
                    <span class="count" *ngIf="pkg.starCount">
                        <!-- TODO: import octicons -->
                        <span class="octicon octicon-star"></span>
                        {{pkg.starCount}}
                    </span>
                    <img src="/node_modules/octicons/svg/chevron-right.svg" />
                </div>
            </a>
        </li>
    </ul>
    `
})

export class PackagesListComponent {
    @Input() noPackages: boolean;
    @Input() packages: List<Object>;
    @Input() errorMessage: string;
}
