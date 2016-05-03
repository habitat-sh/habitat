// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {OnInit, Component} from "angular2/core";
import {List, Map, OrderedSet} from "immutable";
import {SpinnerComponent} from "../SpinnerComponent";

@Component({
    directives: [SpinnerComponent],
    inputs: ["areOrgsLoading", "areReposLoading", "fetchGitHubOrgs",
        "fetchGitHubRepos", "onOrgSelect", "onRepoSelect", "orgs", "repos",
        "selectedOrg", "user"],
    selector: "github-repo-picker",
    template: `
    <div class="hab-github-repo-picker">
        <div class="users">
            <h4>
                <hab-spinner [onClick]="fetchGitHubOrgs"
                             [isSpinning]="areOrgsLoading">
                </hab-spinner>
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
                <li *ngFor="#org of orgs">
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
                <hab-spinner [onClick]="clickFetchGitHubRepos"
                             [isSpinning]="areReposLoading">
                </hab-spinner>
                Repositories
            </h4>
            <ul>
                <li *ngIf="repos.size === 0 && selectedOrg && !areReposLoading">
                    No repositories found in '{{selectedOrg}}'
                </li>
                <li *ngFor="#repo of repos">
                    <a (click)='onRepoSelect(repo.get("full_name"))' href="#">
                        {{repo.get("name")}}
                    </a>
                </li>
            </ul>
        </div>
    </div>`,
})

export class GitHubRepoPickerComponent implements OnInit {
    private clickFetchGitHubRepos;
    private fetchGitHubOrgs: Function;
    private fetchGitHubRepos: Function;
    private selectedOrg: String;
    private user;

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
