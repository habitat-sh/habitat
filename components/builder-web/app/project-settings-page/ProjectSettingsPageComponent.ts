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

import {Component, OnInit} from "angular2/core";
import {AppStore} from "../AppStore";
import {RouteParams} from "angular2/router";
import {ProjectInfoComponent} from "../project-info/ProjectInfoComponent";
import {fetchProject} from "../actions/index";
import {requireSignIn, isProject, projectFromParams} from "../util";
import {setRedirectRoute} from "../actions/index";

// temporary
import {Project} from "../records/Project";
import {Record} from "immutable";

@Component({
    directives: [ProjectInfoComponent],
    template: `
    <div class="hab-project-settings">
      <div class="page-title">
          <h2>Update {{packageName}} settings</h2>
          <p>
              All projects require a path to the plan in the source code repository.
          </p>
      </div>
      <div class="page-body">
        <hab-project-info [project]="project" [ownerAndRepo]="ownerAndRepo"></hab-project-info>
      </div>
    </div>
    `
})

export class ProjectSettingsPageComponent implements OnInit {
    constructor(private routeParams: RouteParams, private store: AppStore) {
        requireSignIn(this);
    }

    get packageName() {
        return this.store.getState().projects.hint["packageName"];
    }

    get ownerAndRepo() {
        if (this.routeParams.params["repo"]) {
            return decodeURIComponent(this.routeParams.params["repo"]);
        } else {
            let parts = this.project["vcs"]["url"].match(/^https?:\/\/.+?\/(.+?)\/(.+?)(?:\.git)?$/);

            if (parts === null) {
                return undefined;
            } else {
                return `${parts[1]}/${parts[2]}`;
            }
        }
    }

    get project() {
        const currentProjectFromState = this.store.getState().projects.current;
        const params = this.routeParams.params;
        let p = projectFromParams(params);

        if (isProject(currentProjectFromState || {}, p)) {
            return currentProjectFromState;
        } else {
            return p;
        }
    }

    get token() {
        return this.store.getState().gitHub.authToken;
    }

    ngOnInit() {
        this.store.dispatch(fetchProject(this.project["id"], this.token, true));

        if (this.routeParams.params["repo"]) {
            this.store.dispatch(setRedirectRoute(["Origin", { origin: this.routeParams.params["origin"] }]));
        } else {
            this.store.dispatch(setRedirectRoute(["ProjectSettings", this.routeParams.params]));
        }
    }
}
