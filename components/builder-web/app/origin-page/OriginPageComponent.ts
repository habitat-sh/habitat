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

import {Component, OnInit} from "angular2/core";
import {RouteParams, RouterLink} from "angular2/router";
import {AppStore} from "../AppStore";
import {fetchOrigin, fetchOriginInvitations, fetchOriginMembers,
    fetchOriginPublicKeys, inviteUserToOrigin, setCurrentOriginAddingPublicKey,
    setCurrentOriginAddingPrivateKey, uploadOriginPrivateKey,
    uploadOriginPublicKey} from "../actions/index";
import config from "../config";
import {KeyAddFormComponent} from "./KeyAddFormComponent";
import {KeyListComponent} from "./KeyListComponent";
import {Origin} from "../records/Origin";
import {OriginMembersTabComponent} from "./OriginMembersTabComponent";
import {TabComponent} from "../TabComponent";
import {TabsComponent} from "../TabsComponent";
import {requireSignIn} from "../util";

@Component({
    directives: [KeyAddFormComponent, KeyListComponent,
        OriginMembersTabComponent, RouterLink, TabsComponent, TabComponent],
    template: `
    <div class="hab-origin">
        <div class="page-title">
            <a class="button hab-origin--pkgs-link"
                [routerLink]="['PackagesForOrigin', { origin: origin.name }]">
                View <em>{{origin.name}}</em> packages
            </a>
            <h2>{{origin.name}}</h2>
            <h4>Origin</h4>
        </div>
        <div *ngIf="!ui.exists && !ui.loading" class="page-body">
            <p>
                Failed to load origin.
                <span *ngIf="ui.errorMessage">
                    Error: {{ui.errorMessage}}
                </span>
            </p>
        </div>
        <tabs *ngIf="ui.exists && !ui.loading">
            <tab tabTitle="Keys">
                <div class="page-body">
                    <div class="hab-origin--left">
                        <div class="hab-origin--key-list">
                            <h3>Public Origin Keys</h3>
                            <p><button
                                (click)="setOriginAddingPublicKey(true)"
                                [disabled]="addingPublicKey">
                                Upload public origin key
                            </button></p>
                            <hab-key-add-form
                                *ngIf="addingPublicKey"
                                [docsUrl]="docsUrl"
                                [errorMessage]="ui.publicKeyErrorMessage"
                                keyFileHeaderPrefix="SIG-PUB-1"
                                [onCloseClick]="onPublicKeyCloseClick"
                                [originName]="origin.name"
                                [uploadKey]="uploadPublicKey">
                            </hab-key-add-form>
                            <p *ngIf="ui.publicKeyListErrorMessage">
                                Failed to load public keys:
                                {{ui.publicKeyListErrorMessage}}.
                            </p>
                            <hab-key-list
                                *ngIf="!ui.publicKeyListErrorMessage"
                                [keys]="publicKeys"
                                type="public origin">
                            </hab-key-list>
                        </div>
                        <hr>
                        <div class="hab-origin--key-list">
                            <h3>Private Origin Keys</h3>
                            <p><button
                                (click)="setOriginAddingPrivateKey(true)"
                                [disabled]="addingPrivateKey">
                                Upload private origin key
                            </button></p>
                            <hab-key-add-form
                                *ngIf="addingPrivateKey"
                                [errorMessage]="ui.privateKeyErrorMessage"
                                keyFileHeaderPrefix="SIG-SEC-1"
                                [onCloseClick]="onPrivateKeyCloseClick"
                                [originName]="origin.name"
                                [uploadKey]="uploadPrivateKey">
                            </hab-key-add-form>
                            <ul class="bullet">
                                <li>For security purposes, private keys can not be viewed or downloaded.</li>
                                <li>Only one private key exists for an origin at a
                                given time.</li>
                                <li><em>Uploading a new private key will overwrite the
                                existing private key.</em></li>
                            </ul>
                        </div>
                    </div>
                    <div class="hab-origin--right">
                        <p>
                            <em>Origin keys</em> ensure only authorized users (or
                            organizations) are able to push updates to packages
                            in this origin.
                        </p>
                        <p>
                            Read the docs for more information on
                            <a href="{{docsUrl}}/concepts-keys/">
                                managing and using keys</a>.
                        </p>
                    </div>
                </div>
            </tab>
            <hab-origin-members-tab
                [errorMessage]="ui.userInviteErrorMessage"
                [invitations]="invitations"
                [members]="members"
                [onSubmit]="onUserInvitationSubmit">
            </hab-origin-members-tab>
        </tabs>
    </div>`,
})

export class OriginPageComponent implements OnInit {
    private onPrivateKeyCloseClick: Function;
    private onPublicKeyCloseClick: Function;
    private onUserInvitationSubmit: Function;
    private uploadPrivateKey: Function;
    private uploadPublicKey: Function;

    constructor(private routeParams: RouteParams, private store: AppStore) {
        this.onPrivateKeyCloseClick = () =>
            this.setOriginAddingPrivateKey(false);
        this.onPublicKeyCloseClick = () =>
            this.setOriginAddingPublicKey(false);
        this.uploadPrivateKey = key =>
            this.store.dispatch(uploadOriginPrivateKey(key,
                this.gitHubAuthToken));
        this.uploadPublicKey = key =>
            this.store.dispatch(uploadOriginPublicKey(key,
                this.gitHubAuthToken));
        this.onUserInvitationSubmit = username =>
            this.store.dispatch(inviteUserToOrigin(
                username,
                this.origin.name,
                this.gitHubAuthToken
            ));
    }

    get addingPrivateKey() {
        return this.ui.addingPrivateKey;
    }

    get addingPublicKey() {
        return this.ui.addingPublicKey;
    }

    get docsUrl() {
        return config["docs_url"];
    }

    get gitHubAuthToken() {
        return this.store.getState().gitHub.authToken;
    }

    get invitations() {
        return this.store.getState().origins.currentPendingInvitations;
    }

    get members() {
        return this.store.getState().origins.currentMembers;
    }

    get publicKeys() {
        return this.store.getState().origins.currentPublicKeys;
    }

    // Initially set up the origin to be whatever comes from the params,
    // so we can query for it. In `ngOnInit`, we'll
    // populate more data by dispatching `fetchOrigin`.
    get origin() {
        const currentOriginFromState = this.store.getState().origins.current;
        const params = this.routeParams.params;

        // Use the current origin from the state if it's the same origin we want
        // here.
        if (currentOriginFromState.name === params["origin"]) {
            return currentOriginFromState;
        } else {
            return Origin({ name: params["origin"] });
        }
    }

    get ui() {
        return this.store.getState().origins.ui.current;
    }

    private setOriginAddingPrivateKey(state: boolean) {
        this.store.dispatch(setCurrentOriginAddingPrivateKey(state));
        return false;
    }

    private setOriginAddingPublicKey(state: boolean) {
        this.store.dispatch(setCurrentOriginAddingPublicKey(state));
        return false;
    }

    public ngOnInit() {
        requireSignIn(this);
        this.store.dispatch(fetchOrigin(this.origin.name));
        this.store.dispatch(fetchOriginPublicKeys(
            this.origin.name, this.gitHubAuthToken
        ));
        this.store.dispatch(fetchOriginMembers(
            this.origin.name, this.gitHubAuthToken
        ));
        this.store.dispatch(fetchOriginInvitations(
            this.origin.name, this.gitHubAuthToken
        ));
    }
}
