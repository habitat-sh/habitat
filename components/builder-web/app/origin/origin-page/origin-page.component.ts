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

import { Component, OnInit, OnDestroy } from "@angular/core";
import { RouterLink, ActivatedRoute } from "@angular/router";
import { Subscription } from "rxjs/Subscription";
import { AppStore } from "../../AppStore";
import { FeatureFlags } from "../../Privilege";
import {
    fetchOrigin, fetchOriginInvitations, fetchOriginMembers,
    inviteUserToOrigin,
    filterPackagesBy, fetchMyOrigins,
    setProjectHint, requestRoute, setCurrentProject, getUniquePackages, fetchIntegrations
} from "../../actions";
import config from "../../config";
import { Origin } from "../../records/Origin";
import { requireSignIn, packageString } from "../../util";

export enum ProjectStatus {
    Connect,
    Settings,
    Lacking
}

@Component({
    template: require("./origin-page.component.html")
})

export class OriginPageComponent implements OnInit, OnDestroy {
    loadPackages: Function;
    perPage: number = 50;
    projectStatus = ProjectStatus;
    sub: Subscription;
    origin;

    constructor(private route: ActivatedRoute, private store: AppStore) {
        this.sub = this.route.params.subscribe(params => {
            this.origin = Origin({ name: params["origin"] });
        });
    }

    ngOnInit() {
        requireSignIn(this);
        this.store.dispatch(fetchOrigin(this.origin.name));
        this.store.dispatch(fetchMyOrigins(this.gitHubAuthToken));
        this.store.dispatch(fetchOriginMembers(
            this.origin.name, this.gitHubAuthToken
        ));
        this.store.dispatch(fetchOriginInvitations(
            this.origin.name, this.gitHubAuthToken
        ));
        this.getPackages();
        this.loadPackages = this.getPackages.bind(this);
        this.store.dispatch(fetchIntegrations(
            this.origin.name, this.gitHubAuthToken
        ));
    }

    ngOnDestroy() {
        this.sub.unsubscribe();
    }

    get navLinks() {
        // ED TODO: Uncomment settings when the privacy api endpoint is implemented
        let links = ["packages", "keys", "members"];
        let flags = this.store.getState().featureFlags.current;

        if (flags.get("integrations")) {
          links.push("integrations");
        }
        if (flags.get("settings")) {
          links.push("settings");
        }

        return links;
    }

    get features() {
        return this.store.getState().users.current.flags;
    }

    get docsUrl() {
        return config["docs_url"];
    }

    get gitHubAuthToken() {
        return this.store.getState().gitHub.authToken;
    }

    get ui() {
        return this.store.getState().origins.ui.current;
    }

    get totalCount() {
        return this.store.getState().packages.totalCount;
    }

    get myOrigins() {
        return this.store.getState().origins.mine;
    }

    get iAmPartOfThisOrigin() {
        return !!this.myOrigins.find(org => {
            return org["name"] === this.origin.name;
        });
    }

    linkToRepo(p): boolean {
        this.store.dispatch(setProjectHint({
            originName: p.origin,
            packageName: p.name
        }));
        this.store.dispatch(requestRoute(["/projects", "create"]));
        return false;
    }

    projectSettings(p): boolean {
        this.store.dispatch(setProjectHint({
            originName: p.origin,
            packageName: p.name
        }));
        this.store.dispatch(setCurrentProject(this.projectForPackage(p)));
        this.store.dispatch(requestRoute(["/projects", p.origin, p.name, "settings"]));
        return false;
    }

    projectForPackage(p) {
        let proj = this.store.getState().projects.added.find(proj => {
            return proj["id"] === this.projectId(p);
        });

        if (proj) {
            if (proj["vcs"] && proj["vcs"]["url"]) {
                return ProjectStatus.Settings;
            } else {
                return ProjectStatus.Lacking;
            }
        } else {
            return ProjectStatus.Connect;
        }
    }

    getPackages() {
        this.store.dispatch(getUniquePackages(this.origin.name, 0, this.gitHubAuthToken));
    }

    fetchMorePackages() {
        this.store.dispatch(getUniquePackages(
            this.origin.name,
            this.store.getState().packages.nextRange,
            this.gitHubAuthToken
        ));
        return false;
    }

    private projectId(p) {
        return `${p["origin"]}/${p["name"]}`;
    }
}
