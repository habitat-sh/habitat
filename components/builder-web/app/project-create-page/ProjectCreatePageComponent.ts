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
import {requireSignIn} from "../util";
import {RouteParams} from "angular2/router";
import {AppStore} from "../AppStore";
import {ProjectInfoComponent} from "../project-info/ProjectInfoComponent";

@Component({
    directives: [ProjectInfoComponent],
    template: `
    <div class="hab-project-create">
      <div class="page-title">
          <h2>Link {{packageName}} to a repo</h2>
          <p>
              All projects require a path to the plan in the source code repository.
          </p>
      </div>
      <div class="page-body">
        <hab-project-info [ownerAndRepo]="ownerAndRepo"></hab-project-info>
      </div>
    </div>`
})

export class ProjectCreatePageComponent implements OnInit {
    constructor(private routeParams: RouteParams, private store: AppStore) {}

    get repo() {
        if (this.routeParams.params["repo"]) {
            return this.routeParams.params["repo"];
        } else {
            return undefined;
        }
    }

    get packageName() {
        return this.store.getState().projects.hint["packageName"];
    }

    get ownerAndRepo() {
        return this.repo ? decodeURIComponent(this.repo) : this.repo;
    }

    public ngOnInit() {
        requireSignIn(this);
    }
}
