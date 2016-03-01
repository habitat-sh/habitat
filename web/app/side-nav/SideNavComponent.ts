// Copyright:: Copyright (c) 2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import {Component, OnInit} from "angular2/core";
import {RouterLink} from "angular2/router";

@Component({
    directives: [RouterLink],
    selector: "bldr-side-nav",
    template: `
    <div class="bldr-side-nav">
        <ul>
            <li><a [routerLink]="['Projects']">Projects</a></li>
        </ul>
        <hr>
        <h4>Public Packages</h4>
        <ul>
            <li><a [routerLink]="['Explore']">Explore</a></li>
            <li><a [routerLink]="['Packages']">All Packages</a></li>
            <li><a [routerLink]="['Packages', { filter: 'mine' }]">My Packages</a></li>
        </ul>
        <h4>Linked Accounts</h4>
        <ul>
            <li><a [routerLink]="['SCMRepos']">GitHub Repos</a></li>
        </ul>
    </div>`
})

export class SideNavComponent { }
