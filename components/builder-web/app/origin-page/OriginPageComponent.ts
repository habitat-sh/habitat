// Copyright:: Copyright (c) 2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {Component, OnInit} from "angular2/core";
import {RouterLink} from "angular2/router";
import {AppStore} from "../AppStore";
import {setOriginAddingPublicKey, setOriginAddingPrivateKey}
    from "../actions/index";
import config from "../config";
import {KeyAddFormComponent} from "./KeyAddFormComponent";
import {TabComponent} from "../TabComponent";
import {TabsComponent} from "../TabsComponent";

@Component({
    directives: [KeyAddFormComponent, RouterLink, TabsComponent, TabComponent],
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
        <tabs>
            <tab tabTitle="Keys">
                <div class="page-body">
                    <div class="hab-origin--left">
                        <div class="hab-origin--key-list">
                            <p><button
                                (click)="setOriginAddingPublicKey(true)"
                                [disabled]="addingPublicKey">
                                Add public origin key
                            </button></p>
                            <hab-key-add-form
                                *ngIf="addingPublicKey"
                                [docsUrl]="docsUrl"
                                keyFileHeaderPrefix="SIG-PUB-1"
                                [onCloseClick]="onPublicKeyCloseClick"
                                [originName]="origin.name">
                            </hab-key-add-form>
                            <p *ngIf="origin.publicKeys.size === 0">
                                No public origin keys found.
                            </p>
                            <ul *ngIf="origin.publicKeys.size > 0" class="hab-item-list">
                                <li></li>
                            </ul>
                        </div>
                        <div class="hab-origin--key-list">
                            <p><button
                                (click)="setOriginAddingPrivateKey(true)"
                                [disabled]="addingPrivateKey">
                                Add private origin key
                            </button></p>
                            <hab-key-add-form
                                *ngIf="addingPrivateKey"
                                keyFileHeaderPrefix="SIG-SEC-1"
                                [onCloseClick]="onPrivateKeyCloseClick"
                                [originName]="origin.name">
                            </hab-key-add-form>
                            <p *ngIf="origin.privateKeys.size === 0">
                                No private origin keys found.
                            </p>
                            <ul *ngIf="origin.privateKeys.size > 0" class="hab-item-list">
                                <li></li>
                            </ul>
                        </div>
                    </div>
                    <div class="hab-origin--right">
                        <p>
                            <em>Origin keys</em> ensure only authorized users (or
                            organizations) are able to push updates to packages.
                        </p>
                        <p>
                            Read the docs for more information on
                            <a href="{{docsUrl}}/concepts-keys/">
                                managing and using keys</a>.
                        </p>
                    </div>
                </div>
            </tab>
            <tab tabTitle="Members">
                <div class="page-body">
                    <div class="hab-origin--left">
                        <h4>Enter a user's GitHub username</h4>
                        <input type="search">
                    </div>
                    <div class="hab-origin--right">
                        <p>
                            As an origin <em>owner</em>, you can grant admin access,
                            manage packages, and manage keys.
                        </p>
                        <p>
                            <em>Members</em> will be able to push updates to
                            packages that are associated with this origin.
                        </p>
                    </div>
                </div>
            </tab>
            <tab tabTitle="Account">
                <div class="page-body">
                    <div class="hab-origin--left">
                        <button disabled>Delete this origin</button>
                        <p><small>
                            Warning: this operation cannot be undone.
                        </small></p>
                        <p class="hab-origin--deny-delete">
                            This origin cannot be deleted since it has existing
                            packages<br> in the depot.
                        </p>
                    </div>
                    <div class="hab-origin--right">
                        <p>
                            Origins can only be deleted if they do not have any
                            packages in the depot.
                        </p>
                    </div>
                </div>
            </tab>
        </tabs>
    </div>`,
})

export class OriginPageComponent {
    private onPrivateKeyCloseClick: Function;
    private onPublicKeyCloseClick: Function;

    constructor(private store: AppStore) {
        this.onPrivateKeyCloseClick = () =>
            this.setOriginAddingPrivateKey(false);
        this.onPublicKeyCloseClick = () =>
            this.setOriginAddingPublicKey(false);
    }

    get addingPrivateKey() {
        return this.store.getState().origins.ui.current.addingPrivateKey;
    }

    get addingPublicKey() {
        return this.store.getState().origins.ui.current.addingPublicKey;
    }

    get docsUrl() {
        return config["docs_url"];
    }

    get origin() {
        return this.store.getState().origins.current;
    }

    private setOriginAddingPrivateKey(state: boolean) {
        this.store.dispatch(setOriginAddingPrivateKey(state));
        return false;
    }

    private setOriginAddingPublicKey(state: boolean) {
        this.store.dispatch(setOriginAddingPublicKey(state));
        return false;
    }
}