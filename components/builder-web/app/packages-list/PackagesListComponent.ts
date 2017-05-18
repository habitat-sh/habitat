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

import { Component, Input } from "@angular/core";
import { List } from "immutable";
import { packageString, releaseToDate } from "../util";

@Component({
    selector: "hab-packages-list",
    template: require("./packages-list.component.html")
})

export class PackagesListComponent {
    @Input() errorMessage: string;
    @Input() noPackages: boolean;
    @Input() packages: List<Object>;
    @Input() versions: List<Object>;
    @Input() layout: string;

    routeFor(pkg) {
        let link = ["/pkgs", pkg.origin];

        [pkg.name, pkg.version, pkg.release].forEach((p) => {
            if (p) {
                link.push(p);
            }
        });

        return link;
    }

    packageString(pkg) {
        return packageString(pkg);
    }

    releaseToDate(release) {
        return releaseToDate(release);
    }
}
