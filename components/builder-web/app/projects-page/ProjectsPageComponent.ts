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

import {Component, OnInit} from "@angular/core";
import {RouterLink} from "@angular/router";
import {fetchProjects} from "../actions/index";
import {AppStore} from "../AppStore";
import {requireSignIn} from "../util";

@Component({
    directives: [RouterLink],
    template: `
    <div class="hab-projects">
        <div class="page-title">
            <h2>Projects</h2>
            <a class="button create" [routerLink]="['/projects', 'create']">Add Project</a>
        </div>
        <div class="page-body">
            <ul class="hab-projects-list">
                <li *ngIf="projects.size === 0">
                    You do not have any Projects yet.
                </li>
                <li *ngFor="let project of projects">
                    <a [routerLink]="['/projects', project.origin, project.name]" class="hab-item-list" href="#">
                        {{project.origin}} / {{project.name}}
                    </a>
                </li>
            </ul>
        </div>
    </div>`
})

export class ProjectsPageComponent implements OnInit {
    constructor(private store: AppStore) {
        requireSignIn(this);
    }

    get token() {
        return this.store.getState().gitHub.authToken;
    }

    get projects() {
        return this.store.getState().projects.all;
    }

    ngOnInit() {
        this.store.dispatch(fetchProjects(this.token));
    }
}
