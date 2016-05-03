// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {AppStore} from "../AppStore";
import {BuildListComponent} from "./BuildListComponent";
import {Component, OnInit} from "angular2/core";
import {RouterLink, RouteParams} from "angular2/router";
import {TabComponent} from "../TabComponent";
import {TabsComponent} from "../TabsComponent";
import {fetchBuilds, fetchProject} from "../actions/index";
import {friendlyTime, requireSignIn} from "../util";

@Component({
    directives: [BuildListComponent, RouterLink, TabsComponent, TabComponent],
    template: `
    <div *ngIf="!project.ui.loading" class="hab-project">
        <div class="page-title" *ngIf="!project.ui.exists">
            <h2>Project Not Found</h2>
            <p>The project you're looking for is not currently available. We apologize for the inconvenience and it should be back up soon.</p>
        </div>
        <div *ngIf="project.ui.exists">
            <header class="page-title">
                <h2>{{project.origin}} / {{project.name}}</h2>
                <h4 *ngIf="project.latestBuild">
                    <a [routerLink]="['Package', {
                        origin: project.origin,
                        name: project.name,
                        version: project.latestBuild.version,
                        release: project.latestBuild.release
                    }]">
                        {{project.latestBuild.version}} /
                        {{project.latestBuild.release}}
                    </a>
                    <span class="lastBuild">
                        Last build
                        <span title="{{project.latestBuild.finishTime}}">
                            {{friendlyTime(project.latestBuild.finishTime)}}
                        </span>
                    </span>
                </h4>
            </header>
            <tabs>
                <tab tabTitle="Info">
                    <div class="info page-body">
                        <div class="l">
                            <h4>Description</h4>
                            <p>{{project.description}}</p>
                        </div>
                        <div class="r">
                            <ul>
                                <li>
                                    <h4>Build Command</h4>
                                    <div class="build-command">
                                        hab install {{project.origin}}/{{project.name}}
                                        <a (click)="false" href="#">⎘</a>
                                    </div>
                                </li>
                                <li>
                                    <h4>Maintainer</h4>
                                    {{project.maintainer.name}}<br>
                                    <a href="mailto:{{project.maintainer.email}}">{{project.maintainer.email}}</a>
                                </li>
                                <li>
                                    <h4>Source Repository</h4>
                                    <a href="{{project.sourceRepository.url}}"
                                       target="_blank">
                                        {{project.sourceRepository.slug}}
                                    </a>
                                </li>
                            </ul>
                        </div>
                    </div>
                </tab>
                <tab tabTitle="Builds">
                    <div class="builds page-body">
                        <div class="l">
                            <build-list [project]="project"
                                        [builds]="project.builds"
                                        [logs]="project.buildLogs">
                            </build-list>
                        </div>
                        <div class="r">
                            <ul>
                                <li>
                                    <h4>Build Command</h4>
                                    <div class="build-command">
                                        hab install {{project.origin}}/{{project.name}}
                                        <a (click)="false" href="#">⎘</a>
                                    </div>
                                </li>
                                <li>
                                    <h4>Build Dependencies</h4>
                                    <p>None</p>
                                </li>
                                <li>
                                    <h4>Runtime Dependencies</h4>
                                    <p>None</p>
                                    </li>
                                <li>
                                    <h4>Source URL</h4>
                                    <a class="sourceUrl"
                                       title="{{project.sourceUrl}}"
                                       href="{{project.sourceUrl}}">
                                        {{project.sourceUrl}}
                                    </a>
                                </li>
                            </ul>
                        </div>
                    </div>
                </tab>
            </tabs>
        </div>
    </div>`
})

export class ProjectPageComponent implements OnInit {
    constructor(private routeParams: RouteParams, private store: AppStore) {
        requireSignIn(this);
    }

    get project() {
        return this.store.getState().projects.current;
    }

    ngOnInit() {
        this.store.dispatch(fetchProject(this.routeParams.params));
        this.store.dispatch(fetchBuilds(this.routeParams.params));
    }

    private friendlyTime(t) { return friendlyTime(t); }
}
