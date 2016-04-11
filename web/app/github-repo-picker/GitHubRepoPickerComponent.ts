// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {Component} from "angular2/core";
import {List, Map, OrderedSet} from "immutable";

@Component({
    inputs: ["repos", "onOrgSelect", "onRepoSelect", "selectedOrg"],
    selector: "github-repo-picker",
    template: `
    <div class="hab-github-repo-picker">
        <div class="users">
            <h4>Users/Organizations</h4>
            <ul>
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
            <h4>Repositories</h4>
            <ul>
                <li *ngFor="#repo of reposForOrg(selectedOrg)">
                    <a (click)='onRepoSelect(repo.get("full_name"))' href="#">
                        {{repo.get("name")}}
                    </a>
                </li>
            </ul>
        </div>
    </div>`,
})

export class GitHubRepoPickerComponent {
    private selectedOrg;
    private repos;

    get orgs(): OrderedSet<Map<string, any>> {
        return this.repos.map((repo) => repo.get("owner")).toOrderedSet();
    }

    private reposForOrg(org) {
        return this.repos.filter(
            (repo) => repo.getIn(["owner", "login"]) === org);
    }
}
