// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {Component} from "angular2/core";
import {RouterLink} from "angular2/router";
import {duration, friendlyTime, packageString} from "../util";

@Component({
    directives: [RouterLink],
    inputs: ["builds", "logs", "project"],
    selector: "build-list",
    template: `
    <p *ngIf="!builds || builds.size === 0">
        No builds found.
    </p>
    <ul *ngIf="builds && builds.size > 0" class="hab-build-list">
        <li *ngFor="#build of builds" class="{{build.status}}">
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
