// Copyright:: Copyright (c) 2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

import {AppStore} from "../AppStore";
import {Component} from "angular2/core";
import {RouterLink} from "angular2/router";
import {linkGitHubAccount, unlinkGitHubAccount} from "../actions/index";
import {icon} from "../util";

@Component({
    directives: [RouterLink],
    template: `
    <div class="hab-linked-accounts">
        <h2>Linked Accounts</h2>
        <p>
            You'll need to link a
            <a href="https://github.com" target="_blank">GitHub</a>
            account in order to create projects. Projects must be connected to
            a plan.sh file in your repository.
        </p>
        <form (submit)="linkGitHubAccount(store.getState().user.username)">
            <div class="gh">
                <img class="github-logo"
                     height=64
                     width=64
                     src='{{icon("mark-github")}}'>
            </div>
            <div class="info">
                <div *ngIf="gitHub.isLinked">
                    <p>
                        <strong>{{gitHub.username}}</strong>
                        (<a (click)="unlinkGitHubAccount()" href="#">unlink</a>)
                    <p>
                    <p>read/write</p>
                    <p>
                        <a class="button" [routerLink]='["SCMRepos"]'>
                            View Repos & Add Project
                        </a>
                    </p>
                </div>
                <div *ngIf="!gitHub.isLinked">
                    <p><button>Link GitHub Account</button></p>
                    <p>
                        {{appName}} will be able to read and write to all
                        public and private repositories. We'll only use write
                        access to automatically configure service hooks.
                    </p>
                </div>
            </div>
        </form>
    </div>`,
})

export class LinkedAccountsPageComponent {
    constructor(private store: AppStore) { }

    get appName() { return this.store.getState().appName; }
    get gitHub() { return this.store.getState().gitHub; }

    private icon(x) { return icon(x); }

    private linkGitHubAccount(username) {
        this.store.dispatch(linkGitHubAccount(username));
        return false;
    }

    private unlinkGitHubAccount() {
        this.store.dispatch(unlinkGitHubAccount());
        return false;
    }
}
