// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {Component} from "angular2/core";
import {RouterLink} from "angular2/router";
import config from "../config";
import {UserNavComponent} from "./user-nav/UserNavComponent";

@Component({
    directives: [RouterLink, UserNavComponent],
    inputs: ["appName", "isUserNavOpen", "isSignedIn", "username", "avatarUrl",
        "signOut", "toggleUserNavMenu"],
    selector: "hab-header",
    template: `
    <header class="hab-header">
        <h1 class="logo">{{appName}}</h1>
        <nav>
            <ul>
                <li><a [routerLink]="['PackagesForOrigin', { origin: 'core' }]">
                    Packages
                </a></li>
                <li><a href="{{config['docs_url']}}">Docs</a></li>
                <li><a href="{{config['tutorials_url']}}">Tutorials</a></li>
                <li><a href="{{config['community_url']}}">Community</a></li>
                <user-nav [isOpen]="isUserNavOpen"
                        [isSignedIn]="isSignedIn"
                        [username]="username"
                        [avatarUrl]="avatarUrl"
                        [signOut]="signOut"
                        [toggleUserNavMenu]="toggleUserNavMenu"></user-nav>
            </ul>
        </nav>
    </header>`,
})

export class HeaderComponent {
    get config() { return config; }
}
