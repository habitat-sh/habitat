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
import { List, Map, OrderedSet } from "immutable";
import { GitHubRepo } from "../github-repos/shared/github-repos.model";

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
                    <a (click)='onOrgSelect(user.get("login"), user.get("login"))'
                       href="#"
                       [class.active]='user.get("login") === selectedOrg'>
                        <img height=16 width=16 src='{{user.get("avatar_url")}}?s=32'>
                        {{user.get("login")}}
                    </a>
                </li>
                <li *ngFor="let org of orgs">
                    <a (click)='onOrgSelect(org.get("login"))' href="#"
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
                    <a (click)='onRepoSelect(repo.get("full_name"))' href="#">
                        {{repo.get("name")}}
                    </a>
                </li>
            </ul>
        </div>
    </div>`,
})

export class GitHubRepoPickerComponent implements OnInit {
    @Input() areOrgsLoading;
    @Input() areReposLoading;
    @Input() fetchGitHubOrgs: Function;
    @Input() fetchGitHubRepos: Function;
    @Input() onOrgSelect;
    @Input() onRepoSelect;
    @Input() orgs;
    @Input() repos;
    @Input() selectedOrg: string;
    @Input() user;

    filter: GitHubRepo = new GitHubRepo();
    clickFetchGitHubRepos: Function;

    constructor() {
        this.clickFetchGitHubRepos = () => {
            this.fetchGitHubRepos(
                this.selectedOrg, 1,
                this.selectedOrg === this.user.get("login") ?
                    this.user.get("login") : undefined
            );
            return false;
        };
    }

    public ngOnInit() {
        this.fetchGitHubOrgs();
    }
}
