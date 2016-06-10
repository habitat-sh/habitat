// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

import {Component, OnInit} from "angular2/core";
import {RouterLink} from "angular2/router";
import {acceptOriginInvitation, fetchMyOriginInvitations, fetchMyOrigins}
    from "../actions/index";
import {AppStore} from "../AppStore";
import {requireSignIn} from "../util";

@Component({
    directives: [RouterLink],
    template: `
    <div class="hab-origins">
        <div class="page-title">
            <h2>
                Origins
            </h2>
            <a class="button create"
               [routerLink]="['OriginCreate']">Add Origin</a>
        </div>
        <div *ngIf="!ui.loading" class="page-body">
            <p *ngIf="ui.errorMessage">
                Failed to load origins: {{ui.errorMessage}}
            </p>
            <div *ngIf="origins.size === 0">
                <div class="hero">
                    <h3>You don't currently have any origins, let's add one now.</h3>
                    <p>
                        <a class="button cta" [routerLink]='["OriginCreate"]'>
                            Add Origin
                        </a>
                    </p>
                </div>
            </div>
            <div *ngIf="origins.size > 0">
                <ul class="hab-origins-list">
                    <li *ngFor="#origin of origins">
                        <a [routerLink]="['Origin', { origin: origin.name }]"
                           class="hab-item-list">
                            <div class="hab-item-list--title">
                                <h3>{{origin.name}}</h3>
                            </div>
                        </a>
                    </li>
                </ul>
            </div>
            <div *ngIf="invitations.size > 0">
                <h3>Invitations</h3>
                <ul>
                    <li *ngFor="#invitation of invitations" class="hab-item-list hab-no-select">
                       <h3 class="hab-item-list--title">{{invitation.origin_name}}</h3>
                       <button
                           class="count"
                           (click)="acceptInvitation(invitation.id)">
                           Accept Invitation
                        </button>
                    </li>
                </ul>
            </div>
        </div>
    </div>`,
})

export class OriginsPageComponent implements OnInit {
    constructor(private store: AppStore) { }

    get invitations() { return this.store.getState().origins.myInvitations; }

    get origins() { return this.store.getState().origins.mine; }

    get ui() { return this.store.getState().origins.ui.mine; }

    private acceptInvitation(invitationId) {
        this.store.dispatch(acceptOriginInvitation(
            invitationId,
            this.store.getState().gitHub.authToken
        ));
    }

    public ngOnInit() {
        requireSignIn(this);
        this.store.dispatch(fetchMyOrigins(
            this.store.getState().gitHub.authToken
        ));
        this.store.dispatch(fetchMyOriginInvitations(
            this.store.getState().gitHub.authToken
        ));
    }
}
