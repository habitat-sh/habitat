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
import { List } from "immutable";
import { fetchMyOrigins, scheduleBuild } from "../actions/index";
import { packageString, releaseToDate, isSignedIn } from "../util";
import { AppStore } from "../AppStore";

@Component({
    selector: "hab-packages-list",
    template: require("./packages-list.component.html")
})

export class PackagesListComponent implements OnInit {
    @Input() errorMessage: string;
    @Input() noPackages: boolean;
    @Input() packages: List<Object>;
    @Input() versions: List<Object>;
    @Input() layout: string;

    constructor(private store: AppStore) {}

    ngOnInit() {
        this.store.dispatch(fetchMyOrigins(this.gitHubAuthToken));
    }

    get gitHubAuthToken() {
        return this.store.getState().gitHub.authToken;
    }

    get myOrigins() {
        return this.store.getState().origins.mine;
    }

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

    get iCanRequestABuild() {
        return isSignedIn() && this.myOrigins.find(org => { return org["name"] === "core"; });
    }

    requestNewBuild(versions) {
        let version = versions[0];

        this.store.dispatch(scheduleBuild(version["origin"], version["name"], this.gitHubAuthToken));
    }
}
