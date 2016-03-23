// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import {Component} from "angular2/core";
import {Router} from "angular2/router";
import {SignUpFormComponent} from "../sign-up-form/SignUpFormComponent";
import {AppStore} from "../AppStore";
import {requestRoute} from "../actions/index";

@Component({
    directives: [SignUpFormComponent],
    template: `
    <div class="hab-hero">
        <div class="hab-home">
            <h2>Applications done correctly</h2>
            <h3>Build, deploy, and run your applications well.</h3>
            <h4>For containers, for the cloud, for the data center.</h4>
        </div>
        <sign-up-form [appName]="appName"></sign-up-form>
    </div>`,
})

export class HomePageComponent {
    constructor(private store: AppStore) { }

    ngOnInit() {
        if (this.store.getState().user.isSignedIn) {
            this.store.dispatch(
                requestRoute(["Packages", { filter: "mine" }])
            );
        }
    }

    get appName() { return this.store.getState().appName; }
    get username() { return this.store.getState().user.username; }
}
