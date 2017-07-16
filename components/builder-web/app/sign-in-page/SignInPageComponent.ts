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

import { Component, OnInit, OnDestroy } from "@angular/core";
import { AppStore } from "../AppStore";
import { setGitHubAuthState, signOut, setLayout } from "../actions/index";
import config from "../config";
import { createGitHubLoginUrl, icon } from "../util";

@Component({
    template: `
    <div class="hab-sign-in">
        <div class="page-title">
            <h2>Sign In</h2>
        </div>
        <div class="page-body">
            <div class="button-area">
                <hr>
                <a [class.disabled]="isSigningIn || isSignedIn"
                   class="button cta" href="{{gitHubLoginUrl}}">
                    <i class="octicon octicon-mark-github"></i>
                    <span *ngIf="isSigningIn">
                        Signing In&hellip;
                    </span>
                    <span *ngIf="!isSignedIn && !isSigningIn">
                        Sign In with GitHub
                    </span>
                    <span *ngIf="isSignedIn && !isSigningIn">
                        Signed In with GitHub
                    </span>
                </a>
                <a *ngIf="isSignedIn"
                   class="button hab-sign-in--out"
                   (click)="signOut()"
                   href="#">
                   Sign Out
                </a>
                <hr>
            </div>
            <p>
                New to {{appName}}? To sign up, simply click the GitHub button
                above.
            </p>
            <p>
                The {{appName}} project is maintained on GitHub and packages are
                built from plan files stored in GitHub repositories. GitHub
                accounts are free.
                <a href="{{gitHubJoinUrl}}" _target="blank">
                    Create one now
                </a>.
            </p>
            <p>
                You can still browse the
                <a href="{{sourceCodeUrl}}">
                    {{appName}} source code
                </a>,
                <a [routerLink]="['/pkgs']">
                    public packages
                </a>,
                and
                <a href="{{docsUrl}}">
                    documentation
                </a>
                without signing in.
            </p>
        </div>
    </div>`,
})

export class SignInPageComponent implements OnInit, OnDestroy {
    constructor(private store: AppStore) { }

    get appName() { return this.store.getState().app.name; }

    get docsUrl() { return config["docs_url"]; }

    get gitHubJoinUrl() { return `${config["github_web_url"]}/join`; }

    get gitHubLoginUrl() {
        return createGitHubLoginUrl(this.store.getState().gitHub.authState);
    }

    get isSignedIn() {
        return this.store.getState().users.current.isSignedIn;
    }

    get isSigningIn() {
        return this.store.getState().users.current.isSigningIn;
    }

    get sourceCodeUrl() { return config["source_code_url"]; }

    signOut() {
        this.store.dispatch(signOut());
    }

    ngOnInit() {
        // Populate the GitHub authstate (used to get a token) in SessionStorage
        // either with what's there already, or with a new UUID.
        this.store.dispatch(setGitHubAuthState());
        // Don't show the side nav on this screen
        this.store.dispatch(setLayout("centered"));
    }

    ngOnDestroy() {
        this.store.dispatch(setLayout("default"));
    }

    private icon(name) {
        return icon(name);
    }
}
