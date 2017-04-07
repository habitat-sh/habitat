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

import {Component, OnInit} from "@angular/core";
import {AppStore} from "../AppStore";
import {fetchGitHubOrgs, fetchGitHubRepos,
        onGitHubOrgSelect, setSelectedGitHubOrg, resetRedirectRoute} from "../actions/index";
import {requireSignIn} from "../util";
import {Router} from "@angular/router";

@Component({
    template: `
    <div class="hab-scm-repos">
      <div class="page-title">
          <h2>
              GitHub Repositories
              <span *ngIf="gitHub.username">
                  for <em>{{gitHub.username}}</em>
              </span>
          </h2>
          <p>
              Select a repository that contains a plan.sh file. In the next step,
              you'll set the path to the plan file.
          </p>
      </div>
      <div class="page-body">
          <github-repo-picker [areOrgsLoading]="gitHub.ui.orgs.loading"
                              [areReposLoading]="gitHub.ui.repos.loading"
                              [fetchGitHubOrgs]="fetchGitHubOrgs"
                              [fetchGitHubRepos]="fetchGitHubRepos"
                              [orgs]="gitHub.orgs"
                              [repos]="gitHub.repos"
                              [onOrgSelect]="onOrgSelect"
                              [onRepoSelect]="onRepoSelect"
                              [selectedOrg]="gitHub.selectedOrg"
                              [user]="user">
          </github-repo-picker>
      </div>
    </div>`
})

export class SCMReposPageComponent implements OnInit {
    private fetchGitHubOrgs: Function;
    private fetchGitHubRepos: Function;
    private onOrgSelect: Function;
    private onRepoSelect: Function;

    constructor(private store: AppStore, private router: Router) {
        this.fetchGitHubOrgs = () => {
            this.store.dispatch(fetchGitHubOrgs());
            return false;
        };

        this.fetchGitHubRepos = (org, page, username) => {
            this.store.dispatch(fetchGitHubRepos(org, page, username));
            return false;
        };

        this.onOrgSelect = (org, username) => {
            this.store.dispatch(onGitHubOrgSelect(org, username));
            return false;
        };

        this.onRepoSelect = (repo) => {
            if (typeof this.redirectRoute === "object" && this.redirectRoute.length) {
                this.router.navigate(this.redirectRoute, {queryParams: {repo: encodeURIComponent(repo)}});
                this.store.dispatch(resetRedirectRoute());
            } else {
                this.router.navigate(["/projects", "create"], {queryParams: {repo: encodeURIComponent(repo)}});
            }

            return false;
        };
    }

    ngOnInit() {
        requireSignIn(this);
    }

    get gitHub() {
        return this.store.getState().gitHub;
    }

    get user() {
        return this.store.getState().users.current.gitHub;
    }

    get redirectRoute() {
        return this.store.getState().router.redirectRoute;
    }
}
