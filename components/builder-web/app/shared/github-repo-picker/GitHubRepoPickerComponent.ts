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

import { Component, Input, OnInit, Output, EventEmitter } from "@angular/core";
import { List, Map, OrderedSet } from "immutable";
import { AppStore } from "../../AppStore";
import { GitHubRepo } from "../../github/repo/shared/github-repo.model";
import {fetchGitHubOrgs, fetchGitHubRepos,
  onGitHubOrgSelect, setSelectedGitHubOrg, resetRedirectRoute} from "../../actions/index";

@Component({
  selector: "hab-github-repo-picker",
  template: `
  <div class="hab-github-repo-picker">
    <div class="users">
      <h4>
        <hab-icon symbol="loading" [class.spinning]="areOrgsLoading"></hab-icon>
        Users/Organizations
      </h4>
      <ul>
        <li>
          <a (click)='orgSelect(user.get("login"), user.get("login"))'
             href="#"
             [class.active]='user.get("login") === selectedOrg'>
            <img height=16 width=16 src='{{user.get("avatar_url")}}?s=32'>
            {{user.get("login")}}
          </a>
        </li>
        <li *ngFor="let org of orgs">
          <a (click)='orgSelect(org.get("login"))' href="#"
             [class.active]='org.get("login") === selectedOrg'>
            <img height=16 width=16 src='{{org.get("avatar_url")}}?s=32'>
            {{org.get("login")}}
          </a>
        </li>
      </ul>
    </div>
    <div class="repos">
      <h4>
        <hab-icon symbol="loading" [class.spinning]="areReposLoading"></hab-icon>
        Repositories
      </h4>
      <label>Filter: <input [(ngModel)]="filter.name"></label>
      <ul>
        <li *ngIf="repos.size === 0 && selectedOrg && !areReposLoading">
          No repositories found in '{{selectedOrg}}'
        </li>
        <li *ngFor="let repo of repos | habGitHubRepoFilter:filter:'name'">
          <a (click)='repoSelect(repo.get("full_name"))' href="#">
            {{repo.get("name")}}
          </a>
        </li>
      </ul>
    </div>
  </div>`,
})

export class GitHubRepoPickerComponent implements OnInit {
  @Output() repoSelected: EventEmitter<string> = new EventEmitter();
  @Output() repoSelectCanceled: EventEmitter<boolean> = new EventEmitter();

  selectedOrg: string;
  filter: GitHubRepo = new GitHubRepo();
  clickFetchGitHubRepos: Function;
  orgSelect: Function;
  repoSelect: Function;
  repoSelectCancel: Function;

  constructor(private store: AppStore) {
    this.store.dispatch(fetchGitHubOrgs());
    this.clickFetchGitHubRepos = () => {
      this.store.dispatch(fetchGitHubRepos(
        this.selectedOrg, 1,
        this.selectedOrg === this.user.get("login") ?
          this.user.get("login") : undefined
      ));
      return false;
    };

    this.orgSelect = (org, username) => {
      this.selectedOrg = org;
      this.store.dispatch(onGitHubOrgSelect(org, username));
      return false;
    };

    this.repoSelect = (repo) => {
      this.repoSelected.emit(repo);
      return false;
    };

    this.repoSelectCancel = () => {
      this.repoSelectCanceled.emit(false);
      return false;
    };
  }

  public ngOnInit() {
    // this.fetchGitHubOrgs();
  }

  get gitHub() {
    return this.store.getState().gitHub;
  }

  get user() {
    return this.store.getState().users.current.gitHub;
  }

  get areReposLoading() {
    return this.gitHub.ui.repos.loading;
  }

  get areOrgsLoading() {
    return this.gitHub.ui.orgs.loading;
  }

  get orgs() {
    return this.gitHub.orgs;
  }

  get repos() {
    return this.gitHub.repos;
  }
}