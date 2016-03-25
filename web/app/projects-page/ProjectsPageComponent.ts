// Copyright:: Copyright (c) 2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import {AppStore} from "../AppStore";
import {Component, OnInit} from "angular2/core";
import {RouterLink} from "angular2/router";
import {fetchProjects} from "../actions/index";

@Component({
    directives: [RouterLink],
    template: `
    <div class="hab-projects">
        <h2>Projects</h2>
        <a class="button create" [routerLink]="['ProjectCreate']">Add Project</a>
        <hr>
        <ul>
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
    </div>`
})

export class ProjectsPageComponent implements OnInit {
    constructor(private store: AppStore) {}

    get projects() {
        return this.store.getState().projects.all;
    }

    ngOnInit() {
        this.store.dispatch(fetchProjects());
    }
}
