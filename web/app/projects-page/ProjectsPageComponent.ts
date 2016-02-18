// Copyright:: Copyright (c) 2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import {AppStore} from "../AppStore";
import {Component, OnInit} from "angular2/core";
import {fetchProjects} from "../actions";

@Component({
    template: `
    <div class="bldr-projects">
        <h2>Projects</h2>
        <ul>
            <li *ngIf="projects.size === 0">
                You do not have any Projects yet. Why not
                <a href="#">create one</a>?
            </li>
            <li *ngFor="#project of projects">
                <a class="bldr-item-list" href="#">
                    {{project.derivation}} / {{project.name}}
                </a>
            </li>
        </ul>
    </div>`
})

export class ProjectsPageComponent implements OnInit {
    constructor(private store: AppStore) {}

    get projects() {
        return this.store.getState().projects;
    }

    ngOnInit() {
        this.store.dispatch(fetchProjects());
    }
}