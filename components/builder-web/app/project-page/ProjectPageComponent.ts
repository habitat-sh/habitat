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

import {AppStore} from "../AppStore";
import {Component, OnInit, OnDestroy} from "@angular/core";
import {RouterLink, ActivatedRoute} from "@angular/router";
import {TabComponent} from "../TabComponent";
import {TabsComponent} from "../TabsComponent";
import {fetchBuilds, fetchProject, deleteProject} from "../actions/index";
import {friendlyTime, requireSignIn} from "../util";
import {Subscription} from "rxjs/Subscription";

@Component({
    template: `
    <div *ngIf="!project.ui.loading" class="hab-project">
        <div class="page-title" *ngIf="!project.ui.exists">
            <h2>Project Not Found</h2>
            <p>The project you're looking for is not currently available. We apologize for the inconvenience and it should be back up soon.</p>
        </div>
        <div *ngIf="project.ui.exists">
            <header class="page-title">
                <h2>{{project.id}}</h2>
                <h4 *ngIf="project.latestBuild">
                    <a [routerLink]="['/pkgs',
                        project.id.split('/')[0],
                        project.id.split('/')[1],
                        project.latestBuild.version,
                        project.latestBuild.release
                    ]">
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
            <hab-tabs>
                <hab-tab tabTitle="Info">
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
                                        hab install {{project.id}}
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
                                <li>
                                  <div class="submit">
                                      <button (click)="deleteProject()">
                                          Delete Project
                                      </button>
                                  </div>
                                </li>
                            </ul>
                        </div>
                    </div>
                </hab-tab>
                <hab-tab tabTitle="Builds">
                    <div class="builds page-body">
                        <div class="l">
                            <hab-build-list [project]="project"
                                        [builds]="project.builds"
                                        [logs]="project.buildLogs">
                            </hab-build-list>
                        </div>
                        <div class="r">
                            <ul>
                                <li>
                                    <h4>Build Command</h4>
                                    <div class="build-command">
                                        hab install {{project.id}}
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
                </hab-tab>
            </hab-tabs>
        </div>
    </div>`
})

export class ProjectPageComponent implements OnInit, OnDestroy {
    private sub: Subscription;
    private origin: string;
    private name: string;

    constructor(private route: ActivatedRoute, private store: AppStore) {
        requireSignIn(this);

        this.sub = route.params.subscribe(params => {
            this.origin = params["origin"];
            this.name = params["name"];
        });
    }

    deleteProject() {
        this.store.dispatch(deleteProject(this.id, this.token, this.origin));
    }

    get project() {
        return this.store.getState().projects.current;
    }

    get token() {
        return this.store.getState().gitHub.authToken;
    }

    get id() {
        return `${this.origin}/${this.name}`;
    }

    ngOnDestroy() {
        this.sub.unsubscribe();
    }

    ngOnInit() {
        this.store.dispatch(fetchProject(this.id, this.token, true));
        // leaving this commented out on purpose as a reminder to make it work
        // again once the API returns build information
        // this.store.dispatch(fetchBuilds(this.routeParams.params));
    }

    friendlyTime(t) { return friendlyTime(t); }
}
