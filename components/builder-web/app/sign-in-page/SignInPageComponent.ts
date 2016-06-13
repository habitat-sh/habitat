// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {Component, OnInit} from "angular2/core";
import {RouteParams, RouterLink} from "angular2/router";
import {AppStore} from "../AppStore";
import {setGitHubAuthState, signOut} from "../actions/index";
import config from "../config";
import {createGitHubLoginUrl, icon} from "../util";

@Component({
    directives: [RouterLink],
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
                <a href="https://github.com/join" _target="blank">
                    Create one now
                </a>.
            </p>
            <p>
                You can still browse the
                <a href="{{sourceCodeUrl}}">
                    {{appName}} source code
                </a>,
                <a [routerLink]="['Packages']">
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

export class SignInPageComponent implements OnInit {
    constructor(private routeParams: RouteParams, private store: AppStore) { }

    get appName() { return this.store.getState().app.name; }

    get docsUrl() { return config["docs_url"]; }

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

    private icon(name) { return icon(name); }

    private signOut() {
        this.store.dispatch(signOut());
    }

    public ngOnInit() {
        // Populate the GitHub authstate (used to get a token) in SessionStorage
        // either with what's there already, or with a new UUID.
        this.store.dispatch(setGitHubAuthState());
    }
}
