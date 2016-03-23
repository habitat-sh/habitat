// Copyright:: Copyright (c) 2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import {AppStore} from "../AppStore";
import {BuildListComponent} from "./BuildListComponent";
import {Component, OnInit} from "angular2/core";
import {RouterLink, RouteParams} from "angular2/router";
import {TabComponent} from "../TabComponent";
import {TabsComponent} from "../TabsComponent";
import {fetchBuilds, fetchProject} from "../actions/index";
import {friendlyTime} from "../util";

@Component({
    directives: [BuildListComponent, RouterLink, TabsComponent, TabComponent],
    template: `
    <div *ngIf="!project.ui.loading" class="hab-project">
        <div *ngIf="!project.ui.exists">
            <h2>Project Not Found</h2>
        </div>
        <div *ngIf="project.ui.exists">
            <header>
                <h2>{{project.origin}} / {{project.name}}</h2>
                <h3 *ngIf="project.latestBuild">
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
                </h3>
            </header>
            <tabs>
                <tab tabTitle="Info">
                    <div class="info">
                        <div class="l">
                            <h4>Description</h4>
                            <p>{{project.description}}</p>
                            <h4>Source Repository</h4>
                            <a href="{{project.sourceRepository.url}}"
                               target="_blank">
                                {{project.sourceRepository.slug}}
                            </a>
                        </div>
                        <div class="r">
                            <h4>Build Command</h4>
                            <div class="build-command">
                                hab install {{project.origin}}/{{project.name}}
                                <a (click)="false" href="#">⎘</a>
                            </div>
                            <h4>Maintainer</h4>
                            {{project.maintainer.name}}
                            <a href="mailto:{{project.maintainer.email}}">{{project.maintainer.email}}</a>
                        </div>
                    </div>
                </tab>
                <tab tabTitle="Builds">
                    <div class="builds">
                        <div class="l">
                            <build-list [project]="project"
                                        [builds]="project.builds"
                                        [logs]="project.buildLogs">
                            </build-list>
                        </div>
                        <div class="r">
                            <h4>Build Command</h4>
                            <div class="build-command">
                                hab install {{project.origin}}/{{project.name}}
                                <a (click)="false" href="#">⎘</a>
                            </div>
                            <h4>Build Dependencies</h4>
                            <p>None</p>
                            <h4>Runtime Dependencies</h4>
                            <p>None</p>
                            <h4>Source URL</h4>
                            <a class="sourceUrl"
                               title="{{project.sourceUrl}}"
                               href="{{project.sourceUrl}}">
                                {{project.sourceUrl}}
                            </a>
                        </div>
                    </div>
                </tab>
            </tabs>
        </div>
    </div>`
})

export class ProjectPageComponent implements OnInit {
    constructor(private routeParams: RouteParams, private store: AppStore) {}

    get project() {
        return this.store.getState().projects.current;
    }

    ngOnInit() {
        this.store.dispatch(fetchProject(this.routeParams.params));
        this.store.dispatch(fetchBuilds(this.routeParams.params));
    }

    private friendlyTime(t) { return friendlyTime(t); }
}
