// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {Component, OnInit} from "angular2/core";
import {AppStore} from "../AppStore";
import {fetchGitHubOrgs, fetchGitHubRepos, onGitHubOrgSelect,
    onGitHubRepoSelect, setSelectedGitHubOrg} from "../actions/index";
import {GitHubRepoPickerComponent} from
    "../github-repo-picker/GitHubRepoPickerComponent";
import {requireSignIn} from "../util";

@Component({
    directives: [GitHubRepoPickerComponent],
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

    constructor(private store: AppStore) {
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
            this.store.dispatch(onGitHubRepoSelect(repo));
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
}
