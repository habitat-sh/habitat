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

import {Component} from "@angular/core";
import {RouterLink} from "@angular/router";
import {isPackage, packageString} from "../util";

@Component({
    inputs: ["currentPackage", "packages"],
    directives: [RouterLink],
    selector: "package-list",
    template: `
    <ul class="hab-package-list">
        <li *ngIf="!packages || packages.length === 0">None</li>
        <li *ngFor="let pkg of packages">
            <a [ngClass]="{ active: isPackage(currentPackage, pkg) }" [routerLink]="['/pkgs', pkg.origin, pkg.name, pkg.version, pkg.release]">
                {{packageString(pkg)}}
            </a>
        </li>
    </ul>`,
})

export class PackageListComponent {
    private currentPackage: Object;
    private packages: Array<Object>;
    private isPackage(x, y) { return isPackage(x, y); }
    private packageString(pkg) { return packageString(pkg); }
}
