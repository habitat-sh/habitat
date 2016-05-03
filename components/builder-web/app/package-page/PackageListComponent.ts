// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {Component} from "angular2/core";
import {RouterLink} from "angular2/router";
import {isPackage, packageString} from "../util";

@Component({
    inputs: ["currentPackage", "packages"],
    directives: [RouterLink],
    selector: "package-list",
    template: `
    <ul class="hab-package-list">
        <li *ngIf="!packages || packages.length === 0">None</li>
        <li *ngFor="#pkg of packages">
            <a [ngClass]="{ active: isPackage(currentPackage, pkg) }" [routerLink]="['Package', { origin: pkg.origin,
                                                                                                  name: pkg.name,
                                                                                                  version: pkg.version,
                                                                                                  release: pkg.release }]">
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
