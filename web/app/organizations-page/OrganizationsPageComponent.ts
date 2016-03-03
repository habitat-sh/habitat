// Copyright:: Copyright (c) 2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import {AppStore} from "../AppStore";
import {Component} from "angular2/core";
import {List} from "immutable";

@Component({
    template: `
    <div class="bldr-organizations">
        <h2>Organizations</h2>
        <hr>
        <div *ngIf="orgs.size === 0">
            <h3 class="hero">
                You don't currently have any organizations, let's add one now.
            </h3>
            <form>
                <p>
                    Create an organization, then start adding projects and users.
                </p>
                <p><button>Add Organization</button></p>
            </form>
            <div class="info">
                <p>Organizations allow you to do the following:</p>
                <ul>
                    <li>
                        Invite users to manage a project
                    </li>
                    <li>
                        List public projects under your organization profile
                    </li>
                    <li>
                        Set admin permissions for users to edit organization
                        settings
                    </li>
                </ul>
            </div>
        </div>
    </div>`
})

export class OrganizationsPageComponent {
    get orgs() { return List(); }
}