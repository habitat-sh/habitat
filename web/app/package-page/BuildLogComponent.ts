// Copyright:: Copyright (c) 2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import {Component} from "angular2/core";

@Component({
    inputs: ["package"],
    selector: "build-log",
    template: `
    <button class="rebuild" disabled title="You do not have authorization to rebuild this package.">Rebuild</button>
    <pre class="output" *ngIf="package.buildLog" [innerHTML]="package.buildLog"></pre>
    <p *ngIf="!package.buildLog">
        No build log found. This package may have been uploaded manually.
    </p>
    `,
})

export class BuildLogComponent { }