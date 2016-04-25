// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {Component, OnInit} from "angular2/core";
import {RouterLink} from "angular2/router";
import {fetchProjects} from "../actions/index";
import {AppStore} from "../AppStore";
import {requireSignIn} from "../util";

@Component({
    directives: [RouterLink],
    template: `
    <div class="hab-projects">
        <div class="page-title">
            <h2>Projects</h2>
            <a class="button create" [routerLink]="['ProjectCreate']">Add Project</a>
        </div>
        <div class="page-body">
            <ul class="hab-projects-list">
                <li *ngIf="projects.size === 0">
                    You do not have any Projects yet. Why not
                    <a [routerLink]="['ProjectCreate']">create one</a>?
                </li>
                <li *ngFor="#project of projects">
                    <a [routerLink]="['Project', { origin: project.origin, name: project.name }]" class="hab-item-list" href="#">
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

    get projects() {
        return this.store.getState().projects.all;
    }

    ngOnInit() {
        this.store.dispatch(fetchProjects());
    }
}
