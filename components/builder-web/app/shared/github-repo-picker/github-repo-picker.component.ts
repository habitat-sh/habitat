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
import { resetRedirectRoute } from "../../actions/index";

@Component({
  selector: "hab-github-repo-picker",
  template: require("./github-repo-picker.component.html")
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
    this.clickFetchGitHubRepos = () => {
      return false;
    };

    this.orgSelect = (org, username) => {
      this.selectedOrg = org;
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
