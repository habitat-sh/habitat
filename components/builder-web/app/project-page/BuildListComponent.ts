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

import {Component} from "@angular/core";
import {duration, friendlyTime, packageString} from "../util";

@Component({
    inputs: ["builds", "logs", "project"],
    selector: "build-list",
    template: `
    <p *ngIf="!builds || builds.size === 0">
        No builds found.
    </p>
    <ul *ngIf="builds && builds.size > 0" class="hab-build-list">
        <li *ngFor="let build of builds" class="{{build.status}}">
            <span class="status color">{{build.status}}</span>
            <h1>
                <span class="id color">#{{build.id}}</span>
                {{packageString(ident(build))}}
            </h1>
            <dl>
                <dt>Author</dt>
                <dd>{{build.author}}</dd>
                <dt>Start Time</dt>
                <dd>
                    <span title="{{build.startTime}}">
                        {{friendlyTime(build.startTime)}}
                    </span>
                </dd>
                <dt *ngIf="build.duration">Duration</dt>
                <dd *ngIf="build.duration">{{duration(build.duration)}}</dd>
                <dt *ngIf="build.gitCommitUrl && build.gitCommit">Commit</dt>
                <dd *ngIf="build.gitCommitUrl && build.gitCommit">
                    <a target="_blank" href="{{build.gitCommitUrl}}">
                        {{build.gitCommit}}
                    </a>
                </dd>
            </dl>
            <button *ngIf="logs && logs.get(build.id)" class="rebuild" disabled
                    title="You do not have authorization to rebuild this package.">
                Rebuild
            </button>
            <pre class="output" *ngIf="logs && logs.get(build.id)"
                 [innerHTML]="logs.get(build.id)">
            </pre>
        </li>
    </ul>`,
})

export class BuildListComponent {
    private project;

    private packageString(pkg) { return packageString(pkg); }

    // Come up with an identifier for a build based on the existing package
    // and what happened in the build.
    private ident(build) {
        return Object.assign({
            origin: this.project.origin,
            name: this.project.name,
        },
            {
                version: build.version,
                release: build.release
            });
    }

    private duration(s) { return duration(s); }

    private friendlyTime(t) { return friendlyTime(t); }
}
