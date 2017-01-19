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

import {Component, OnInit, OnDestroy} from "@angular/core";
import {AppStore} from "../AppStore";
import {ActivatedRoute} from "@angular/router";
import {ProjectInfoComponent} from "../project-info/ProjectInfoComponent";
import {fetchProject} from "../actions/index";
import {requireSignIn, isProject, projectFromParams} from "../util";
import {setRedirectRoute} from "../actions/index";
import {Subscription} from "rxjs/Subscription";

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

export class ProjectSettingsPageComponent implements OnInit, OnDestroy {
    private querySub: Subscription;
    private routeSub: Subscription;
    private repo: string;
    private origin: string;
    private name: string;

    constructor(private route: ActivatedRoute, private store: AppStore) {
        requireSignIn(this);
        this.querySub = route.queryParams.subscribe(params => {
            this.repo = params["repo"];
        });
        this.routeSub = route.params.subscribe(params => {
            this.origin = params["origin"];
            this.name = params["name"];
        });
    }

    get packageName() {
        return this.store.getState().projects.hint["packageName"];
    }

    get ownerAndRepo() {
        if (this.repo) {
            return decodeURIComponent(this.repo);
        } else {
            if (this.project && this.project["vcs"] && this.project["vcs"]["url"]) {
                let parts = this.project["vcs"]["url"].match(/^https?:\/\/.+?\/(.+?)\/(.+?)(?:\.git)?$/);

                if (parts === null) {
                    return undefined;
                } else {
                    return `${parts[1]}/${parts[2]}`;
                }
            } else {
                return undefined;
            }
        }
    }

    get project() {
        const currentProjectFromState = this.store.getState().projects.current;
        const params = {
            origin: this.origin,
            name: this.name
        };

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

    ngOnDestroy() {
        this.querySub.unsubscribe();
        this.routeSub.unsubscribe();
    }

    ngOnInit() {
        this.store.dispatch(fetchProject(this.project["id"], this.token, true));

        if (this.repo) {
            this.store.dispatch(setRedirectRoute(["/origins", this.origin]));
        } else {
            this.store.dispatch(setRedirectRoute(["/projects", this.origin, this.name, "settings"]));
        }
    }
}
