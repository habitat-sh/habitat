// Copyright:: Copyright (c) 2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import {AppStore} from "../AppStore";
import {Component, OnInit} from "angular2/core";
import {GitHubRepoPickerComponent} from "../github-repo-picker/GitHubRepoPickerComponent";
import {fetchGitHubRepos, onGitHubRepoSelect, setSelectedGitHubOrg}
    from "../actions/index";

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
          <github-repo-picker [repos]="gitHub.repos"
                              [onOrgSelect]="onOrgSelect"
                              [onRepoSelect]="onRepoSelect"
                              [selectedOrg]="gitHub.selectedOrg">
          </github-repo-picker>
          <p *ngIf="!gitHub.isLinked">
              You do not have a linked GitHub account.
              <a href="#">Try linking one now</a>.
          </p>
          <p *ngIf="gitHub.repos.size === 0">
              You have no GitHub repositories. You might need to
              <a target="_blank" href="https://github.com/new">create one on GitHub</a>.
          </p>
      </div>
    </div>`
})

export class SCMReposPageComponent implements OnInit {
    private onOrgSelect: Function;
    private onRepoSelect: Function;

    constructor(private store: AppStore) {
        this.onOrgSelect = (org) => {
            this.store.dispatch(setSelectedGitHubOrg(org));
            return false;
        };

        this.onRepoSelect = (repo) => {
            this.store.dispatch(onGitHubRepoSelect(repo));
            return false;
        };
    }

    ngOnInit() {
        this.store.dispatch(fetchGitHubRepos());
    }

    get gitHub() {
        return this.store.getState().gitHub;
    }
}
